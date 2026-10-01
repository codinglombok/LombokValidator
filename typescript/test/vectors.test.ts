// Cross-language conformance: every case in ../vectors/lombokvalidator-vectors-v1.json.
import { test } from "node:test";
import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { dirname, join } from "node:path";
import * as V from "../src/index.js";

const here = dirname(fileURLToPath(import.meta.url));
const doc = JSON.parse(readFileSync(join(here, "../../../vectors/lombokvalidator-vectors-v1.json"), "utf8")) as {
  cases: Array<{ fn: string; in: any; out?: unknown; err?: string }>;
};
const big = (v: unknown): bigint | null => (v === null ? null : BigInt(v as string | number));
// Vector encoding: i64 args are decimal strings, u64/f64 args are JSON numbers.
const enc = (es: V.ValidationResult) =>
  es.length === 0 ? null : es.map((e) => ({ field: e.field, code: e.code, messageId: e.messageId,
    args: Object.fromEntries(Object.entries(e.args).map(([k, v]) => [k, typeof v === "bigint" ? v.toString() : v])) }));

function evalCase(fn: string, i: any): { out?: unknown; err?: string } {
  switch (fn) {
    case "is_email": return { out: V.isEmail(i) };
    case "is_url": return { out: V.isUrl(i) };
    case "is_uuid": return { out: V.isUuid(i) };
    case "is_integer_str": return { out: V.isIntegerStr(i) };
    case "is_float_str": return { out: V.isFloatStr(i) };
    case "strip_html": return { out: V.stripHtml(i) };
    case "trim": return { out: V.trim(i) };
    case "to_lowercase": return { out: V.toLowercase(i) };
    case "validate_length": return { out: enc(V.validateLength("f", i.value, i.min, i.max)) };
    case "validate_range_i64": return { out: enc(V.validateRangeI64("f", big(i.value)!, big(i.min), big(i.max))) };
    case "validate_range_f64": return { out: enc(V.validateRangeF64("f", i.value, i.min, i.max)) };
    case "validate_positive_i64": return { out: enc(V.validatePositiveI64("f", big(i)!)) };
    case "validate_positive_f64": return { out: enc(V.validatePositiveF64("f", i)) };
    case "matches_pattern": { const r = V.tryMatchesPattern(i.value, i.pattern); return r === "TooComplex" ? { err: r } : { out: r }; }
    case "validate_pattern": return { out: enc(V.validatePattern("f", i.value, i.pattern)) };
    default: throw new Error("unknown fn " + fn);
  }
}

test(`conformance: ${doc.cases.length} vectors`, () => {
  const failures: string[] = [];
  doc.cases.forEach((c, n) => {
    const got = evalCase(c.fn, c.in);
    try { assert.deepStrictEqual(got.out, c.out); assert.deepStrictEqual(got.err, c.err); }
    catch { failures.push(`case ${n} ${c.fn} in=${JSON.stringify(c.in).slice(0, 120)} expected=${JSON.stringify(c.out ?? c.err)} got=${JSON.stringify(got.out ?? got.err)}`); }
  });
  assert.equal(failures.length, 0, `${failures.length} failures:\n` + failures.slice(0, 8).join("\n"));
});

test("Schema collects all errors and prefixes nested fields", () => {
  const address = { validate: () => V.validateLength("postcode", "12", 5, 5) };
  const errs = new V.Schema()
    .field("email", () => V.validateEmail("email", "nope"))
    .field("age", () => V.validateRangeI64("age", 5, 13, 120))
    .nested("address", address).validateAll();
  assert.deepEqual(errs.map((e) => e.field), ["email", "age", "address.postcode"]);
  assert.deepEqual(new V.Schema().field("x", () => V.validateEmail("x", "a@b.co")).validateAll(), []);
});
