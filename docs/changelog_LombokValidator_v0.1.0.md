# LombokValidator — CHANGELOG

Format mengikuti [Keep a Changelog](https://keepachangelog.com/en/1.1.0/) dan [SemVer](https://semver.org/). Versi ini dirilis lewat `release-please` (ADR-011) — commit `feat:`/`fix:` men-trigger bump otomatis; jangan edit versi manual di sini.

*Entri terbaru selalu di bagian paling depan (PRINSIP_UNIVERSAL U12).*

## [0.1.0] — 2026-09-28

### Added
- Port TypeScript (lulus 1.124 vektor) + uji mutasi
- Regex-lite: `{n}`/`{n,}`/`{n,m}`, anggaran langkah, `PATTERN_TOO_COMPLEX`
- 8 katalog draf terjemahan mesin + uji paritas
- Pseudo-fuzz 30.000 iterasi

### Changed
- Keputusan dependensi: LombokJSON opsional (rencana), LombokLocale wajib
- Dependensi LombokLocale memakai `default-features = false` agar fitur `std` merambat benar

### Fixed
- `is_email` menerima lokal berkutip/berspasi dan mengabaikan batas panjang RFC 5321
- `is_url` menerima host berspasi dan karakter tak aman
- `strip_html` kuadratik pada `<<<<…` (kini linear)

### Security
- Dokumentasi risiko backtracking pada pola tak tepercaya

---

*Untuk riwayat lengkap tiap versi, lihat [GitHub Releases](https://github.com/codinglombok/LombokValidator/releases).*
