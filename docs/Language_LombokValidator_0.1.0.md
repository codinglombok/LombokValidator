# Language — LombokValidator 0.1.0

Per-language port map: role, location, vector consumption, and platform notes. Single contract: `vectors/lombokvalidator-vectors-v1.json` (SHA-256 recorded in the SPEC) — **all ports consume the same file**, no copies.

| Language | Role | Location | Status 0.1.0 | Vectors | Platform notes |
|---|---|---|---|---|---|
| **Rust** | **Reference** implementation — pins the SPEC contract and fills vector outputs (`LOMBOK_REGEN`) | `rust/` | ✅ complete, 35 tests green, clippy `-D warnings` CLEAN, `no_std+alloc` proven | `rust/tests/vectors.rs` (serde_json, dev-dep only) | MSRV 1.75; single runtime dep `lomboklocale` (L0, ADR-009, path-dep — TD-1); robustness pseudo-fuzz with fixed xorshift seed |
| **TypeScript** | **Mandatory port** — the ecosystem's npm publication path | `typescript/` | ✅ complete, 18 unit + 1124 vector tests green, coverage 99.18/98.89 | `typescript/test/vectors.test.ts` (reads the same JSON file) | Node ≥18; dual ESM+CJS; `compat.ts` replicates Rust `str/char` semantics (White_Space, Cc, code point, UTF-8 byte); `bigint` for i64; zero runtime deps |
| Python | Future port | `python/` | 🟩 stub (README only, no manifest) | not yet | priority pending the ecosystem README decision; CI guarded by `hashFiles('python/pyproject.toml')` |
| Go | Future port | `go/` | 🟩 stub (README only) | not yet | table-driven tests when created; guard `hashFiles('go/go.mod')` |
| PHP | Future port | `php/` | 🟩 stub (README only) | not yet | phpstan max + phpunit when created; guard `hashFiles('php/composer.json')` |

## 1. Vector consumption rules

1. Only the reference implementation may FILL `out`/`err`. Ports never write vectors.
2. Existing ports (TS) must pass 100% of cases; a single mismatch is a port bug (not a vector bug, unless a new contract is agreed in the SPEC).
3. Comparisons are bit-identical on the JSON structure; float near-equality is forbidden (see SPEC §4.4).
4. Stubs are not failures: they are fenced in CI so they stay *skipped* (green) until their manifest actually exists (S6/DELTA B) — never delete them.

## 2. Per-language notes (existing ports)

### Rust (reference)
- Layout: `src/{error,string,number,pattern,sanitize,schema}.rs` + `tests/{vectors,robustness,catalogs}.rs` + `examples/smoke.rs`.
- Features: `std` (default) / `no_std+alloc`. The vector test uses `serde_json` as a dev-dependency — never linked into the runtime library (S1-safe).
- Vector harness: one `conformance` test maps `fn` → function calls; `LOMBOK_REGEN=1` rewrites the file (procedure in SPEC §9).

### TypeScript (public port)
- Layout: `src/index.ts` (full API) + `src/compat.ts` (Rust semantics) + `test/{vectors,unit}.test.ts`.
- Deliberate fallback plan: string → `Array.from` (code point); `trim` → `rustTrim` (White_Space property); `is_control/is_whitespace` → explicit Unicode tables; byte-length → `TextEncoder`.
- `package.json`: `exports` map (`import`/`require`/`types`), `files` keeps the package small; `c8` coverage gate via `npm run coverage`.
- Vector comparison: `assert.deepStrictEqual` against the parsed result structure; `bigint → string` for i64 args (SPEC §4.3).

## 3. Procedure for adding a future port

1. Read the full SPEC + the error code table (§3) + the serialization rules (§4).
2. Create the `<lang>/` folder with the language's standard manifest; write a harness that reads `../vectors/lombokvalidator-vectors-v1.json`.
3. Language strict toolchain (S5) + coverage thresholds (S4) + offline tests.
4. Add a CI job guarded by `hashFiles('<lang>/manifest')` — do not touch other jobs.
5. Update this document + `Full Summary Project` + the README table row once the port enters priority.
