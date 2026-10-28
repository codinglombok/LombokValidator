# LombokValidator — DEVELOPMENT IDE / ARAH PENGEMBANGAN v0.1.0

"Ide" di sini = arah pengembangan (development direction), bukan editor kode.

## 1. Roadmap

| Versi | Isi | Status |
|---|---|---|
| 0.1.0 | Inti Rust + port TypeScript + vektor 1.124 kasus + 10 katalog | ✅ (belum dipublikasikan) |
| 0.2.0 | `lombokvalidator-derive`; dependensi path → registry; port Python | ⚪ |
| 0.3.0 | Port Go/PHP | ⚪ |
| 0.4.0 | Validasi JSON Schema (LombokJSON opsional via feature `json`) | ⚪ |

## 2. Yang Sengaja Belum Dikerjakan (Deferred Scope)

Derive macro; JSON Schema; validasi async/lintas-field (mis. `password == confirm`).

## 3. Prinsip Desain untuk Kontributor Baru

Validator murni (tanpa I/O); setiap aturan baru = `code` baru + `message_id` baru + entri katalog `en` & `id` + vector.

## 4. Cara Berkontribusi

1. Baca [SPEC_LombokValidator_v0.1.0.md](SPEC_LombokValidator_v0.1.0.md) — perilaku yang diubah harus tetap sesuai kontrak, atau kontrak diajukan perubahan lewat RFC (lihat ARCHITECTURE_UTAMA §2, aturan L0 "perubahan breaking butuh RFC").
2. Tambahkan/ubah test vector di `vectors/lombokvalidator-vectors-v1.json` **sebelum** mengubah implementasi (test-first, selaras ADR-002/ADR-015).
3. Jalankan `./scripts/lombok-doctor-docs.sh LombokValidator` — PR yang mengubah API publik wajib memperbarui `API_LombokValidator_v0.1.0.md` di PR yang sama.
4. `cargo test && cargo clippy -- -D warnings` harus hijau sebelum membuka PR.

## 5. Pertanyaan Terbuka (Open Questions)

- Perlu validasi lintas-field pada v0.2.0?
