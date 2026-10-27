# Full Summary Project — LombokValidator 0.1.0

Executive summary for consumers, reviewers, and the ecosystem owner.

## 1. Purpose

LombokValidator provides zero-dependency input validation (strings, numbers, nested schemas, custom rules, sanitization) with a stable, i18n-ready error structure, as the Lombok ecosystem's single standard validation layer. The cross-language contract is pinned by the Rust reference implementation and verified through 1124 deterministic test vectors re-executed by every port (byte-identical). The contract is the error structure — `field`, stable `code`, `messageId`, typed `args` — not translation text; locale catalogs may change without touching the contract.

## 2. Architecture at a glance

- **Rust (reference)**: `rust/src` — pure predicates (email/URL/UUID/numbers), budgeted regex-lite (`MAX_STEPS`, anti-ReDoS, fail-closed), collect-all schema with nested prefixes, sanitization (Rust White_Space trim, Unicode lowercase, linear strip_html). `no_std + alloc` supported; one controlled runtime dependency: LombokLocale (L0, ADR-009; path-dep — TD-1).
- **TypeScript (mandatory npm port)**: `typescript/src` — 1:1 mapping, `compat.ts` replicates Rust semantics (`White_Space`, `Cc`, code points, UTF-8 bytes), `bigint` for i64, dual ESM+CJS build, zero runtime dependencies.
- **Vectors**: `vectors/gen_inputs.py` (fixed seed) generates inputs; the reference fills outputs (`LOMBOK_REGEN`); both languages consume the same file; bit-identical comparison; the SHA-256 hash is recorded in the SPEC and bound into the doctor script.
- **CI**: ecosystem org job + local rust gates (clippy `-D warnings`, no_std proof, tests) and typescript (typecheck, test, coverage gate); python/go/php are fenced by `hashFiles` so they stay *skipped* until their manifests exist.

## 3. Status vs SPEC (as of 0.1.0)

| Area | Status |
|---|---|
| Error structure contract (SPEC §2–3) | ✅ full — 10 stable codes implemented in 2 languages |
| Byte-precise vector format (§4) | ✅ full — single file, hash recorded & bound into the doctor |
| Semantics of all 15 functions (§5) | ✅ full — vector-locked |
| Edge cases §6 | ✅ vector-locked + extra TS units |
| TS port (mandatory) | ✅ complete, coverage 99.18/98.89 |
| python/go/php ports | 🟩 stubs by design, fenced in CI |
| LombokLocale catalog rendering | ✅ reference integration tests pass against the 0.2.0 API (after the ADR-006 fix) |

## 4. Metrics (measured during the 2026-10-27 engineering session)

| Metric | Value |
|---|---|
| Rust tests | 35 passing (31 unit + 1 catalogs + 2 robustness + 1 vector) |
| TypeScript tests | 18 passing (unit + conformance over 1124 vectors) |
| Shared vectors | 1124 cases; SHA-256 `7f526729667f369d874c942476b8186fbc003e11afd908ac2af5d4058c033781` |
| Rust coverage | 98.38% region / 98.60% lines (llvm-cov) |
| TS coverage | 99.18% stmt / 98.89% branch / 95.23% funcs (c8, gate 95/90/95/95) |
| Toolchain | clippy `-D warnings` CLEAN; `tsc --strict` CLEAN; CI YAML validated |
| no_std | `cargo build --no-default-features` succeeds |
| Smoke | `cargo run --example smoke` OK; npm package ESM+CJS OK from the tarball |
| Runtime dependencies | Rust: lomboklocale (L0); TS: 0 |
| TS package size | ~25 KB JS (dist) |

## 5. Key changes in this session (summary)

1. **API drift fix** (critical): `ArgValue` is now owned locally (ADR-006) — previously the crate failed to compile against LombokLocale 0.2.0.
2. **Dual ESM+CJS** for the npm port (STEP 1) + tarball smoke.
3. **c8 coverage gate** (S4) + 18 TS unit tests + the Rust smoke example.
4. **Local CI** rust/ts + hashFiles guards for stub ports (S6) — org jobs untouched.
5. **Docs pack of 13 documents** + root & TS READMEs updated + doctor script aligned to the new naming.
6. `.gitignore`: blanket `/docs` → ecosystem block (public documents get committed).

## 6. Roadmap (short — details in Masterplan)

0.2.0 distribution (TD-1 registry dep, TD-2 peer sync, TD-4 pin CI ref) → 0.3.0 third port → 0.4.0 native-speaker catalog review → 0.5.0 derive macro → 0.6.0 `json` feature → **1.0.0** frozen contract + 3 stable ports + gated coverage on all ports.

## 7. Quick start

```bash
# Rust
cd rust && cargo run --release --example smoke
# TypeScript
cd typescript && npm ci && npm test && npm run coverage
# Document doctor
./scripts/lombok-doctor-docs.sh LombokValidator .
```

References: SPEC (contract), API (signatures), Guide (usage), Architecture (why), TECH DEBT (debts), Language (port map).
