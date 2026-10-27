# LombokValidator — TypeScript Port (npm: `lombokvalidator`)

Zero-dependency input validation with an i18n-ready error structure. **Byte-identical** with the Rust reference (`../rust/`) — both run the same 1124 test vectors from `../vectors/lombokvalidator-vectors-v1.json`. Behavioral contract: `docs/SPEC_LombokValidator_0.1.0.md` (stable error structure `field`+`code`+`messageId`+`args`; translation text via LombokLocale is outside the contract).

## Status

✅ Port complete and equivalent to the Rust reference at v0.1.0: 1124 vectors pass, coverage 99.18% statements / 98.89% branches (c8 gate ≥95/≥90).

## Install

```bash
npm install lombokvalidator     # v0.1.0 (after release — until then: build from the repo)
```

- Node ≥ 18, **zero runtime dependencies**
- Dual format: **ESM** (`import`) + **CJS** (`require`) via the `exports` map
- `sideEffects: false`, TypeScript types included (`dist/src/*.d.ts`)

## Quick start

```ts
import * as V from "lombokvalidator";

const errs = new V.Schema()
  .field("email", () => V.validateEmail("email", "budi@contoh.co.id"))
  .field("age",   () => V.validateRangeI64("age", 17, 13, 120))
  .validateAll();
// [] = valid

const bad = V.validateEmail("email", "not-an-email");
// [{ field: "email", code: "INVALID_EMAIL", messageId: "validator.string.invalid_email", args: {} }]
```

CJS is also supported: `const V = require("lombokvalidator");`

## Development

```bash
npm ci                 # reproducible install (typescript, @types/node, c8 — dev only)
npm run typecheck      # tsc --strict --noEmit
npm test               # build ESM + run unit + 1124 vectors
npm run coverage       # c8 gate: ≥95% statements / ≥90% branches
npm run build          # dist/src (ESM + .d.ts) + dist/cjs (CommonJS, rootDir src)
```

Build note: `dist/cjs/package.json` (`{"type":"commonjs"}`) is generated automatically by `build:cjs` — it must ship in the package; without it `require` fails because the root package is typed `module`.

## Layout

- `src/index.ts` — the full port API (1:1 mapping to the reference; `isEmail`, `validateRangeI64`, `tryMatchesPattern`, `Schema`, etc.)
- `src/compat.ts` — Rust semantics for JS: Unicode `White_Space`, category `Cc`, code point counting, UTF-8 byte length, `rustTrim`
- `test/vectors.test.ts` — consumes the SAME vector file as Rust (bit-identical, `deepStrictEqual`)
- `test/unit.test.ts` — edge branches & helpers (18 tests)
