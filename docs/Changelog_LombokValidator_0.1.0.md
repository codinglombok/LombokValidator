# Changelog — LombokValidator 0.1.0

Format: [Keep a Changelog](https://keepachangelog.com/id-ID/1.1.0/) + [SemVer 2.0.0](https://semver.org/lang/id/). Versi pertama publik pustaka ini; semua catatan berikut mencakup paket Rust (`lombokvalidator` crate) dan paket npm (`lombokvalidator`, port TypeScript) secara bersama kecuali disebut lain.

## [0.1.0] — 2026-10-27

Rilis perdana: kontrak validasi lintas bahasa dipatok di acuan Rust, port TypeScript wajib, dan 1124 test vector deterministik yang dibagikan ke semua port (byte-identical).

### Added

- **Validator string**: `is_email` / `validate_email` (subset struktural RFC 5321/5322/6531 — limit byte 254/64/63, SMTPUTF8, IDN A-label), `is_url` / `validate_url` (subset RFC 3986, tolak whitespace/kontrol/`<>"\^`{|}`), `is_uuid` / `validate_uuid` (RFC 4122), `validate_length` (code point Unicode, min/max inklusif), `validate_pattern` (via regex-lite).
- **Validator numerik**: `validate_range_i64`, `validate_range_f64` (inklusif, `None`/`null` = tanpa batas), `validate_positive_i64`, `validate_positive_f64`, `is_integer_str`, `is_float_str`. `i64` penuh (TS memakai `bigint`; vector mengirim desimal string agar presisi >2^53 aman).
- **regex-lite**: mesin pola zero-dep — literal, `.`, kelas + negasi + rentang, `\d \w \s`, kuantifier `* + ? {n} {n,} {n,m}` (clamp 1000), anchor `^ $`, alternasi top-level; anggaran langkah `MAX_STEPS = 1_000_000` → `PATTERN_TOO_COMPLEX` alih-alih hang (anti-ReDoS, fail-closed).
- **Skema**: `Schema` (kumpulkan SEMUA error, urutan registrasi), `nested()` dengan prefix `parent.child`, trait/interface `Validate`, `FieldValidator` + `ClosureValidator` untuk rule kustom, `merge_results`/`mergeResults`.
- **Sanitasi**: `trim` (semantik Rust `White_Space`), `to_lowercase` (Unicode penuh), `strip_html` linear-time (hapus rentang `<…>`, bukan sanitizer HTML).
- **Struktur error i18n-ready**: `field` + `code` stabil + `messageId` katalog + `args` ter-tipe (`Str/Int/UInt/Float`); kontrak = struktur, teks terjemahan bebas (LombokLocale).
- **Rust**: `no_std + alloc` (fitur `std` opsional), satu runtime dependency terkontrol: `lomboklocale` (L0, ADR-009); `ArgValue` dimiliki lokal dengan konversi `From<&ArgValue> for lomboklocale::Arg`.
- **TypeScript**: dual build **ESM + CJS** (`exports` map dengan `import`/`require`, `dist/cjs` bertipe `commonjs`), Node ≥ 18, zero runtime dependency, `sideEffects: false`.
- **Test**: 1124 test vector bersama (`vectors/lombokvalidator-vectors-v1.json`, SHA-256 `7f526729667f369d874c942476b8186fbc003e11afd908ac2af5d4058c033781`) dijalankan oleh kedua bahasa; robustness pseudo-fuzz deterministik (30k kasus acak + hostile sizes); uji konsistensi 10 katalog locale; unit TS 18 test; contoh smoke `rust/examples/smoke.rs`.
- **Coverage**: Rust `cargo llvm-cov` 98.6% lines; TypeScript `c8` dengan gate 95% statement / 90% branch / 95% function / 95% line ( aktual: 99.18 / 98.89 / 95.23 / 99.18).
- **CI**: job lokal `rust` (clippy `-D warnings`, bukti no_std, test + vector) dan `typescript` (typecheck, test, coverage) plus job `python-port`/`go-port`/`php-port` ber-guard `hashFiles` agar *skipped* sampai manifest portnya ada (S6); job ekosistem `lombok-ci` dipertahankan.
- **Dokumentasi**: DOCS PACK 13 dokumen di `docs/` (API, Changelog, SPEC, Architecture, Masterplan, dst.).
- Katalog locale 10 bahasa: `en` (sumber), `id` (ditulis pemilik), 8 draf mesin (`de es fr pt ru ja ko zh-Hans`) — ditandai wajib review penutur asli.

### Changed

- **[Fix-perilaku build]** `rust/Cargo.toml` semula mengimpor `ArgValue` dari `lomboklocale` — simbol tersebut tidak ada di LombokLocale 0.2.0 (API drift; crate tidak bisa dikompilasi sama sekali sebelum perbaikan ini). Kini `ArgValue` didefinisikan lokal sebagai bagian kontrak LombokValidator; test integrasi di `lib.rs` menyesuaikan diri ke API `resolve(…, Args<'_>)` LombokLocale 0.2.0.
- **[TS]** `typescript/README.md` diperbarui dari template stub ("not yet implemented") menjadi dokumentasi port aktual (instalasi, quickstart, jalur vector).
- **[Repo]** `.gitignore`: aturan blanket `/docs` diganti blok ekosistem (6 pola dokumen internal) sehingga dokumen publik (API, Changelog, dst.) dapat di-commit sementara dokumen internal tetap lokal.
- **[TS]** `package.json`: `main`/`exports` mengarah ke artefak dual ESM+CJS; skrip `typecheck`, `coverage`, `build:esm`, `build:cjs` ditambahkan; 6 helper `compat.ts` yang tidak pernah dipakai dihapus (`asciiLower`, `asciiUpper`, `eqIgnoreAsciiCase`, `isAsciiAlpha`, `isAsciiAlnum`, `cmpCodePoints`) sebelum rilis perdana (belum pernah dipublikasi → bukan breaking).

### Deprecated

- (tidak ada)

### Removed

- (tidak ada — rilis perdana; lihat Changed untuk pembersihan pre-release)

### Fixed

- **[Bug build]** `rust/examples/smoke.rs` menambah bukti end-to-end; `strip_html` dokumen menyebut "linear time" — komentari `let` binding borrow (`E0716`) pada contoh, bukan pada pustaka.
- **[Dok]** Referensi README ke `docs/*_v0.1.0.md` (prefiks `v`) diselaraskan ke penamaan DOCS PACK `[Versi]` tanpa prefiks (`*_0.1.0.md`).

### Security

- Input tidak tepercaya tidak pernah dieksekusi dinamis (tanpa `eval`/`exec`); pattern regex dibatasi anggaran langkah sehingga backtracking katastrofik tidak bisa menjadi DoS (`PATTERN_TOO_COMPLEX`, fail-closed pada `matches_pattern`).
- `strip_html` ditandai eksplisit bukan sanitizer — mencegah salah pakai pada untrusted rich-text (arahkan ke LombokHTML).
- Tanpa secret di repo; test tanpa jaringan (semua deterministik, tanpa env-gating yang menyala default).

[0.1.0]: https://github.com/codinglombok/LombokValidator/releases/tag/v0.1.0
