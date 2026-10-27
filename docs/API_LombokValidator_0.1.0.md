# API — LombokValidator 0.1.0

Referensi lengkap semua ekspor publik. Kontrak perilaku (byte-identical lintas bahasa) didefinisikan di `SPEC_LombokValidator_0.1.0.md`; dokumen ini mendokumentasikan signature dan cara pakai per bahasa. Kesepakatan penamaan: Rust `snake_case`, TypeScript `camelCase` — mapping 1:1 di tabel tiap fungsi. Semua contoh Rust memakai prelude `use lombokvalidator::*;` dan TS `import * as V from "lombokvalidator"`.

Tipe dasar (identik di kedua bahasa):

| Konsep | Rust | TypeScript |
|---|---|---|
| Satu error | `ValidationError { field: String, code: &'static str, messageId: &'static str, args: Vec<(String, ArgValue)> }` | `ValidationError { field: string, code: string, messageId: string, args: Record<string, ArgValue> }` |
| Hasil | `ValidationResult = Result<(), Vec<ValidationError>>` | `ValidationResult = ValidationError[]` (array kosong = valid) |
| Argumen pesan | `ArgValue::Str(String) \| Int(i64) \| UInt(u64) \| Float(f64)` | `ArgValue = string \| number \| bigint` |
| Kegagalan budget regex | `Result<bool, PatternError>` dengan `PatternError::TooComplex` | `boolean \| "TooComplex"` |

`ArgValue` adalah tipe kontrak milik LombokValidator (dimiliki repo ini, bukan LombokLocale) sehingga perubahan API locale tidak pernah menggeser kontrak; konversi ke `lomboklocale::Arg` hanya terjadi saat merender pesan katalog.

## 1. String — predikat

| Fungsi | Signature Rust | Signature TypeScript |
|---|---|---|
| `is_email` | `fn is_email(s: &str) -> bool` | `isEmail(s: string): boolean` |
| `is_url` | `fn is_url(s: &str) -> bool` | `isUrl(s: string): boolean` |
| `is_uuid` | `fn is_uuid(s: &str) -> bool` | `isUuid(s: string): boolean` |

Semua predikat murni, tanpa alokasi kecuali buffer internal kecil, O(n). Semantik detail (limit RFC 5321, subset RFC 3986, format RFC 4122) ada di SPEC §5.1–5.3. Contoh:

```rust
assert!(is_email("user@example.co.id"));
assert!(!is_url("https://exa mple.com"));   // spasi = whitespace Rust
assert!(is_uuid("550e8400-e29b-41d4-a716-446655440000"));
```

```ts
assert.equal(V.isEmail("user@example.co.id"), true);
assert.equal(V.isUrl("https://exa mple.com"), false);
assert.equal(V.isUuid("550e8400-e29b-41d4-a716-446655440000"), true);
```

## 2. String — validasi (menghasilkan error berstruktur)

| Fungsi | Signature Rust | Signature TypeScript |
|---|---|---|
| `validate_email` | `fn validate_email(field: &str, value: &str) -> ValidationResult` | `validateEmail(field: string, value: string): ValidationResult` |
| `validate_url` | `fn validate_url(field: &str, value: &str) -> ValidationResult` | `validateUrl(field, value)` |
| `validate_uuid` | `fn validate_uuid(field: &str, value: &str) -> ValidationResult` | `validateUuid(field, value)` |
| `validate_length` | `fn validate_length(field: &str, value: &str, min: Option<usize>, max: Option<usize>) -> ValidationResult` | `validateLength(field: string, value: string, min?: number \| null, max?: number \| null): ValidationResult` |
| `validate_pattern` | `fn validate_pattern(field: &str, value: &str, pattern: &str) -> ValidationResult` | `validatePattern(field, value, pattern): ValidationResult` |

Error yang mungkin: `INVALID_EMAIL`, `INVALID_URL`, `INVALID_UUID`, `TOO_SHORT` (args `min`, `actual`), `TOO_LONG` (args `max`, `actual`), `PATTERN_MISMATCH`, `PATTERN_TOO_COMPLEX`. Panjang dihitung per code point Unicode. Contoh dengan i18n (Rust):

```rust
let catalog = lomboklocale::parse_catalog(&std::fs::read_to_string("locales/id/lombokvalidator.json")?)?;
let err = validate_length("username", "ab", Some(4), None).unwrap_err();
let e = &err[0]; // TOO_SHORT, args: min=4, actual=2
let mut args: Vec<(&str, lomboklocale::Arg)> =
    e.args.iter().map(|(k, v)| (k.as_str(), lomboklocale::Arg::from(v))).collect();
args.push(("field", lomboklocale::Arg::Str(e.field.clone())));
let msg = lomboklocale::resolve("id", &catalog, e.code, e.message_id, &args);
assert_eq!(msg.text, "username minimal 4 karakter (saat ini 2)");
```

TypeScript menyediakan helper `errorArgs(e)` yang menambahkan `field` ke args sehingga pemanggilan `lomboklocale.resolve` tinggal satu baris.

