# Development Idea — LombokValidator 0.1.0

Backlog ide terurut prioritas (P0 = sudah/sedang, P1 = berikutnya, P2/P3 = nanti) dengan kriteria selesai (Definition of Done) dan non-goals. Ide baru masuk sini dulu, lalu dipromosikan ke Masterplan/Milestone bila disetujui.

## P0 — Fondasi (selesai di 0.1.0)

### ID-01 Kontrak vector lintas bahasa ✅
Generator deterministik + 1124 kasus + harness Rust & TS. **DoD**: kedua bahasa lulus 100%; hash tercatat di SPEC; dokter dokumen memvalidasi. (Tercapai.)

### ID-02 Anti-ReDoS by construction ✅
Anggaran langkah 1.000.000 + fail-closed. **DoD**: kasus `a*a*a*a*a*a*c` pada 3000×`a` menghasilkan `PATTERN_TOO_COMPLEX`, bukan hang; pseudo-fuzz tidak panik. (Tercapai.)

### ID-03 Dual build ESM+CJS ✅
`exports` map lengkap + smoke paket. **DoD**: `import` dan `require` lulus contoh nyata dari tarball. (Tercapai.)

## P1 — Berikutnya

### ID-04 Registry dependency (TD-1)
Ganti path-dep `lomboklocale` → crates.io. **DoD**: `cargo build` tanpa sibling clone; CI tanpa checkout sibling; publish dry-run bersih.

### ID-05 Port ketiga (pilih python ATAU go sesuai prioritas README ekosistem)
**DoD**: lulus 100% vector file yang sama; ruff/mypy strict atau go vet CLEAN; coverage ≥95/90; job CI ber-hashFiles menyala (tidak lagi skipped); dokumen Language diperbarui.

### ID-06 Ekspansi vector dari lapangan
Kumpulkan kasus salah-validasi nyata (issue) → jadikan vector dulu (red di port yang salah), baru perbaiki. **DoD**: setiap bug fix menyertakan ≥1 kasus vector baru + bump hash SPEC + entri Changelog.

### ID-07 Gate coverage Rust di CI (TD-7)
**DoD**: job rust menjalankan `cargo llvm-cov --fail-under-lines 95` hijau.

## P2 — Menengah

### ID-08 Review katalog oleh penutur asli (TD-3)
**DoD**: minimal 5 bahasa bertanda "reviewed" di locales/STATUS.md dengan nama reviewer/tanggal.

### ID-09 Opsi pesan langsung (non-katalog)
API opsional `with_message_text` untuk aplikasi tanpa LombokLocale. **DoD**: default tetap struktur-only; vector tidak berubah; fitur off = byte-identical.

### ID-10 Kumpulan preset skema umum
Preset `signup`, `login`, `address` (komposisi validator yang ada saja, tanpa logika baru). **DoD**: preset murni fungsi pabrik; semua test vector tetap; docs cookbook diperbarui.

### ID-11 Benchmark deterministik
Criterion (Rust) + benchmark Node untuk mesin pattern & is_email. **DoD**: angka disimpan dengan seed/timestamp; regression >20% menandai review (informasi, bukan gate).

## P3 — Jauh

### ID-12 Feature `json` (TD-8) — JSON-Schema subset via LombokJSON
**DoD**: feature flag off tetap no_std; kontrak baru via vector tambahan; docs + Masterplan 0.6.0.

### ID-13 Derive macro (TD-9)
**DoD**: crate terpisah; hasil derive identik dengan Schema manual pada test paralel.

### ID-14 WASM port
Target `wasm32` untuk dipakai di browser/edge dengan hasil identik. **DoD**: suite vector lulus di wasm; jalur SIMD (bila ada) identik dengan skalar (DELTA B).

### ID-15 Katalog tambahan Core-20 (ar/hi/bn/tr/vi/th/it/pl/nl/uk) + bahasa Nusantara (jv/su/ms)
**DoD**: kunci/placeholder identik (test otomatis) + review penutur; ar perlu keputusan RTL/BiDi di LombokLocale dulu.

## Non-goals (ditolak demi fokus & keamanan kontrak)

1. **Sanitizer HTML lengkap** — bukan wilayah pustaka ini (ADR-008; arahkan ke LombokHTML).
2. **Parser email/URL RFC penuh** (quoted-local, IP-literal, comment folding) — subset struktural sudah dikunci kontrak; ekspansi hanya via kontrak baru (MAJOR/MINOR sesuai SPEC §10).
3. **Regex penuh** (grup, lookaround, backreference, lazy) — melanggar determinisme lintas bahasa & anggaran keamanan (ADR-004).
4. **Validasi lintas-field yang stateful/timing** (mis. konfirmasi password berbasis waktu) — milik aplikasi; pustaka tetap murni dan bebas efek samping.
5. **Async I/O / network calls** — tidak akan pernah ada; semua fungsi murni.
6. **Telemetry/loging bawaan** — tidak pernah ada (U10).
