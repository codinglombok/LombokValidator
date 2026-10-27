# API — LombokValidator 0.1.0

Complete reference of all public exports. The behavioral contract (byte-identical across languages) is defined in `SPEC_LombokValidator_0.1.0.md`; this document records signatures and per-language usage. Naming convention: Rust `snake_case`, TypeScript `camelCase` — a 1:1 mapping in each function's table. All Rust examples assume `use lombokvalidator::*;` and TS assumes `import * as V from "lombokvalidator"`.

Base types (identical in both languages):

| Concept | Rust | TypeScript |
|---|---|---|
| One error | `ValidationError { field: String, code: &'static str, messageId: &'static str, args: Vec<(String, ArgValue)> }` | `ValidationError { field: string, code: string, messageId: string, args: Record<string, ArgValue> }` |
| Result | `ValidationResult = Result<(), Vec<ValidationError>>` | `ValidationResult = ValidationError[]` (empty array = valid) |
| Message args | `ArgValue::Str(String) \| Int(i64) \| UInt(u64) \| Float(f64)` | `ArgValue = string \| number \| bigint` |
| Regex budget failure | `Result<bool, PatternError>` with `PatternError::TooComplex` | `boolean \| "TooComplex"` |

`ArgValue` is a contract type owned by LombokValidator (owned by this repo, not LombokLocale), so locale API changes can never shift the contract; conversion to `lomboklocale::Arg` happens only when rendering catalog messages.

## 1. Strings — predicates

| Function | Rust signature | TypeScript signature |
|---|---|---|
| `is_email` | `fn is_email(s: &str) -> bool` | `isEmail(s: string): boolean` |
| `is_url` | `fn is_url(s: &str) -> bool` | `isUrl(s: string): boolean` |
| `is_uuid` | `fn is_uuid(s: &str) -> bool` | `isUuid(s: string): boolean` |

All predicates are pure, allocation-free except small internal buffers, O(n). Detailed semantics (RFC 5321 limits, RFC 3986 subset, RFC 4122 format) live in SPEC §5.1–5.3. Examples:

```rust
assert!(is_email("user@example.co.id"));
assert!(!is_url("https://exa mple.com"));   // space = Rust whitespace
assert!(is_uuid("550e8400-e29b-41d4-a716-446655440000"));
```

```ts
assert.equal(V.isEmail("user@example.co.id"), true);
assert.equal(V.isUrl("https://exa mple.com"), false);
assert.equal(V.isUuid("550e8400-e29b-41d4-a716-446655440000"), true);
```

## 2. Strings — validation (structured errors)

| Function | Rust signature | TypeScript signature |
|---|---|---|
| `validate_email` | `fn validate_email(field: &str, value: &str) -> ValidationResult` | `validateEmail(field: string, value: string): ValidationResult` |
| `validate_url` | `fn validate_url(field: &str, value: &str) -> ValidationResult` | `validateUrl(field, value)` |
| `validate_uuid` | `fn validate_uuid(field: &str, value: &str) -> ValidationResult` | `validateUuid(field, value)` |
| `validate_length` | `fn validate_length(field: &str, value: &str, min: Option<usize>, max: Option<usize>) -> ValidationResult` | `validateLength(field: string, value: string, min?: number \| null, max?: number \| null): ValidationResult` |
| `validate_pattern` | `fn validate_pattern(field: &str, value: &str, pattern: &str) -> ValidationResult` | `validatePattern(field, value, pattern): ValidationResult` |

Possible errors: `INVALID_EMAIL`, `INVALID_URL`, `INVALID_UUID`, `TOO_SHORT` (args `min`, `actual`), `TOO_LONG` (args `max`, `actual`), `PATTERN_MISMATCH`, `PATTERN_TOO_COMPLEX`. Length counts Unicode code points. Example with i18n (Rust):

```rust
let catalog = lomboklocale::parse_catalog(&std::fs::read_to_string("locales/id/lombokvalidator.json")?)?;
let err = validate_length("username", "ab", Some(4), None).unwrap_err();
let e = &err[0]; // TOO_SHORT, args: min=4, actual=2
let mut args: Vec<(&str, lomboklocale::Arg)> =
    e.args.iter().map(|(k, v)| (k.as_str(), lomboklocale::Arg::from(v))).collect();
args.push(("field", lomboklocale::Arg::Str(e.field.clone())));
let msg = lomboklocale::resolve("id", &catalog, e.code, e.message_id, &args);
assert_eq!(msg.text, "username minimal 4 karakter (saat ini 2)");
```

TypeScript provides the `errorArgs(e)` helper which adds `field` to the args so calling `lomboklocale.resolve` is a one-liner.

