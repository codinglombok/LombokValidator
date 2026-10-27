# Development Idea — LombokValidator 0.1.0

Prioritized idea backlog (P0 = done/in progress, P1 = next, P2/P3 = later) with Definition of Done and non-goals. New ideas enter here first, then get promoted to Masterplan/Milestones once approved.

## P0 — Foundation (done in 0.1.0)

### ID-01 Cross-language vector contract ✅
Deterministic generator + 1124 cases + Rust & TS harnesses. **DoD**: both languages pass 100%; hash recorded in the SPEC; doctor script validates documents. (Achieved.)

### ID-02 Anti-ReDoS by construction ✅
Step budget 1,000,000 + fail-closed. **DoD**: the `a*a*a*a*a*a*c` case on 3000×`a` yields `PATTERN_TOO_COMPLEX`, not a hang; pseudo-fuzz does not panic. (Achieved.)

### ID-03 Dual ESM+CJS build ✅
Complete `exports` map + package smoke. **DoD**: both `import` and `require` pass a real example from the tarball. (Achieved.)

## P1 — Next

### ID-04 Registry dependency (TD-1)
Replace the `lomboklocale` path-dep → crates.io. **DoD**: `cargo build` without a sibling clone; CI without a sibling checkout; clean publish dry-run.

### ID-05 Third port (pick python OR go per the ecosystem README priority)
**DoD**: passes 100% of the same vector file; ruff/mypy strict or go vet CLEAN; coverage ≥95/90; the hashFiles-guarded CI job activates (no longer skipped); the Language document updated.

### ID-06 Vector expansion from the field
Collect real mis-validation cases (issues) → turn them into vectors first (red on wrong ports), then fix. **DoD**: every bug fix ships ≥1 new vector case + SPEC hash bump + a Changelog entry.

### ID-07 Rust coverage gate in CI (TD-7)
**DoD**: the rust job runs `cargo llvm-cov --fail-under-lines 95` green.

## P2 — Mid-term

### ID-08 Catalog review by native speakers (TD-3)
**DoD**: at least 5 languages marked "reviewed" in locales/STATUS.md with reviewer name/date.

### ID-09 Direct message option (non-catalog)
Optional `with_message_text` API for applications without LombokLocale. **DoD**: default stays structure-only; vectors unchanged; feature off = byte-identical.

### ID-10 Common schema preset collection
Presets `signup`, `login`, `address` (composition of existing validators only, no new logic). **DoD**: presets are pure factory functions; all vector tests keep passing; docs cookbook updated.

### ID-11 Deterministic benchmark
Criterion (Rust) + Node benchmarks for the pattern engine & is_email. **DoD**: numbers stored with seed/timestamp; a >20% regression flags a review (informational, not a gate).

## P3 — Long-term

### ID-12 `json` feature (TD-8) — JSON-Schema subset via LombokJSON
**DoD**: feature flag off stays no_std; new contract via additional vectors; docs + Masterplan 0.6.0.

### ID-13 Derive macro (TD-9)
**DoD**: separate crate; derive output identical to the manual Schema in parallel tests.

### ID-14 WASM port
`wasm32` target for browser/edge use with identical results. **DoD**: the vector suite passes on wasm; the SIMD path (if any) is identical to scalar (DELTA B).

### ID-15 Additional Core-20 catalogs (ar/hi/bn/tr/vi/th/it/pl/nl/uk) + Nusantara languages (jv/su/ms)
**DoD**: identical keys/placeholders (automated tests) + native-speaker review; `ar` needs the RTL/BiDi decision in LombokLocale first.

## Non-goals (rejected for focus & contract safety)

1. **Full HTML sanitizer** — not this library's territory (ADR-008; redirect to LombokHTML).
2. **Full RFC email/URL parser** (quoted-local, IP-literals, comment folding) — the structural subset is contract-locked; expansion only via a new contract (MAJOR/MINOR per SPEC §10).
3. **Full regex** (groups, lookaround, backreferences, lazy) — breaks cross-language determinism & the security budget (ADR-004).
4. **Stateful/timing cross-field validation** (e.g. time-based password confirmation) — belongs to the application; the library stays pure and side-effect free.
5. **Async I/O / network calls** — will never exist; all functions are pure.
6. **Built-in telemetry/logging** — will never exist (U10).
