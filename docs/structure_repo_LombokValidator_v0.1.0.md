# LombokValidator — STRUCTURE REPO v0.1.0

## 1. Struktur Folder

```
LombokValidator/
├── rust/            # src/{lib,error,string,pattern,number,sanitize,schema}.rs · tests/{vectors,catalogs,robustness}.rs
├── typescript/      # src/{index,compat}.ts · test/vectors.test.ts (lulus vektor bersama)
├── python/ go/ php/ # stub README
├── locales/{en,id,es,fr,de,pt,ru,ja,ko,zh-Hans}/lombokvalidator.json  + STATUS.md
├── vectors/         # lombokvalidator-vectors-v1.json (1.124 kasus) + gen_inputs.py
├── docs/  scripts/  .github/workflows/{ci,release}.yml
├── README.md  LICENSE  .gitignore
```

## 2. Konvensi Penamaan (mengikuti MASTERPLAN_UTAMA §4)

| Platform | Nama di repo ini |
|---|---|
| GitHub repo | `codinglombok/LombokValidator` |
| Package registry (npm/PyPI/crates.io) | `lombokvalidator` |
| Packagist | `codinglombok/lombokvalidator` |
| Go module | `github.com/codinglombok/lombokvalidator/go` (tag `go/vX.Y.Z`) |
| Namespace/Class | `LombokValidator` (Rust: `lombokvalidator::`) |

## 3. File Wajib di Root

| File | Ada? |
|---|---|
| `README.md` | ✅ |
| `LICENSE` (dual-license: `LICENSE-APACHE` + `LICENSE-MIT`) | ✅ |
| `.gitignore` | ✅ |
| `CHANGELOG.md` → lihat `changelog_LombokValidator_v0.1.0.md` | ✅ |
| `.github/workflows/ci.yml` | ✅ |
| `.github/workflows/release.yml` | ✅ |
| 12 dokumen standar (`docs/`) | ✅ — lihat daftar di [Masterplan](masterplan_LombokValidator_v0.1.0.md) |

## 4. Struktur per Bahasa Port

- `rust/` — implementasi referensi; bergantung pada `../../LombokLocale/rust` (path) — **ganti ke versi registry sebelum publish**.
- `typescript/` — port native tanpa dependensi runtime (`lomboklocale` hanya `peerDependency` opsional untuk merender pesan); `npm test` menjalankan seluruh vektor.
- `python/ go/ php/` — stub.

## 5. Catatan

Sebelum rilis, `Cargo.toml` harus memakai `lomboklocale = "0.1"` (versi registry), bukan `path`, karena crates.io menolak dependensi path tanpa versi.
