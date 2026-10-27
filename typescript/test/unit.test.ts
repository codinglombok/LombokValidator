// Unit tests for API surface and edge branches not exercised by the shared
// vectors (which stay the single cross-language conformance source).
import { test } from "node:test";
import assert from "node:assert/strict";
import * as V from "../src/index.js";
import {
  chars, isRustControl, isRustWhitespace, rustTrim, utf8Len,
} from "../src/compat.js";

// ── compat: Rust Unicode semantics ──────────────────────────────────────────

test("isRustWhitespace covers Unicode White_Space boundaries", () => {
  const ws = [0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x20, 0x85, 0xa0, 0x1680,
    0x2000, 0x2005, 0x200a, 0x2028, 0x2029, 0x202f, 0x205f, 0x3000];
  for (const n of ws) assert.ok(isRustWhitespace(String.fromCodePoint(n)), `U+${n.toString(16)} is whitespace`);
  const notWs = [0x08, 0x0e, 0x21, 0x86, 0xa1, 0x1681, 0x1fff, 0x200b, 0x2027,
    0x202a, 0x2030, 0x2060, 0x3001, 0xfeff];
  for (const n of notWs) assert.ok(!isRustWhitespace(String.fromCodePoint(n)), `U+${n.toString(16)} is not whitespace`);
});

test("isRustControl covers Cc category boundaries", () => {
  for (const n of [0x00, 0x09, 0x1f]) assert.ok(isRustControl(String.fromCodePoint(n)));
  for (const n of [0x20, 0x7e]) assert.ok(!isRustControl(String.fromCodePoint(n)));
  for (const n of [0x7f, 0x80, 0x9f]) assert.ok(isRustControl(String.fromCodePoint(n)));
  assert.ok(!isRustControl("\u00a0"));
  assert.ok(!isRustControl("a"));
});

test("rustTrim strips Rust whitespace on both sides only", () => {
  assert.equal(rustTrim("\u{00a0}x\u{3000}"), "x");
  assert.equal(rustTrim("\t\nx\r\n"), "x");
  assert.equal(rustTrim("  "), "");
  assert.equal(rustTrim("abc"), "abc");
  assert.equal(rustTrim(""), "");
});

test("utf8Len counts UTF-8 bytes like Rust str::len", () => {
  assert.equal(utf8Len(""), 0);
  assert.equal(utf8Len("abc"), 3);
  assert.equal(utf8Len("é"), 2);
  assert.equal(utf8Len("😀"), 4);
  assert.equal(utf8Len("日本語"), 9);
});

test("chars splits by code point, not UTF-16 unit", () => {
  assert.deepEqual(chars("a😀b"), ["a", "😀", "b"]);
  assert.equal(chars("é\u0301").length, 2);
});

// ── error helpers ────────────────────────────────────────────────────────────

test("errorArgs appends field for lomboklocale.resolve", () => {
  const errs = V.validateLength("user", "ab", 4, null);
  const e = errs[0]!;
  assert.deepEqual(V.errorArgs(e), { min: 4, actual: 2, field: "user" });
});

test("mergeResults flattens and keeps order", () => {
  const a = V.validateEmail("a", "bad");
  const b = V.validateRangeI64("b", -1, 0, 10);
  const merged = V.mergeResults([a, b, []]);
  assert.deepEqual(merged.map((e) => e.field), ["a", "b"]);
  assert.deepEqual(V.mergeResults([[], []]), []);
});

// ── fail-closed pattern matching ────────────────────────────────────────────

test("matchesPattern fail-closes on TooComplex", () => {
  const input = "a".repeat(3000) + "b";
  assert.equal(V.tryMatchesPattern(input, "a*a*a*a*a*a*c"), "TooComplex");
  assert.equal(V.matchesPattern(input, "a*a*a*a*a*a*c"), false);
  assert.equal(V.MAX_STEPS, 1_000_000);
});

test("pattern tokenizer edge branches", () => {
  assert.equal(V.tryMatchesPattern("]", "[]]"), true); // first ] literal
  assert.equal(V.tryMatchesPattern("a-b", "[a\\-b]+"), true);
  assert.equal(V.tryMatchesPattern("-", "[a-]"), true); // trailing '-' literal
  assert.equal(V.tryMatchesPattern("a{", "a{"), true); // invalid counted → literal
  assert.equal(V.tryMatchesPattern("a{1", "a{1"), true);
  assert.equal(V.tryMatchesPattern("x", "{"), false); // literal '{' ≠ 'x'
  assert.equal(V.tryMatchesPattern("", "|"), true); // empty alternate matches empty input
  assert.equal(V.tryMatchesPattern("a", "|a"), true);
  assert.equal(V.tryMatchesPattern("a", "a|"), true); // first alternate already matches full string
  assert.equal(V.tryMatchesPattern("$", "\\$"), true);
  assert.equal(V.tryMatchesPattern("", ""), true);
});

