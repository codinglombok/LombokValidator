# LombokValidator — API REFERENCE v0.1.0

Seluruh API publik, per bahasa, dengan kode error dan status stabilitas (ARCHITECTURE_UTAMA §5, baris "Dokumen" — normatif sejak MASTERPLAN_UTAMA v3.3 §1.1/§5).

Legenda stabilitas: 🟢 **Stable** (breaking change butuh major bump + RFC) · 🟡 **Beta** (boleh berubah, diumumkan di changelog) · 🔴 **Experimental** (bisa hilang tanpa notice).

## Rust (referensi)

| Simbol | Tanda tangan | Stabilitas |
|---|---|---|
| `is_email/is_url/is_uuid` | `(&str) -> bool` | 🟢 |
| `validate_length` | `(field, value, Option<usize> min, Option<usize> max) -> ValidationResult` | 🟢 |
| `validate_email/url/uuid` | `(field, value) -> ValidationResult` | 🟢 |
| `validate_pattern` | `(field, value, pattern) -> ValidationResult` (`PATTERN_MISMATCH` / `PATTERN_TOO_COMPLEX`) | 🟡 |
| `matches_pattern` / `try_matches_pattern` | `(input, pattern) -> bool` (fail-closed) / `-> Result<bool, PatternError>` | 🟡 |
| `validate_range_i64/f64` | `(field, value, Option min, Option max) -> ValidationResult` | 🟢 |
| `validate_positive_i64/f64` | `(field, value) -> ValidationResult` | 🟢 |
| `is_integer_str/is_float_str` | `(&str) -> bool` | 🟢 |
| `trim/to_lowercase/strip_html` | `(&str) -> &str / String / String` | 🟢 |
| `Validate` | `trait { fn validate(&self) -> ValidationResult }` | 🟢 |
| `Schema` | `new().field(name, closure).nested(name, &dyn Validate).validate_all()` | 🟡 |
| `FieldValidator<T>` / `ClosureValidator` | `check(field, &T) -> ValidationResult` | 🟡 |

### Kode Error

| `code` | `message_id` | Args |
|---|---|---|
| `TOO_SHORT` | `validator.string.too_short` | `min`, `actual` |
| `TOO_LONG` | `validator.string.too_long` | `max`, `actual` |
| `INVALID_EMAIL` | `validator.string.invalid_email` | — |
| `INVALID_URL` | `validator.string.invalid_url` | — |
| `INVALID_UUID` | `validator.string.invalid_uuid` | — |
| `PATTERN_MISMATCH` | `validator.string.pattern_mismatch` | — |
| `PATTERN_TOO_COMPLEX` | `validator.string.pattern_too_complex` | — |
| `BELOW_MIN` | `validator.number.below_min` | `min`, `actual` |
| `ABOVE_MAX` | `validator.number.above_max` | `max`, `actual` |
| `NOT_POSITIVE` | `validator.number.not_positive` | `actual` |

## Port Lain

**TypeScript** (`typescript/src/index.ts`): padanan camelCase (`isEmail`, `validateLength`, `validateRangeI64` menerima `bigint`, `tryMatchesPattern` → `boolean | "TooComplex"`, `stripHtml`, `Schema`, `ClosureValidator`, `errorArgs`). `ValidationResult` = larik `ValidationError[]` (kosong = valid). Argumen `i64` pada error berbentuk `bigint`. Lulus seluruh vektor.

Python/Go/PHP: belum ada.

## Kompatibilitas Lintas Bahasa

Setiap fungsi di atas **harus** menghasilkan output byte-identik lintas port untuk input yang sama, dibuktikan oleh `vectors/lombokvalidator-vectors-v1.json` (lihat [SPEC](SPEC_LombokValidator_v0.1.0.md)). Perbedaan hasil antar-port adalah bug, bukan variasi yang sah (ADR-015).
