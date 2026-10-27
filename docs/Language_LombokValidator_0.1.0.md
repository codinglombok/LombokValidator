# Language — LombokValidator 0.1.0

Peta port per bahasa: peran, lokasi, konsumsi vector, dan catatan platform. Kontrak tunggal: `vectors/lombokvalidator-vectors-v1.json` (SHA-256 tercatat di SPEC) — **semua port mengonsumsi file yang sama**, tidak ada salinan.

| Bahasa | Peran | Lokasi | Status 0.1.0 | Vector | Catatan platform |
|---|---|---|---|---|---|
| **Rust** | **Acuan** (reference implementation) — mematok kontrak SPEC + mengisi output vector (`LOMBOK_REGEN`) | `rust/` | ✅ lengkap, 35 test hijau, clippy `-D warnings` CLEAN, `no_std+alloc` terbukti | `rust/tests/vectors.rs` (serde_json, dev-dep saja) | MSRV 1.75; satu runtime dep `lomboklocale` (L0, ADR-009, path-dep — TD-1); robustness pseudo-fuzz xorshift seed tetap |
| **TypeScript** | **Port wajib** — jalur publikasi npm ekosistem | `typescript/` | ✅ lengkap, 18 unit + 1124 vector hijau, coverage 99.18/98.89 | `typescript/test/vectors.test.ts` (baca file JSON yang sama) | Node ≥18; dual ESM+CJS; `compat.ts` mereplikasi semantik Rust `str/char` (White_Space, Cc, code point, UTF-8 byte); `bigint` untuk i64; zero runtime dep |
| Python | Port masa depan | `python/` | 🟩 stub (README saja, tanpa manifest) | belum | prioritas menunggu keputusan README ekosistem; CI ber-guard `hashFiles('python/pyproject.toml')` |
| Go | Port masa depan | `go/` | 🟩 stub (README saja) | belum | table-driven tests saat dibuat; guard `hashFiles('go/go.mod')` |
| PHP | Port masa depan | `php/` | 🟩 stub (README saja) | belum | phpstan max + phpunit saat dibuat; guard `hashFiles('php/composer.json')` |

## 1. Peran dan hukum konsumsi vector

1. Hanya acuan yang boleh MENGISI `out`/`err`. Port tidak pernah menulis vector.
2. Port yang sudah ada (TS) diwajibkan lulus 100% kasus; satu saja mismatch = bug port (bukan bug vector, kecuali disepakati kontrak baru di SPEC).
3. Perbandingan bit-identical pada struktur JSON; larangan near-equality float (lihat SPEC §4.4).
4. Stub bukan kegagalan: dipagari di CI agar *skipped* (hijau) sampai manifestnya benar-benar ada (S6/DELTA B) — jangan dihapus.

## 2. Catatan per bahasa (yang sudah ada)

### Rust (acuan)
- Layout: `src/{error,string,number,pattern,sanitize,schema}.rs` + `tests/{vectors,robustness,catalogs}.rs` + `examples/smoke.rs`.
- Fitur: `std` (default) / `no_std+alloc`. Test vector memakai `serde_json` sebagai dev-dependency — tidak pernah ter-link ke pustaka runtime (S1 aman).
- Harness vector: 1 test `conformance` memetakan `fn` → panggilan fungsi; `LOMBOK_REGEN=1` menulis ulang file (prosedur SPEC §9).

### TypeScript (port publik)
- Layout: `src/index.ts` (seluruh API) + `src/compat.ts` (semantik Rust) + `test/{vectors,unit}.test.ts`.
- Rencana fallback khusus: string → `Array.from` (code point); `trim` → `rustTrim` (White_Space property); `is_control/is_whitespace` → tabel Unicode eksplisit; byte-length → `TextEncoder`.
- `package.json`: `exports` map (`import`/`require`/`types`), `files` menjaga ukuran paket; `c8` gate coverage di `npm run coverage`.
- Pembandingan vector: `assert.deepStrictEqual` terhadap struktur hasil parse; `bigint → string` untuk args i64 (SPEC §4.3).

## 3. Prosedur menambah port baru (masa depan)

1. Baca SPEC penuh + tabel kode error (§3) + aturan serialisasi (§4).
2. Buat folder `<lang>/` dengan manifest standar bahasa; tulis harness yang membaca `../vectors/lombokvalidator-vectors-v1.json`.
3. Toolchain ketat bahasa (S5) + threshold coverage (S4) + test tanpa jaringan.
4. Tambah job CI ber-guard `hashFiles('<lang>/manifest')` — jangan menyentuh job lain.
5. Perbarui dokumen ini + `Full Summary Project` + baris tabel README bila port masuk prioritas.