## 3. Numerik

| Fungsi | Signature Rust | Signature TypeScript |
|---|---|---|
| `validate_range_i64` | `(field, value: i64, min: Option<i64>, max: Option<i64>) -> ValidationResult` | `(field, value: bigint \| number, min?: bigint \| number \| null, max?: bigint \| number \| null)` |
| `validate_range_f64` | `(field, value: f64, min: Option<f64>, max: Option<f64>) -> ValidationResult` | `(field, value: number, min?: number \| null, max?: number \| null)` |
| `validate_positive_i64` | `(field, value: i64) -> ValidationResult` | `(field, value: bigint \| number)` |
| `validate_positive_f64` | `(field, value: f64) -> ValidationResult` | `(field, value: number)` |
| `is_integer_str` | `fn is_integer_str(s: &str) -> bool` | `isIntegerStr(s: string): boolean` |
| `is_float_str` | `fn is_float_str(s: &str) -> bool` | `isFloatStr(s: string): boolean` |

Error: `BELOW_MIN` (args `min`, `actual`), `ABOVE_MAX` (args `max`, `actual`), `NOT_POSITIVE` (arg `actual`). `i64` di TS memakai `bigint` agar presisi penuh; `number` diterima untuk nilai aman. Batas `i64::MIN/MAX` aman dipakai.

## 4. regex-lite (mesin pola)

| Fungsi | Signature Rust | Signature TypeScript |
|---|---|---|
| `try_matches_pattern` | `fn try_matches_pattern(input: &str, pattern: &str) -> Result<bool, PatternError>` | `tryMatchesPattern(input: string, pattern: string): boolean \| "TooComplex"` |
| `matches_pattern` | `fn matches_pattern(input: &str, pattern: &str) -> bool` (fail-closed: `false` saat TooComplex) | `matchesPattern(input, pattern): boolean` |
| Konstanta | `pub const MAX_STEPS: u64 = 1_000_000` | `MAX_STEPS = 1_000_000` |

Fitur & larangan, aturan anggaran langkah, dan perlakuan pola malformed dijelaskan normatif di SPEC §5.5. Full-string match; gunakan `^`/`$` eksplisit bila perlu anchor (memang full-string, anchor berguna di dalam alternasi).

## 5. Sanitasi

| Fungsi | Signature Rust | Signature TypeScript |
|---|---|---|
| `trim` | `fn trim(s: &str) -> &str` | `trim(s: string): string` |
| `to_lowercase` | `fn to_lowercase(s: &str) -> String` | `toLowercase(s: string): string` |
| `strip_html` | `fn strip_html(s: &str) -> String` | `stripHtml(s: string): string` |

`trim` memakai properti Unicode `White_Space` (semantik Rust — NBSP dan U+3000 ikut terpotong). `strip_html` menghapus rentang `<…>` lengkap, linear time, tanpa dekode entitas; bukan sanitizer HTML (lihat `Guide How to Use` §Troubleshooting).

## 6. Skema & validator kustom

| Ekspor | Signature Rust | Signature TypeScript |
|---|---|---|
| trait `Validate` | `fn validate(&self) -> ValidationResult` | `interface Validate { validate(): ValidationResult }` |
| `Schema` | `Schema::new().field(name, closure).nested(name, &child).validate_all()` | `new Schema().field(name, fn).nested(name, child).validateAll()` |
| trait `FieldValidator<T>` | `fn check(&self, field: &str, value: &T) -> ValidationResult` | `interface FieldValidator<T> { check(field, value): ValidationResult }` |
| `ClosureValidator` | `ClosureValidator(closure)` mengimplement `FieldValidator<T>` | `new ClosureValidator<T>((field, value) => ValidationResult)` |
| `merge_results` | `fn merge_results(results: Vec<ValidationResult>) -> ValidationResult` | `mergeResults(rs: ValidationResult[]): ValidationResult` |
| helper args | — (build `Vec<(&str, Arg)>` manual) | `errorArgs(e: ValidationError): Record<string, ArgValue>` |

`Schema` menjalankan SEMUA rule lalu menggabungkan error (bukan fail-fast) dengan urutan registrasi; `nested()` menambahkan prefix `"<name>."` pada setiap `field` error anak. Contoh lengkap ada di `rust/examples/smoke.rs` dan `Guide How to Use` §Quickstart.

## 7. Kode error [KODE]

Daftar kode stabil + `messageId` + args: lihat `SPEC_LombokValidator_0.1.0.md` §3 (tabel normatif — jangan diduplikasi di sini agar tidak bisa berbeda). Kode kustom aplikasi bebas, asal struktur empat-kunci dipertahankan.

## 8. Kompilasi bersyarat (khusus Rust)

- `default = ["std"]`; `--no-default-features` menghasilkan build `no_std + alloc` (bukti: CI job `rust`).
- `dev-dependencies` (`serde_json`) hanya untuk test vector — tidak pernah ter-link ke pustaka runtime. S1: satu-satunya runtime dependency adalah `lomboklocale` (L0, diizinkan ADR-009).
