# How To Dist — LombokValidator 0.1.0

Prosedur distribusi: build artefak, publish per registry, verifikasi pasca-rilis, dan rollback. Aturan mutlak ekosistem: **publish hanya atas perintah eksplisit pemilik repo** (lihat PUBLIKASI pada prompt sesi). Target publikasi resmi repo ini: **npm** (port TS) — sesuai BLOK REPO; crates.io mengikuti bila diminta.

## 1. Prasyarat rilis (semua wajib hijau)

| Gate | Perintah | Kriteria |
|---|---|---|
| Rust test + vector | `cd rust && cargo test --release` | semua ok (35 test, 1124 vector) |
| Rust toolchain ketat | `cd rust && cargo clippy --all-targets --all-features -- -D warnings` | nol warning |
| Rust no_std | `cd rust && cargo build --no-default-features` | sukses |
| Smoke acuan | `cd rust && cargo run --release --example smoke` | `SMOKE OK` |
| TS test | `cd typescript && npm test` | 18/18 + 1124 vector |
| TS coverage | `cd typescript && npm run coverage` | ≥95 stmt / ≥90 branch (aktual 99.18/98.89) |
| Smoke paket | `npm pack` + smoke di dir bersih | ESM+CJS OK |
| Dokumen | `./scripts/lombok-doctor-docs.sh LombokValidator .` | `OK: lombok doctor docs passed` |
| Versi konsisten | `Cargo.toml` == `package.json` == nama dokumen docs | `0.1.0` |

## 2. Build artefak

### 2.1 TypeScript (satu-satunya artefak publik wajib)

```bash
cd typescript
npm ci                # reproducible dari package-lock.json
npm run build         # dist/src (ESM + .d.ts) + dist/cjs (CJS + package.json bertipe commonjs)
npm pack --pack-destination ../dist-artifacts   # lombokvalidator-0.1.0.tgz
```

Isi tgz dijaga `files`: `dist/src`, `dist/cjs`, `README.md`. Verifikasi cepat isi:

```bash
tar -tzf ../dist-artifacts/lombokvalidator-0.1.0.tgz | sort
# wajib ada: dist/src/index.js, dist/src/index.d.ts, dist/cjs/index.js, dist/cjs/package.json
```

### 2.2 Rust (bila crates.io diminta)

```bash
cd rust
cargo build --release            # artefak rlib
cargo package --list             # audit isi crate (jangan sampai target/ ikut)
cargo package                    # .crate di target/package/
```

## 3. Publish per registry

### 3.1 npm (port TS) — jalur utama

```bash
npm whoami                                       # WAJIB terverifikasi dulu
npm view lombokvalidator version                 # WAJIB 404/belum ada 0.1.0
cd typescript
npm publish --access public                      # HANYA setelah konfirmasi eksplisit pemilik
```

Checklist sebelum menekan enter: versi `package.json` = target rilis; `npm pack --dry-run` sudah diaudit; changelog + docs pack siap; CI hijau di commit yang sama.

### 3.2 crates.io (hanya bila diminta)

Ganti dulu path-dep ke registry (TD-1): `lomboklocale = { version = "0.2", default-features = false }`, jalankan ulang semua gate, lalu:

```bash
cargo login <token>        # token dari owner, tanpa menyimpan di repo
cargo publish --dry-run
cargo publish
```

Registry lain (PyPI/Packagist/go module) mengikuti prosedur resmi bahasanya — port python/go/php belum ada (stub), jadi tidak relevan di 0.1.0.

## 4. Verifikasi pasca-rilis

1. `npm view lombokvalidator version` → `0.1.0`.
2. Direktori bersih: `npm init -y && npm install lombokvalidator` → jalankan smoke (impor ESM + require CJS; contoh skrip ada di sesi engineering, `smoke-npm-dist.mjs`).
3. Pastikan `dist/cjs/package.json` ikut terpasang (`ls node_modules/lombokvalidator/dist/cjs/`) — tanpa itu `require` gagal di paket `type: module`.
4. (crates.io) `cargo add lombokvalidator` di proyek uji + `cargo test`.
5. Buat git tag `v0.1.0` + GitHub Release dengan catatan = Changelog; tag menandai SHA commit yang persis dipublikasikan.

## 5. Rollback

- npm: `npm deprecate lombokvalidator@0.1.0 "alasan"` (jangan unpublish bila sudah dikonsumsi; npm hanya mengizinkan unpublish ≤72 jam dengan syarat ketat). Rilis patch `0.1.1` berisi fix, jelaskan di Changelog.
- crates.io: tidak ada delete — yank (`cargo yank --vers 0.1.0`) lalu rilis versi baru.
- Git: tag rusak → hapus tag (`git push --delete origin v0.1.0`), perbaiki, re-tag; jangan pernah memindahkan tag ke SHA lain setelah publikasi registry.

## 6. Pasca-rilis yang sering terlewat

- Perbarui badge/README: ganti kalimat "not yet published".
- Arsipkan SHA-256 vector + hash tgz di catatan release (reproducibility audit).
- Jalankan ulang `lombok-doctor-docs.sh` — dokumen dengan nama `[Versi]` lama tetap; versi baru berarti dokumen baru dibuat (naming ikut versi).
