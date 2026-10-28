# LombokValidator — LANG (i18n) v0.1.0

Tingkat i18n, katalog ID pesan, dan cakupan bahasa untuk `LombokValidator` (ARCHITECTURE_UTAMA §7, ADR-009: "Library L1+ → hanya menyimpan ID pesan + teks en; terjemahan di-load via LombokLocale").

## 1. Tingkat i18n Repo Ini

**Tingkat F** menurut MASTERPLAN_UTAMA §13 — semua error membawa `message_id`; teks `en` di `locales/en/lombokvalidator.json`, terjemahan dimuat lewat LombokLocale (ADR-009). `code` tidak diterjemahkan. **Kesenjangan jujur:** katalog baru `en` + `id` (2 dari 20 bahasa Core-20).

## 2. Katalog ID Pesan

| `messageId` | Teks EN (default) | Digunakan di |
|---|---|---|
| `validator.string.too_short` | {field} must be at least {min} characters (got {actual}) | `validate_length` |
| `validator.string.too_long` | {field} must be at most {max} characters (got {actual}) | `validate_length` |
| `validator.string.invalid_email` | {field} is not a valid email address | `validate_email` |
| `validator.string.invalid_url` | {field} is not a valid URL | `validate_url` |
| `validator.string.invalid_uuid` | {field} is not a valid UUID | `validate_uuid` |
| `validator.string.pattern_mismatch` | {field} does not match the required pattern | `validate_pattern` |
| `validator.string.pattern_too_complex` | {field} pattern is too complex to evaluate safely | `validate_pattern` |
| `validator.number.below_min` | {field} must be at least {min} (got {actual}) | `validate_range_*` |
| `validator.number.above_max` | {field} must be at most {max} (got {actual}) | `validate_range_*` |
| `validator.number.not_positive` | {field} must be positive (got {actual}) | `validate_positive_*` |
| `validator.custom.not_even` | {field} must be an even number | contoh kustom (dokumentasi) |

## 3. Cakupan Bahasa (Core-20 + Nusantara)

| Bahasa | Kode BCP-47 | Status katalog |
|---|---|---|
| Bahasa Indonesia | `id` | ✅ `locales/id/lombokvalidator.json` |
| English | `en` | ✅ `locales/en/lombokvalidator.json` (default/sumber kebenaran) |
| Español, Français, Deutsch, Português, Русский, 日本語, 한국어, 简体中文 | `es` `fr` `de` `pt` `ru` `ja` `ko` `zh-Hans` | 🟡 **draf terjemahan mesin — wajib ditinjau penutur asli** (`locales/STATUS.md`) |
| Core-20 lainnya (`ar`, `hi`, `bn`, `tr`, `vi`, `th`, `it`, `pl`, `nl`, `uk`) + Nusantara (`jv`, `su`, `ms`) | — | ⚪ belum ada (`ar` butuh RTL) |

## 4. Cara Menambah Bahasa Baru

1. Salin `locales/en/lombokvalidator.json` ke `locales/<bcp47>/lombokvalidator.json`.
2. Terjemahkan setiap value; jangan ubah `messageId` (key).
3. Jalankan test integrasi katalog (`cargo test catalog`) — memverifikasi setiap `messageId` di §2 punya entri di file baru.
4. Perbarui tabel §3 di dokumen ini pada PR yang sama.

## 5. Ketergantungan pada LombokLocale

Konsumen memanggil `lomboklocale::resolve(locale, &catalog, err.code, err.message_id, &args)` dengan `args` = `err.args` + `("field", err.field)`. Validator sendiri tidak memuat katalog; pemanggil memuatnya (`include_str!` atau berkas).
