# LombokValidator — GUIDE: HOW TO USE v0.1.0

## Instalasi

```toml
# Rust — saat ini via git (belum di crates.io)
[dependencies]
lombokvalidator = { git = "https://github.com/codinglombok/LombokValidator", package = "lombokvalidator" }
```

```bash
# TypeScript — saat ini dari sumber (belum di npm)
cd typescript && npm install && npm run build
```

Setelah rilis pertama: `cargo add lombokvalidator` · `npm i lombokvalidator`. Python/Go/PHP: belum ada port.

## Konsep Dasar

- **`ValidationResult`** = `Ok(())` atau `Err(Vec<ValidationError>)`.
- **`ValidationError`** = `field` + `code` (kontrak) + `message_id` (terjemahan) + `args`.
- **`Schema`** merangkai aturan bernama dan menggabungkan hasilnya (tidak berhenti di kegagalan pertama).

## Contoh Penggunaan

```rust
use lombokvalidator::*;
struct Signup { email: &'static str, age: i64 }
impl Validate for Signup {
    fn validate(&self) -> ValidationResult {
        Schema::new()
            .field("email", || validate_email("email", self.email))
            .field("age", || validate_range_i64("age", self.age, Some(13), Some(120)))
            .validate_all()
    }
}
assert!(Signup { email: "a@b.co", age: 20 }.validate().is_ok());
```

Pesan terjemahan: lihat README (`lomboklocale::resolve` dengan `locales/id/lombokvalidator.json`).

## Pola Pemakaian Umum (Recipes)

- **Form dengan banyak field**: satu `Schema` per struct; tampilkan semua error sekaligus.
- **Objek bersarang**: `Schema::new().nested("address", &self.address)` → error `address.postcode`.
- **Aturan domain**: `ClosureValidator(|field, value| ...)`.
- **Validasi konfigurasi perangkat**: rentang + pola pada nilai yang dibaca dari berkas/EEPROM.

## Kesalahan Umum (Common Pitfalls)

- `strip_html` bukan pengaman XSS.
- Jangan menerima pola regex dari pengguna akhir.
- Sertakan `field` dan argumen `field` saat memanggil `resolve` — katalog memakai `{field}`.

## Lihat Juga

- [API Reference lengkap](API_LombokValidator_v0.1.0.md)
- [Full Summary](full_summary_project_LombokValidator_v0.1.0.md)