## 3. Numbers

| Function | Rust signature | TypeScript signature |
|---|---|---|
| `validate_range_i64` | `(field, value: i64, min: Option<i64>, max: Option<i64>) -> ValidationResult` | `(field, value: bigint \| number, min?: bigint \| number \| null, max?: bigint \| number \| null)` |
| `validate_range_f64` | `(field, value: f64, min: Option<f64>, max: Option<f64>) -> ValidationResult` | `(field, value: number, min?: number \| null, max?: number \| null)` |
| `validate_positive_i64` | `(field, value: i64) -> ValidationResult` | `(field, value: bigint \| number)` |
| `validate_positive_f64` | `(field, value: f64) -> ValidationResult` | `(field, value: number)` |
| `is_integer_str` | `fn is_integer_str(s: &str) -> bool` | `isIntegerStr(s: string): boolean` |
| `is_float_str` | `fn is_float_str(s: &str) -> bool` | `isFloatStr(s: string): boolean` |

Errors: `BELOW_MIN` (args `min`, `actual`), `ABOVE_MAX` (args `max`, `actual`), `NOT_POSITIVE` (arg `actual`). i64 uses `bigint` in TS for full precision; `number` is accepted for safe values. The `i64::MIN/MAX` bounds are safe to use.

## 4. regex-lite (pattern engine)

| Function | Rust signature | TypeScript signature |
|---|---|---|
| `try_matches_pattern` | `fn try_matches_pattern(input: &str, pattern: &str) -> Result<bool, PatternError>` | `tryMatchesPattern(input: string, pattern: string): boolean \| "TooComplex"` |
| `matches_pattern` | `fn matches_pattern(input: &str, pattern: &str) -> bool` (fail-closed: `false` on TooComplex) | `matchesPattern(input, pattern): boolean` |
| Constant | `pub const MAX_STEPS: u64 = 1_000_000` | `MAX_STEPS = 1_000_000` |

Features & exclusions, step-budget rules, and the treatment of malformed patterns are normative in SPEC §5.5. Matching is full-string; use explicit `^`/`$` when anchoring is needed (it is full-string; anchors matter inside alternation).

## 5. Sanitization

| Function | Rust signature | TypeScript signature |
|---|---|---|
| `trim` | `fn trim(s: &str) -> &str` | `trim(s: string): string` |
| `to_lowercase` | `fn to_lowercase(s: &str) -> String` | `toLowercase(s: string): string` |
| `strip_html` | `fn strip_html(s: &str) -> String` | `stripHtml(s: string): string` |

`trim` uses the Unicode `White_Space` property (Rust semantics — NBSP and U+3000 are trimmed too). `strip_html` removes complete `<…>` ranges, linear time, no entity decoding; it is not an HTML sanitizer (see `Guide How to Use` §Troubleshooting).

## 6. Schema & custom validators

| Export | Rust signature | TypeScript signature |
|---|---|---|
| trait `Validate` | `fn validate(&self) -> ValidationResult` | `interface Validate { validate(): ValidationResult }` |
| `Schema` | `Schema::new().field(name, closure).nested(name, &child).validate_all()` | `new Schema().field(name, fn).nested(name, child).validateAll()` |
| trait `FieldValidator<T>` | `fn check(&self, field: &str, value: &T) -> ValidationResult` | `interface FieldValidator<T> { check(field, value): ValidationResult }` |
| `ClosureValidator` | `ClosureValidator(closure)` implements `FieldValidator<T>` | `new ClosureValidator<T>((field, value) => ValidationResult)` |
| `merge_results` | `fn merge_results(results: Vec<ValidationResult>) -> ValidationResult` | `mergeResults(rs: ValidationResult[]): ValidationResult` |
| args helper | — (build `Vec<(&str, Arg)>` manually) | `errorArgs(e: ValidationError): Record<string, ArgValue>` |

`Schema` runs ALL rules then merges errors (not fail-fast) in registration order; `nested()` prefixes every child error's `field` with `"<name>."`. Full examples live in `rust/examples/smoke.rs` and `Guide How to Use` §Quickstart.

## 7. Error codes [CODE]

The stable code list + `messageId` + args: see `SPEC_LombokValidator_0.1.0.md` §3 (normative table — not duplicated here so it cannot diverge). Application-defined custom codes are free, as long as the four-key structure is preserved.

## 8. Conditional compilation (Rust only)

- `default = ["std"]`; `--no-default-features` produces the `no_std + alloc` build (proof: the CI `rust` job).
- `dev-dependencies` (`serde_json`) exist for the vector tests only — never linked into the runtime library. S1: the single runtime dependency is `lomboklocale` (L0, allowed by ADR-009).
