// LombokValidator — TypeScript port (zero dependencies). Behaviour is defined by
// SPEC_LombokValidator and verified against ../vectors/lombokvalidator-vectors-v1.json.
// Errors carry `code` (stable contract) + `messageId` (catalog key) + typed `args`;
// resolve them with `lomboklocale.resolve(locale, catalog, e.code, e.messageId, {...errorArgs(e)})`.

import { chars, isRustControl, isRustWhitespace, rustTrim, utf8Len } from "./compat.js";

export type ArgValue = string | number | bigint;
export interface ValidationError { field: string; code: string; messageId: string; args: Record<string, ArgValue> }
/** Empty array = valid. */
export type ValidationResult = ValidationError[];

const fail = (field: string, code: string, messageId: string, args: Record<string, ArgValue> = {}): ValidationResult => [{ field, code, messageId, args }];
/** Args plus `field`, ready for `lomboklocale.resolve`. */
export const errorArgs = (e: ValidationError): Record<string, ArgValue> => ({ ...e.args, field: e.field });
export const mergeResults = (rs: ValidationResult[]): ValidationResult => rs.flat();

// ───────────────────────────── regex-lite ─────────────────────────────

export const MAX_STEPS = 1_000_000;
const MAX_REPEAT = 1000;
export type PatternError = "TooComplex";

type Tok =
  | { k: "lit"; c: string } | { k: "any" } | { k: "cls"; negate: boolean; ranges: Array<[number, number]> }
  | { k: "rep"; inner: Tok; min: number; max: number | null } | { k: "start" } | { k: "end" };
const cp = (c: string): number => c.codePointAt(0)!;

function parseClass(cs: string[], st: { i: number }): Tok {
  st.i++;
  const negate = cs[st.i] === "^";
  if (negate) st.i++;
  const ranges: Array<[number, number]> = [];
  let first = true;
  while (st.i < cs.length) {
    let c = cs[st.i]!;
    if (c === "]" && !first) { st.i++; break; }
    first = false;
    if (c === "\\") { st.i++; c = cs[st.i] ?? "\\"; }
    const e = cs[st.i + 2];
    if (cs[st.i + 1] === "-" && e !== undefined && e !== "]") { ranges.push([cp(c), cp(e)]); st.i += 3; }
    else { ranges.push([cp(c), cp(c)]); st.i++; }
  }
  return { k: "cls", negate, ranges };
}
function parseCounted(cs: string[], st: { i: number }): [number, number | null] | null {
  let j = st.i + 1;
  const num = (): number | null => {
    const s = j;
    while (j < cs.length && /^[0-9]$/.test(cs[j]!)) j++;
    if (j === s || j - s > 4) return null;
    return Number(cs.slice(s, j).join(""));
  };
  const min = num();
  if (min === null) return null;
  let max: number | null;
  if (cs[j] === ",") {
    j++;
    if (cs[j] === "}") max = null;
    else { const m = num(); if (m === null) return null; max = m; }
  } else max = min;
  if (cs[j] !== "}") return null;
  if (max !== null && max < min) return null;
  st.i = j + 1;
  return [Math.min(min, MAX_REPEAT), max === null ? null : Math.min(max, MAX_REPEAT)];
}
const CLS_D: Tok = { k: "cls", negate: false, ranges: [[48, 57]] };
function tokenize(pattern: string): Tok[] {
  const cs = chars(pattern), toks: Tok[] = [], st = { i: 0 };
  while (st.i < cs.length) {
    let base: Tok;
    const ch = cs[st.i]!;
    if (ch === "^") { toks.push({ k: "start" }); st.i++; continue; }
    if (ch === "$") { toks.push({ k: "end" }); st.i++; continue; }
    if (ch === ".") { st.i++; base = { k: "any" }; }
    else if (ch === "[") base = parseClass(cs, st);
    else if (ch === "\\") {
      st.i++;
      const esc = cs[st.i] ?? "\\";
      st.i++;
      if (esc === "d") base = CLS_D;
      else if (esc === "w") base = { k: "cls", negate: false, ranges: [[97, 122], [65, 90], [48, 57], [95, 95]] };
      else if (esc === "s") base = { k: "cls", negate: false, ranges: [[32, 32], [9, 9], [10, 10], [13, 13]] };
      else base = { k: "lit", c: esc };
    } else { st.i++; base = { k: "lit", c: ch }; }
    const q = cs[st.i];
    if (q === "*") { st.i++; toks.push({ k: "rep", inner: base, min: 0, max: null }); }
    else if (q === "+") { st.i++; toks.push({ k: "rep", inner: base, min: 1, max: null }); }
    else if (q === "?") { st.i++; toks.push({ k: "rep", inner: base, min: 0, max: 1 }); }
    else if (q === "{") {
      const r = parseCounted(cs, st);
      if (r) toks.push({ k: "rep", inner: base, min: r[0], max: r[1] });
      else toks.push(base);
    } else toks.push(base);
  }
  return toks;
}
function single(t: Tok, c: string): boolean {
  if (t.k === "lit") return t.c === c;
  if (t.k === "any") return true;
  if (t.k === "cls") { const n = cp(c); return t.ranges.some(([lo, hi]) => n >= lo && n <= hi) !== t.negate; }
  return false;
}
class TooComplex extends Error {}
function matchHere(toks: Tok[], ti: number, input: string[], ci: number, steps: { n: number }): boolean {
  if (++steps.n > MAX_STEPS) throw new TooComplex();
  if (ti === toks.length) return ci === input.length;
  const t = toks[ti]!;
  switch (t.k) {
    case "start": return ci === 0 ? matchHere(toks, ti + 1, input, ci, steps) : false;
    case "end": return ci === input.length ? matchHere(toks, ti + 1, input, ci, steps) : false;
    case "rep": {
      const limit = t.max ?? Infinity;
      let run = 0;
      while (run < limit && ci + run < input.length && single(t.inner, input[ci + run]!)) run++;
      if (run < t.min) return false;
      for (let n = run; n >= t.min; n--) if (matchHere(toks, ti + 1, input, ci + n, steps)) return true;
      return false;
    }
    default:
      return ci < input.length && single(t, input[ci]!) ? matchHere(toks, ti + 1, input, ci + 1, steps) : false;
  }
}
/** Full-string match. Returns `"TooComplex"` if the step budget is exhausted. */
export function tryMatchesPattern(input: string, pattern: string): boolean | PatternError {
  const inp = chars(input), steps = { n: 0 };
  try {
    for (const alt of pattern.split("|")) if (matchHere(tokenize(alt), 0, inp, 0, steps)) return true;
    return false;
  } catch (e) { if (e instanceof TooComplex) return "TooComplex"; throw e; }
}
/** Fail-closed variant: `false` when the budget is exhausted. */
export const matchesPattern = (input: string, pattern: string): boolean => tryMatchesPattern(input, pattern) === true;

