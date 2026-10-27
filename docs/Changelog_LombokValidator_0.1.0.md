# Changelog — LombokValidator 0.1.0

Format: [Keep a Changelog](https://keepachangelog.com/en/1.1.0/) + [SemVer 2.0.0](https://semver.org/). This is the library's first public release; unless stated otherwise, all entries below cover both the Rust package (`lombokvalidator` crate) and the npm package (`lombokvalidator`, the TypeScript port).

## [0.1.0] — 2026-10-27

Initial release: the cross-language validation contract is pinned by the Rust reference implementation, the mandatory TypeScript port, and 1124 deterministic test vectors shared across all ports (byte-identical).

### Added

- **String validators**: `is_email` / `validate_email` (structural subset of RFC 5321/5322/6531 — byte limits 254/64/63, SMTPUTF8, IDN A-label), `is_url` / `validate_url` (RFC 3986 subset, rejects whitespace/controls/`<>"\^`{|}`), `is_uuid` / `validate_uuid` (RFC 4122), `validate_length` (Unicode code points, inclusive min/max), `validate_pattern` (via regex-lite).
- **Numeric validators**: `validate_range_i64`, `validate_range_f64` (inclusive; `None`/`null` = unbounded), `validate_positive_i64`, `validate_positive_f64`, `is_integer_str`, `is_float_str`. Full `i64` (TS uses `bigint`; vectors carry decimal strings so precision beyond 2^53 stays safe).
- **regex-lite**: zero-dep pattern engine — literals, `.`, classes + negation + ranges, `\d \w \s`, quantifiers `* + ? {n} {n,} {n,m}` (clamped at 1000), anchors `^ $`, top-level alternation; step budget `MAX_STEPS = 1_000_000` → `PATTERN_TOO_COMPLEX` instead of hanging (anti-ReDoS, fail-closed).
- **Schema**: `Schema` (collects ALL errors, registration order), `nested()` with `parent.child` prefix, `Validate` trait/interface, `FieldValidator` + `ClosureValidator` for custom rules, `merge_results`/`mergeResults`.
- **Sanitization**: `trim` (Rust `White_Space` semantics), `to_lowercase` (full Unicode), `strip_html` linear-time (removes `<…>` ranges; not an HTML sanitizer).
- **i18n-ready error structure**: stable `field` + `code` + catalog `messageId` + typed `args` (`Str/Int/UInt/Float`); the contract is the structure — translation text is free (LombokLocale).
- **Rust**: `no_std + alloc` (optional `std` feature), one controlled runtime dependency: `lomboklocale` (L0, ADR-009); `ArgValue` is owned locally with `From<&ArgValue> for lomboklocale::Arg` conversion.
- **TypeScript**: dual **ESM + CJS** build (`exports` map with `import`/`require`, `dist/cjs` typed `commonjs`), Node ≥ 18, zero runtime dependencies, `sideEffects: false`.
- **Tests**: 1124 shared test vectors (`vectors/lombokvalidator-vectors-v1.json`, SHA-256 `7f526729667f369d874c942476b8186fbc003e11afd908ac2af5d4058c033781`) executed by both languages; deterministic pseudo-fuzz robustness (30k random cases + hostile sizes); consistency tests across the 10 locale catalogs; 18 TS unit tests; smoke example `rust/examples/smoke.rs`.
- **Coverage**: Rust `cargo llvm-cov` 98.6% lines; TypeScript `c8` with gates 95% statements / 90% branches / 95% functions / 95% lines (actual: 99.18 / 98.89 / 95.23 / 99.18).
- **CI**: local `rust` job (clippy `-D warnings`, no_std proof, tests + vectors) and `typescript` job (typecheck, test, coverage) plus `python-port`/`go-port`/`php-port` jobs guarded by `hashFiles` so they stay *skipped* until their port manifests exist (S6); the ecosystem `lombok-ci` job is preserved.
- **Documentation**: DOCS PACK of 13 documents in `docs/` (API, Changelog, SPEC, Architecture, Masterplan, etc.).
- Locale catalogs for 10 languages: `en` (source), `id` (written by the ecosystem owner), 8 machine drafts (`de es fr pt ru ja ko zh-Hans`) — flagged as requiring native-speaker review.

### Changed

- **[Build behavior fix]** `rust/Cargo.toml` previously imported `ArgValue` from `lomboklocale` — that symbol does not exist in LombokLocale 0.2.0 (API drift; the crate could not compile at all before this fix). `ArgValue` is now defined locally as part of the LombokValidator contract; the integration tests in `lib.rs` were adapted to LombokLocale 0.2.0's `resolve(…, Args<'_>)` API.
- **[TS]** `typescript/README.md` rewritten from the stub template ("not yet implemented") to real port documentation (installation, quickstart, vector path).
- **[Repo]** `.gitignore`: the blanket `/docs` rule was replaced with the ecosystem block (6 internal-document patterns) so public documents (API, Changelog, etc.) can be committed while internal documents stay local.
- **[TS]** `package.json`: `main`/`exports` now point at the dual ESM+CJS artifacts; `typecheck`, `coverage`, `build:esm`, `build:cjs` scripts added; 6 never-used `compat.ts` helpers removed (`asciiLower`, `asciiUpper`, `eqIgnoreAsciiCase`, `isAsciiAlpha`, `isAsciiAlnum`, `cmpCodePoints`) before the initial release (never published → not breaking).

### Deprecated

- (nothing)

### Removed

- (nothing — initial release; see Changed for pre-release cleanups)

### Fixed

- **[Build bug]** `rust/examples/smoke.rs` gained end-to-end proof; `strip_html` documents "linear time" — the `E0716` borrow issue was commented on in the example, not in the library.
- **[Docs]** README references to `docs/*_v0.1.0.md` (with `v` prefix) aligned to the DOCS PACK `[Version]` naming without prefix (`*_0.1.0.md`).

### Security

- Untrusted input is never dynamically executed (no `eval`/`exec`); regex patterns run under a step budget so catastrophic backtracking cannot become a DoS (`PATTERN_TOO_COMPLEX`, fail-closed in `matches_pattern`).
- `strip_html` is explicitly flagged as not a sanitizer — preventing misuse on untrusted rich-text (redirect to LombokHTML).
- No secrets in the repo; tests are offline (all deterministic, no env-gating that turns on by default).

[0.1.0]: https://github.com/codinglombok/LombokValidator/releases/tag/v0.1.0
