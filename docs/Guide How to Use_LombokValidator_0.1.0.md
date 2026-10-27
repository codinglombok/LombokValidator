# Guide How to Use — LombokValidator 0.1.0

End-user guide: installation, the 5-minute quickstart, a cookbook of common recipes, and troubleshooting.

## 1. Installation

### TypeScript / JavaScript (npm)

```bash
npm install lombokvalidator          # v0.1.0 (E404 until released — see How To Dist)
```

Requirements: Node ≥ 18. Zero runtime dependencies. Dual format: ESM (`import`) and CJS (`require`) — any bundler and older Node are served.

### Rust

```toml
[dependencies]
lombokvalidator = { git = "https://github.com/codinglombok/LombokValidator", package = "lombokvalidator" }
# once on crates.io:
# lombokvalidator = "0.1"
```

Build note: the crate depends on `lomboklocale` via the path `../../LombokLocale/rust` — to build from git, place a `LombokLocale` clone as a sibling repo (CI does the same; see TECH DEBT TD-1).

## 2. 5-minute quickstart (TypeScript)

```ts
import * as V from "lombokvalidator";

// 1) Validate a single field
const emailErrs = V.validateEmail("email", "budi@contoh.co.id"); // [] = valid
const ageErrs   = V.validateRangeI64("age", 17, 13, 120);        // []

// 2) Schema: collect ALL errors at once, nesting supported
const address = { validate: () => V.validateLength("postcode", "12", 5, 5) };
const errs = new V.Schema()
  .field("email", () => V.validateEmail("email", "not-an-email"))
  .field("age",   () => V.validateRangeI64("age", 5, 13, 120))
  .nested("address", address)
  .validateAll();

console.log(errs.map(e => `${e.field}: ${e.code}`));
// [ 'email: INVALID_EMAIL', 'age: BELOW_MIN', 'address.postcode: TOO_SHORT' ]

// 3) Localized messages via LombokLocale (the error structure stays the contract)
//    resolve(locale, catalog, e.code, e.messageId, V.errorArgs(e))
```

## 3. 5-minute quickstart (Rust)

```rust
use lombokvalidator::*;

struct Signup { email: String, age: i64 }

impl Validate for Signup {
    fn validate(&self) -> ValidationResult {
        Schema::new()
            .field("email", || validate_email("email", &self.email))
            .field("age",   || validate_range_i64("age", self.age, Some(13), Some(120)))
            .validate_all()
    }
}

fn main() {
    let s = Signup { email: "budi@contoh.co.id".into(), age: 17 };
    assert!(s.validate().is_ok());
}
```

Run the real example from the repo: `cargo run --release --example smoke` (a full smoke test with nesting + sanitization + patterns).

## 4. Cookbook

### 4.1 Complete signup form (collect all errors for display)

Recommended pattern: one `Schema` per struct, one closure per field, display `field` + `code` in the UI, render text via catalogs. Never match message text — match `code` (stable). Error order follows field registration order, deterministically.

### 4.2 UUID ID validation + input normalization

```ts
const id = V.trim(rawId).toLowerCase();         // sanitize first
const ok = V.isUuid(id) && V.validateUuid("id", id).length === 0;
```

`to_lowercase`/`trim` use Rust semantics — safe for data compared across screens/services of the ecosystem's languages.

### 4.3 Custom patterns (office codes, etc.)

```ts
V.validatePattern("code", input, "[A-Z]{2}-\\d{4}")   // AB-1234
```

If a pattern comes from user input, it is still safe: the engine has a step budget (`PATTERN_TOO_COMPLEX`), it will not hang. Still, prefer patterns from code, not from users.

### 4.4 Custom rules across types

```ts
const even = new V.ClosureValidator<number>((f, v) =>
  v % 2 === 0 ? [] : [{ field: f, code: "NOT_EVEN",
                        messageId: "validator.custom.not_even", args: {} }]);
```

The four-key structure (`field`, `code`, `messageId`, `args`) must be preserved so catalogs and the contract stay consistent.

### 4.5 Cleaning input before storage

```ts
const name = V.stripHtml(V.trim(raw));   // "  <b>Wayan</b> " → "Wayan"
```

`strip_html` removes tags structurally (linear time). For user rich-text that will be rendered as HTML, this is NOT a sanitizer replacement (see Troubleshooting).

### 4.6 Big numbers / i64 limits

TS: use `bigint` for values beyond ±2^53 (e.g. database IDs): `V.validateRangeI64("n", 9007199254740993n, ...)`. Rust: native `i64`. Vectors carry i64 as decimal strings so cross-language consumption loses no precision.

### 4.7 Full i18n integration (Rust)

See `docs/API_LombokValidator_0.1.0.md` §2 — build `Vec<(&str, Arg)>` from `e.args` then call `lomboklocale::resolve(...)`. `default_message()` is available as a hardcoded English fallback for scripts/quick tests, not for production.

## 5. Troubleshooting

| Symptom | Cause | Fix |
|---|---|---|
| `cargo build` fails: `no ArgValue in the root` on older versions | repo version before the ADR-006 fix | pull ≥ the fix commit; `ArgValue` is now a local export |
| `cargo build` fails: `lomboklocale` not found | missing sibling path-dep | clone `codinglombok/LombokLocale` into this repo's PARENT folder |
| Valid email rejected (e.g. `"quoted"@x.com`, `a@[127.0.0.1]`) | structural subset (ADR-003) | by design; watch the SPEC for expansion in a minor version |
| `is_url("file:///etc/passwd")` = false | empty host is rejected | by design, vector-locked |
| `tryMatchesPattern` returns `"TooComplex"` | the 1,000,000-step budget ran out | simplify the pattern; treat it like an error, not `false` |
| `strip_html("5 < 10")` unchanged | a `<` without a following `>` is left alone | by design (fail-safe); do not use for untrusted rich-text |
| `validate_length("日本語")` = 3, not 9 | count per code point (ADR-005) | use `utf8Len` (compat) when you need bytes |
| TS vs Rust results differ? | must not happen — file a bug | run `npm test` (1124 vectors) + `cargo test --release`; compare the case in `vectors/` |
| npm 404 on `npm install lombokvalidator` | not yet published (E404) | follow `How To Dist` for the release, or install from git/tarball |

## 6. Compatibility

- Node ≥ 18 (ESM & CJS), TypeScript ≥ 5.6 for types (`.d.ts` ships in the package).
- Rust ≥ 1.75 (`rust-version`), `no_std + alloc` fully supported (the default `std` feature is only a pass-through to LombokLocale).
- All functions are pure and side-effect free; safe for WebWorkers/edge runtimes as long as `TextEncoder` is available (a standard Node 18/browser API).