// ── validator branch edges ──────────────────────────────────────────────────

test("isEmail branch matrix", () => {
  assert.ok(V.isEmail("a@b.co"));
  assert.ok(!V.isEmail("a".repeat(65) + "@b.co")); // local > 64
  assert.ok(!V.isEmail("a@" + "b".repeat(64) + ".com")); // label > 63
  assert.ok(!V.isEmail("a@b.co.")); // label empty (trailing dot)
  assert.ok(!V.isEmail("a@-b.co") && !V.isEmail("a@b-.co"));
  assert.ok(!V.isEmail("a@b.c0m")); // TLD has digit
  assert.ok(!V.isEmail("a@b.c")); // TLD < 2
  assert.ok(!V.isEmail("\u0001a@b.co")); // control in local
  assert.ok(!V.isEmail("a@b.co ".slice(0, 7) + " ")); // trailing space
  assert.ok(V.isEmail("ünï@example.com"));
});

test("isUrl authority branches", () => {
  assert.ok(V.isUrl("https://user:pw@host:8080/p?q=1#f"));
  assert.ok(!V.isUrl("https://:8080"));
  assert.ok(!V.isUrl("https://user@"));
  assert.ok(!V.isUrl("1http://x"));
  assert.ok(!V.isUrl("-http://x"));
  assert.ok(V.isUrl("a+b-c.d://host")); // valid scheme letters/digits/+-. — locked by shared vectors
  assert.ok(!V.isUrl("https://exa\u00a0mple.com")); // NBSP is Rust whitespace
  assert.ok(!V.isUrl("https://a\tb.com"));
  assert.ok(!V.isUrl("ht^tp://x"));
});

test("isUuid group-count and hex branches", () => {
  assert.ok(V.isUuid("550E8400-E29B-41D4-A716-446655440000"));
  assert.ok(!V.isUuid("550e8400-e29b-41d4-a716-44665544000")); // short last group
  assert.ok(!V.isUuid("550e8400e29b41d4a716446655440000"));
});

test("number shape and boundary branches", () => {
  assert.ok(!V.isIntegerStr("+5") && !V.isIntegerStr("1e5") && !V.isIntegerStr("-"));
  assert.ok(V.isIntegerStr("-0"));
  assert.ok(!V.isFloatStr("-.") && V.isFloatStr("5.") && !V.isFloatStr("--1")); // trailing dot accepted by reference — locked by vectors
  assert.ok(V.isFloatStr("-0.5"));
  assert.deepEqual(V.validateLength("f", "abc", null, null), []);
  assert.equal(V.validateRangeI64("f", 5n, null, null).length, 0);
  assert.equal(V.validateRangeF64("f", 5, null, null).length, 0);
});

test("stripHtml branch edges", () => {
  assert.equal(V.stripHtml("a<b"), "a<b"); // unclosed tag kept
  assert.equal(V.stripHtml("<<<<"), "<<<<");
  assert.equal(V.stripHtml("<<<>"), ""); // first < opens a tag (a later > exists), all consumed
  assert.equal(V.stripHtml(""), "");
  assert.equal(V.stripHtml("x</b>y"), "xy");
  assert.equal(V.stripHtml("a > b"), "a > b");
});

test("toLowercase matches Rust str::to_lowercase", () => {
  assert.equal(V.toLowercase("İstanbul"), "i̇stanbul"); // İ → i + U+0307
  assert.equal(V.toLowercase("ǅ"), "ǆ");
  assert.equal(V.toLowercase("straße"), "straße");
  assert.equal(V.toLowercase(""), "");
});

test("Schema and custom validators", () => {
  const even = new V.ClosureValidator<number>((f, v) =>
    v % 2 === 0 ? [] : [{ field: f, code: "NOT_EVEN", messageId: "validator.custom.not_even", args: {} }]);
  assert.deepEqual(even.check("n", 2), []);
  assert.equal(even.check("n", 3).length, 1);

  const child = { validate: () => V.validateLength("postcode", "1", 5, 5) };
  const errs = new V.Schema()
    .field("email", () => V.validateEmail("email", "a@b.co"))
    .nested("addr", child)
    .validateAll();
  assert.deepEqual(errs.map((e) => e.field), ["addr.postcode"]);
  assert.deepEqual(new V.Schema().validateAll(), []); // empty schema passes
});
