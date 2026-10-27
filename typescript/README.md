# LombokValidator — TypeScript Port (npm: `lombokvalidator`)

Zero-dependency input validation with i18n-ready error structure. **Byte-identical** with the Rust reference (`../rust/`) — both run the same 1124 test vectors from `../vectors/lombokvalidator-vectors-v1.json`. Kontrak perilaku: `docs/SPEC_LombokValidator_0.1.0.md` (struktur error `field`+`code`+`messageId`+`args` stabil; teks terjemahan via LombokLocale di luar kontrak).

## Status

✅ Port lengkap dan setara acuan Rust pada v0.1.0: 1124 vector lulus, coverage 99.18% statement / 98.89% branch (gate c8 ≥95/≥90).

## Install

```bash
npm install lombokvalidator     # v0.1.0 (setelah rilis — sampai saat ini: build dari repo)
```

- Node ≥ 18, **zero runtime dependencies**
- Dual format: **ESM** (`import`) + **CJS** (`require`) via `exports` map
- `sideEffects: false`, TypeScript types included (`dist/src/*.d.ts`)

## Quick start

```ts
import * as V from "lombokvalidator";

const errs = new V.Schema()
  .field("email", () => V.validateEmail("email", "budi@contoh.co.id"))
  .field("age",   () => V.validateRangeI64("age", 17, 13, 120))
  .validateAll();
// [] = valid

const bad = V.validateEmail("email", "bukan-email");
// [{ field: "email", code: "INVALID_EMAIL", messageId: "validator.string.invalid_email", args: {} }]
```

CJS juga didukung: `const V = require("lombokvalidator");`

## Development

```bash
npm ci                 # reproducible install (typescript, @types/node, c8 — dev only)
npm run typecheck      # tsc --strict --noEmit
npm test               # build ESM + jalankan unit + 1124 vector
npm run coverage       # c8 gate: ≥95% statements / ≥90% branches
npm run build          # dist/src (ESM + .d.ts) + dist/cjs (CommonJS, rootDir src)
```

Catatan build: `dist/cjs/package.json` (`{"type":"commonjs"}`) dibuat otomatis oleh `build:cjs` — wajib ikut dalam paket, tanpa itu `require` gagal karena root package bertipe `module`.

## Layout

- `src/index.ts` — seluruh API port (mapping 1:1 ke acuan; `isEmail`, `validateRangeI64`, `tryMatchesPattern`, `Schema`, dst.)
- `src/compat.ts` — semantik Rust untuk JS: Unicode `White_Space`, kategori `Cc`, hitungan code point, panjang byte UTF-8, `rustTrim`
- `test/vectors.test.ts` — konsumsi file vector yang SAMA dengan Rust (bit-identical, `deepStrictEqual`)
- `test/unit.test.ts` — edge branch & helper (18 test)