// ───────────────────────────── string validators ─────────────────────────────

const isDigit = (c: string): boolean => c >= "0" && c <= "9";
const isAlnumA = (c: string): boolean => /^[A-Za-z0-9]$/.test(c);
const ATEXT = "!#$%&'*+/=?^_`{|}~-.";

export function isEmail(s: string): boolean {
  if (utf8Len(s) > 254) return false;
  const at = s.indexOf("@");
  if (at === -1) return false;
  const local = s.slice(0, at), domain = s.slice(at + 1);
  if (local === "" || utf8Len(local) > 64 || domain === "") return false;
  for (const c of chars(local)) {
    const ascii = cp(c) < 128;
    if (!(isAlnumA(c) || ATEXT.includes(c) || (!ascii && !isRustWhitespace(c) && !isRustControl(c)))) return false;
  }
  if (local.startsWith(".") || local.endsWith(".") || local.includes("..")) return false;
  if (!domain.includes(".")) return false;
  const labels = domain.split(".");
  for (const l of labels) {
    if (l === "" || utf8Len(l) > 63 || l.startsWith("-") || l.endsWith("-") || !chars(l).every((c) => isAlnumA(c) || c === "-")) return false;
  }
  const tld = labels[labels.length - 1]!;
  return utf8Len(tld) >= 2 && /^[A-Za-z]+$/.test(tld);
}
const URL_BAD = "<>\"\\^`{|}";
export function isUrl(s: string): boolean {
  for (const c of chars(s)) if (isRustWhitespace(c) || isRustControl(c) || URL_BAD.includes(c)) return false;
  const k = s.indexOf("://");
  if (k === -1) return false;
  const scheme = s.slice(0, k), rest = s.slice(k + 3);
  if (scheme === "" || !/^[A-Za-z]/.test(scheme) || !/^[A-Za-z0-9+.\-]+$/.test(scheme)) return false;
  const end = rest.search(/[/?#]/);
  const authority = end === -1 ? rest : rest.slice(0, end);
  const host = authority.slice(authority.lastIndexOf("@") + 1);
  return host !== "" && !host.startsWith(":");
}
export function isUuid(s: string): boolean {
  const g = s.split("-"), lens = [8, 4, 4, 4, 12];
  return g.length === 5 && g.every((p, i) => p.length === lens[i] && /^[0-9a-fA-F]+$/.test(p));
}
export const isIntegerStr = (s: string): boolean => { const t = s.startsWith("-") ? s.slice(1) : s; return t !== "" && chars(t).every(isDigit); };
export function isFloatStr(s: string): boolean {
  const t = s.startsWith("-") ? s.slice(1) : s;
  if (t === "") return false;
  let dot = false, digit = false;
  for (const c of t) {
    if (c === ".") { if (dot) return false; dot = true; } else if (isDigit(c)) digit = true; else return false;
  }
  return digit;
}

export function validateLength(field: string, value: string, min: number | null = null, max: number | null = null): ValidationResult {
  const len = chars(value).length;
  if (min !== null && len < min) return fail(field, "TOO_SHORT", "validator.string.too_short", { min, actual: len });
  if (max !== null && len > max) return fail(field, "TOO_LONG", "validator.string.too_long", { max, actual: len });
  return [];
}
export const validateEmail = (field: string, v: string): ValidationResult => (isEmail(v) ? [] : fail(field, "INVALID_EMAIL", "validator.string.invalid_email"));
export const validateUrl = (field: string, v: string): ValidationResult => (isUrl(v) ? [] : fail(field, "INVALID_URL", "validator.string.invalid_url"));
export const validateUuid = (field: string, v: string): ValidationResult => (isUuid(v) ? [] : fail(field, "INVALID_UUID", "validator.string.invalid_uuid"));
export function validatePattern(field: string, value: string, pattern: string): ValidationResult {
  const r = tryMatchesPattern(value, pattern);
  if (r === true) return [];
  if (r === false) return fail(field, "PATTERN_MISMATCH", "validator.string.pattern_mismatch");
  return fail(field, "PATTERN_TOO_COMPLEX", "validator.string.pattern_too_complex");
}

// ───────────────────────────── number validators ─────────────────────────────

/** i64 arguments are `bigint` (Rust `i64`); `number` is accepted for safe integers. */
export function validateRangeI64(field: string, value: bigint | number, min: bigint | number | null = null, max: bigint | number | null = null): ValidationResult {
  const v = BigInt(value);
  if (min !== null && v < BigInt(min)) return fail(field, "BELOW_MIN", "validator.number.below_min", { min: BigInt(min), actual: v });
  if (max !== null && v > BigInt(max)) return fail(field, "ABOVE_MAX", "validator.number.above_max", { max: BigInt(max), actual: v });
  return [];
}
export function validateRangeF64(field: string, value: number, min: number | null = null, max: number | null = null): ValidationResult {
  if (min !== null && value < min) return fail(field, "BELOW_MIN", "validator.number.below_min", { min, actual: value });
  if (max !== null && value > max) return fail(field, "ABOVE_MAX", "validator.number.above_max", { max, actual: value });
  return [];
}
export const validatePositiveI64 = (field: string, value: bigint | number): ValidationResult =>
  BigInt(value) > 0n ? [] : fail(field, "NOT_POSITIVE", "validator.number.not_positive", { actual: BigInt(value) });
export const validatePositiveF64 = (field: string, value: number): ValidationResult =>
  value > 0 ? [] : fail(field, "NOT_POSITIVE", "validator.number.not_positive", { actual: value });

// ───────────────────────────── sanitization ─────────────────────────────

export const trim = rustTrim;
export const toLowercase = (s: string): string => s.toLowerCase();
/** Removes `<…>` spans (only if a later `>` exists). Plain-text hygiene, NOT an HTML sanitizer. Linear time. */
export function stripHtml(s: string): string {
  const cs = chars(s);
  const lastGt = cs.lastIndexOf(">");
  let out = "", inTag = false;
  cs.forEach((c, i) => {
    if (!inTag && c === "<") { if (lastGt > i) inTag = true; else out += c; }
    else if (inTag && c === ">") inTag = false;
    else if (!inTag) out += c;
  });
  return out;
}

// ───────────────────────────── schema / custom rules ─────────────────────────────

export interface Validate { validate(): ValidationResult }
export interface FieldValidator<T> { check(field: string, value: T): ValidationResult }
export class ClosureValidator<T> implements FieldValidator<T> {
  constructor(private fn: (field: string, value: T) => ValidationResult) {}
  check(field: string, value: T): ValidationResult { return this.fn(field, value); }
}
/** Struct-level validator: runs every rule and collects ALL errors (not fail-fast). */
export class Schema {
  private rules: Array<() => ValidationResult> = [];
  field(_name: string, check: () => ValidationResult): this { this.rules.push(check); return this; }
  /** Validate a child object; its error fields are prefixed with `<name>.`. */
  nested(name: string, child: Validate): this {
    this.rules.push(() => child.validate().map((e) => ({ ...e, field: `${name}.${e.field}` })));
    return this;
  }
  validateAll(): ValidationResult { return mergeResults(this.rules.map((r) => r())); }
}
