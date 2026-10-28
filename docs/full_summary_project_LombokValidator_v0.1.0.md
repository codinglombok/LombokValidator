# LombokValidator — FULL SUMMARY PROJECT v0.1.0

Ringkasan satu-halaman untuk pembaca baru (reviewer, calon kontributor, aplikasi yang mengevaluasi dependensi) — tidak perlu membaca 11 dokumen lain untuk memahami apa, mengapa, dan status proyek ini.

## Apa Ini?

Validasi input zero-dependency: string (email/URL/UUID/panjang/pola), angka (rentang/positif), skema struct + objek bersarang, validator kustom berbasis trait, dan sanitasi — dengan error yang membawa `code` stabil + `messageId` yang diterjemahkan lewat LombokLocale.

## Mengapa Dibuat (bukan pakai library pihak ketiga)?

`validator`/`garde` menarik `regex`, `url`, `syn` dan tidak menjamin hasil identik lintas bahasa; Validator memberi kontrak lintas-port dan pesan i18n bawaan tanpa dependensi.

## Fitur Utama

*Library ini mandiri dan universal (PRINSIP_UNIVERSAL U1); tidak menjadi bagian dari aplikasi/framework mana pun.*

- email (batas RFC 5321: total 254, lokal 64, label 63 byte; karakter lokal `atext` + non-ASCII SMTPUTF8; domain LDH/punycode), URL (tanpa spasi/kontrol/`<>"\\^`{|}`), UUID
- panjang (dalam karakter Unicode), rentang & positif `i64`/`f64`
- regex-lite: literal, `.`, kelas, `\\d \\w \\s`, `* + ? {n} {n,} {n,m}`, `^ $`, `|`; **anggaran langkah 1.000.000** → `PATTERN_TOO_COMPLEX` (tidak hang)
- `Schema` mengumpulkan semua error; `.nested()` mengawali nama field dengan `induk.`; validator kustom trait/closure
- Sanitasi trim/lowercase/strip_html (linear)
- Error: `code` + `message_id` + args bertipe; 10 katalog (en, id + 8 draf mesin)

## Status Saat Ini

| Aspek | Nilai |
|---|---|
| Status | 🔵 BUILDING — inti Rust + port TypeScript lulus vektor bersama; belum dipublikasikan ke GitHub/registry |
| Versi | 0.1.0 |
| Bahasa referensi | Rust |
| Port tersedia | TypeScript ✅ (lulus seluruh vektor); Python, Go, PHP — folder stub, belum ada kode |
| Test | 35 test Rust (unit + robustness 30.000 iterasi + paritas katalog + 1.124 vektor); port TypeScript lulus 1.124 vektor yang sama + uji Schema; `no_std` lulus; `clippy -D warnings` bersih; uji mutasi TS: batas panjang email/lokal, `MAX_REPEAT`, parser float, dll. tertangkap |
| Dependensi wajib | LombokLocale (L0, **wajib**) — resolusi pesan error (ADR-009); LombokJSON (L0, **opsional**, dipakai bila dibutuhkan) — validasi berbasis JSON Schema, belum diimplementasikan |
| Tingkat (tier) | L1 |

## Contoh Pemakai (ilustrasi, bukan kepemilikan)

- Siapa pun yang menerima masukan dari pengguna, berkas, atau perangkat; tidak ada pemakai yang memiliki library ini.
- Contoh dependen di ekosistem Lombok dicatat hanya di `map_` (arah dependensi).

## Batasan yang Diketahui (Known Limitations)

- Email/URL bukan parser RFC 5322/3986 penuh (email berkutip dan literal IP ditolak; IDN harus punycode; URL tanpa validasi port/IPv6).
- Regex-lite: tanpa grup, lookaround, `{n,m}` di atas 1000 (dijepit), lazy quantifier; alternasi hanya tingkat atas; kelas `[a` tak tertutup diterima apa adanya. Backtracking dibatasi anggaran langkah, tetapi pola tetap harus dari sumber tepercaya.
- `strip_html` sederhana; bukan sanitizer.
- `to_lowercase` mengikuti aturan Unicode bahasa host (Rust/JS sama pada vektor; Go/Python/PHP belum diuji).
- Belum ada derive macro; belum ada validasi JSON Schema.
- Dependensi `path` ke LombokLocale harus diganti sebelum publish.
- Port Python/Go/PHP belum ada.

## Untuk Info Lebih Lanjut

- Cara pakai → [Guide How to Use](guide_how_to_use_LombokValidator_v0.1.0.md)
- Cara instal/deploy → [How to Dist](how_to_dist_LombokValidator_v0.1.0.md)
- API lengkap → [API](API_LombokValidator_v0.1.0.md)
- Kontrak lintas bahasa → [SPEC](SPEC_LombokValidator_v0.1.0.md)

## Gap vs Pembanding (U6)

| Pembanding | Kelebihan pembanding | Posisi LombokValidator (jujur) |
|---|---|---|
| validator / garde (Rust) | Derive macro, ekosistem serde | Belum ada derive macro; unggul pada nol dependensi eksternal dan kontrak lintas bahasa |
| Zod / Ajv / Joi | Skema deklaratif, JSON Schema | Belum ada JSON Schema (rencana 0.4.0 via LombokJSON opsional) |
| Keunikan yang diklaim | — | Error `code` + `messageId` identik lintas bahasa, siap i18n, `no_std + alloc` |
