# LombokValidator

Zero-dependency input validation: strings, numbers, schemas (nested), custom rules and sanitization, with i18n-ready errors.

A standalone, general-purpose library of the **Lombok Ecosystem** — Tier **L1**. Requires [LombokLocale](https://github.com/codinglombok/LombokLocale) (L0) for catalog-backed messages (ADR-009); LombokJSON (L0) is an optional, planned dependency for JSON-Schema validation (`json` feature, not in v0.1.0). The Rust crate currently depends on LombokLocale by path — switch to the registry version before publishing.

> **Universal by design.** Usable by anyone — from small embedded devices to premium industrial software — without any application or framework. It is not part of, and not owned by, any app or server (e.g. RAG stacks); apps are merely example users. 

## Status

🔵 **v0.1.0 — not yet published** to GitHub/registries. Rust reference + **TypeScript port** both pass the same shared test vectors (`vectors/lombokvalidator-vectors-v1.json`, byte-identical behaviour per ADR-015). Python/Go/PHP ports are stubs. 

## Features

- **Strings**: email (RFC 5321 length limits), URL (no unsafe characters), UUID, length (Unicode code points), pattern
- **regex-lite** (own engine): literals, `.`, classes, `\d \w \s`, `* + ? {n} {n,} {n,m}`, `^ $`, `|` — with a **step budget** (`PATTERN_TOO_COMPLEX` instead of hanging)
- **Numbers**: range / positive for `i64` & `f64`, integer/float shape
- **Schema**: collects *all* errors; nested objects (`address.postcode`); trait/closure custom validators
- **Sanitization**: `trim`, `to_lowercase`, linear-time `strip_html` (plain text only — not an HTML sanitizer; use LombokHTML)
- **i18n-ready errors**: stable `code` + `messageId` + typed args, resolved via LombokLocale; 10 catalogs (en, id + 8 draft machine translations)
- `no_std + alloc` core

## Quick Start

### Rust (reference)

```toml
[dependencies]
lombokvalidator = { git = "https://github.com/codinglombok/LombokValidator", package = "lombokvalidator" }   # not on crates.io yet
```

```rust
use lombokvalidator::*;
struct Signup { email: &'static str, age: i64 }
impl Validate for Signup {
    fn validate(&self) -> ValidationResult {
        Schema::new()
            .field("email", || validate_email("email", self.email))
            .field("age", || validate_range_i64("age", self.age, Some(13), Some(120)))
            .validate_all()
    }
}
assert!(Signup { email: "a@b.co", age: 20 }.validate().is_ok());
```

### TypeScript

```bash
cd typescript && npm ci && npm test     # builds ESM + CJS, then runs every shared vector
```

Zero runtime dependencies, dual **ESM + CJS** (`exports` map), Node ≥ 18. Coverage gate: `npm run coverage` (≥95% statements / ≥90% branches). See `docs/API_LombokValidator_0.1.0.md` for the camelCase API.

## Testing

```bash
cd rust && cargo test --release                       # unit + robustness (pseudo-fuzz) + shared vectors
cd rust && cargo build --no-default-features          # no_std + alloc proof
cd typescript && npm test                             # same vectors, TypeScript port
cd typescript && npm run coverage                     # c8 gate: ≥95% stmt / ≥90% branch
LOMBOK_REGEN=1 cargo test --release --test vectors    # regenerate expected outputs from the Rust reference (review the diff!)
./scripts/lombok-doctor-docs.sh LombokValidator .     # 13 docs, versions, vector hash, license, no ownership claims
```

Vector inputs are authored in `vectors/gen_inputs.py` (deterministic); expected outputs come from the Rust reference and are reviewed by hand and, where possible, by independent checks. Changing any vector requires updating its SHA-256 in `docs/SPEC_LombokValidator_0.1.0.md` (doctor script enforces it).



## License

Apache 2.0 — see [LICENSE](LICENSE). (MASTERPLAN §10.1 suggests `Apache-2.0 OR MIT`; pending owner decision.)
