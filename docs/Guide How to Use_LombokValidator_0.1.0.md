# Guide How to Use — LombokValidator 0.1.0

Panduan pengguna akhir: instalasi, quickstart 5 menit, cookbook resep umum, dan troubleshooting.

## 1. Instalasi

### TypeScript / JavaScript (npm)

```bash
npm install lombokvalidator          # v0.1.0 (E404 sampai dirilis — lihat How To Dist)
```

Syarat: Node ≥ 18. Zero runtime dependency. Dual format: ESM (`import`) dan CJS (`require`) — bundler mana pun dan Node lama terlayani.

### Rust

```toml
[dependencies]
lombokvalidator = { git = "https://github.com/codinglombok/LombokValidator", package = "lombokvalidator" }
# setelah ada di crates.io:
# lombokvalidator = "0.1"
```

Catatan build: crate bergantung pada `lomboklocale` lewat path `../../LombokLocale/rust` — untuk membangun dari git, letakkan clone `LombokLocale` sebagai sibling repo (CI melakukan hal yang sama; lihat TECH DEBT TD-1).

## 2. Quickstart 5 menit (TypeScript)

```ts
import * as V from "lombokvalidator";

// 1) Validasi satu field
const emailErrs = V.validateEmail("email", "budi@contoh.co.id"); // [] = valid
const ageErrs   = V.validateRangeI64("age", 17, 13, 120);        // []

// 2) Skema: kumpulkan SEMUA error sekaligus, nested didukung
const address = { validate: () => V.validateLength("postcode", "12", 5, 5) };
const errs = new V.Schema()
  .field("email", () => V.validateEmail("email", "bukan-email"))
  .field("age",   () => V.validateRangeI64("age", 5, 13, 120))
  .nested("address", address)
  .validateAll();

console.log(errs.map(e => `${e.field}: ${e.code}`));
// [ 'email: INVALID_EMAIL', 'age: BELOW_MIN', 'address.postcode: TOO_SHORT' ]

// 3) Pesan lokal via LombokLocale (struktur error tetap kontrak)
//    resolve(locale, catalog, e.code, e.messageId, V.errorArgs(e))
```

## 3. Quickstart 5 menit (Rust)

```rust
use lombokvalidator::*;

struct Signup { email: String, age: i64 }

impl Validate for Signup {
    fn validate(&self) -> ValidationResult {
        Schema::new()
            .field("email", || validate_email("email", &self.email))
            .field("age",   || validate_range_i64("age", self.age, Some(13), Some(120)))
            .validate_all()
    }
}

fn main() {
    let s = Signup { email: "budi@contoh.co.id".into(), age: 17 };
    assert!(s.validate().is_ok());
}
```

Jalankan contoh nyata dari repo: `cargo run --release --example smoke` (smoke test lengkap dengan nested + sanitasi + pola).

## 4. Cookbook

### 4.1 Form pendaftaran lengkap (kumpulkan semua error untuk ditampilkan)

Pola yang disarankan: satu `Schema` per struct, satu closure per field, tampilkan `field` + `code` ke UI, render teks via katalog. Jangan pernah mencocokkan teks pesan — cocokkan `code` (stabil). Urutan error mengikuti urutan registrasi field, deterministik.

### 4.2 Validasi ID UUID + normalisasi input

```ts
const id = V.trim(rawId).toLowerCase();         // sanitasi dulu
const ok = V.isUuid(id) && V.validateUuid("id", id).length === 0;
```

`to_lowercase`/`trim` memakai semantik Rust — aman untuk data yang akan dibandingkan antar layar/layanan lintas bahasa ekosistem.

### 4.3 Pola kustom (kode kantor, dll.)

```ts
V.validatePattern("kode", input, "[A-Z]{2}-\\d{4}")   // AB-1234
```

Jika pola berasal dari input pengguna, tetap aman: mesin punya anggaran langkah (`PATTERN_TOO_COMPLEX`), tidak akan hang. Namun biasakan pola dari kode, bukan dari user.

### 4.4 Rule kustom lintas tipe

```ts
const even = new V.ClosureValidator<number>((f, v) =>
  v % 2 === 0 ? [] : [{ field: f, code: "NOT_EVEN",
                        messageId: "validator.custom.not_even", args: {} }]);
```

Struktur empat kunci (`field`, `code`, `messageId`, `args`) wajib dipertahankan agar katalog dan kontrak tetap konsisten.

### 4.5 Pembersihan input sebelum disimpan

```ts
const name = V.stripHtml(V.trim(raw));   // "  <b>Wayan</b> " → "Wayan"
```

`strip_html` menghapus tag secara struktur (linear time). Untuk rich-text dari pengguna yang akan dirender HTML, ini BUKAN pengganti sanitizer (lihat Troubleshooting).

### 4.6 Angka besar / batas i64

TS: pakai `bigint` untuk nilai di luar ±2^53 (mis. ID database): `V.validateRangeI64("n", "9007199254740993n" as any, ...)`. Rust: `i64` native. Vector mengirim i64 sebagai string desimal sehingga konsumsi lintas bahasa bebas presisi-hilang.

### 4.7 Integrasi i18n penuh (Rust)

Lihat `docs/API_LombokValidator_0.1.0.md` §2 — build `Vec<(&str, Arg)>` dari `e.args` lalu `lomboklocale::resolve(...)`. `default_message()` tersedia sebagai fallback Inggris hardcoded untuk script/test cepat, bukan untuk produksi.

## 5. Troubleshooting

| Gejala | Sebab | Solusi |
|---|---|---|
| `cargo build` gagal: `no ArgValue in the root` versi lama | versi repo sebelum fix ADR-006 | tarik ≥ commit perbaikan; `ArgValue` kini ekspor lokal |
| `cargo build` gagal: `lomboklocale` tidak ditemukan | path-dep sibling hilang | clone `codinglombok/LombokLocale` di folder INDUK repo ini |
| Email valid ditolak (mis. `"quoted"@x.com`, `a@[127.0.0.1]`) | subset struktural (ADR-003) | by design; pantau SPEC untuk ekspansi di versi minor |
| `is_url("file:///etc/passwd")` = false | host kosong ditolak | by design, ter-lock vector |
| `tryMatchesPattern` mengembalikan `"TooComplex"` | budget 1.000.000 langkah habis | sederhanakan pola; perlakukan seperti error, bukan `false` |
| `strip_html("5 < 10")` tidak berubah | `<` tanpa `>` berikutnya dibiarkan | by design (fail-safe); jangan pakai untuk untrusted rich-text |
| `validate_length("日本語")` = 3, bukan 9 | hitungan per code point (ADR-005) | pakai `utf8Len` (compat) bila butuh byte |
| Hasil TS vs Rust beda? | tidak boleh — lapor bug | jalankan `npm test` (1124 vector) + `cargo test --release`; bandingkan kasus di `vectors/` |
| npm 404 saat `npm install lombokvalidator` | belum dipublikasikan (E404) | ikuti `How To Dist` untuk rilis, atau pasang dari git/tarball |

## 6. Kompatibilitas

- Node ≥ 18 (ESM & CJS), TypeScript ≥ 5.6 untuk tipe (`.d.ts` ikut dalam paket).
- Rust ≥ 1.75 (`rust-version`), `no_std + alloc` didukung penuh (fitur `std` default hanya pass-through ke LombokLocale).
- Semua fungsi murni dan bebas efek samping; aman untuk WebWorker/edge runtime selama `TextEncoder` tersedia (API standar Node 18/Browser).
