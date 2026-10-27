# How To Dist — LombokValidator 0.1.0

Distribution procedure: build artifacts, publish per registry, post-release verification, and rollback. Absolute ecosystem rule: **publish only on the explicit command of the repo owner** (see PUBLIKASI in the session prompt). This repo's official publication target: **npm** (TS port) — per the REPO BLOCK; crates.io follows if requested.

## 1. Release prerequisites (all must be green)

| Gate | Command | Criteria |
|---|---|---|
| Rust tests + vectors | `cd rust && cargo test --release` | all pass (35 tests, 1124 vectors) |
| Rust strict toolchain | `cd rust && cargo clippy --all-targets --all-features -- -D warnings` | zero warnings |
| Rust no_std | `cd rust && cargo build --no-default-features` | succeeds |
| Reference smoke | `cd rust && cargo run --release --example smoke` | `SMOKE OK` |
| TS tests | `cd typescript && npm test` | 18/18 + 1124 vectors |
| TS coverage | `cd typescript && npm run coverage` | ≥95 stmt / ≥90 branch (actual 99.18/98.89) |
| Package smoke | `npm pack` + smoke in a clean dir | ESM+CJS OK |
| Documents | `./scripts/lombok-doctor-docs.sh LombokValidator .` | `OK: lombok doctor docs passed` |
| Consistent version | `Cargo.toml` == `package.json` == docs filename version | `0.1.0` |

## 2. Build artifacts

### 2.1 TypeScript (the only mandatory public artifact)

```bash
cd typescript
npm ci                # reproducible from package-lock.json
npm run build         # dist/src (ESM + .d.ts) + dist/cjs (CJS + commonjs-typed package.json)
npm pack --pack-destination ../dist-artifacts   # lombokvalidator-0.1.0.tgz
```

The tgz contents are controlled by `files`: `dist/src`, `dist/cjs`, `README.md`. Quick content check:

```bash
tar -tzf ../dist-artifacts/lombokvalidator-0.1.0.tgz | sort
# must include: dist/src/index.js, dist/src/index.d.ts, dist/cjs/index.js, dist/cjs/package.json
```

### 2.2 Rust (if crates.io is requested)

```bash
cd rust
cargo build --release            # rlib artifact
cargo package --list             # audit crate contents (make sure target/ stays out)
cargo package                    # .crate in target/package/
```

## 3. Publish per registry

### 3.1 npm (TS port) — primary path

```bash
npm whoami                                       # MUST be verified first
npm view lombokvalidator version                 # MUST be 404/not yet 0.1.0
cd typescript
npm publish --access public                      # ONLY after the owner's explicit confirmation
```

Checklist before pressing enter: `package.json` version = release target; `npm pack --dry-run` audited; changelog + docs pack ready; CI green on the same commit.

### 3.2 crates.io (only if requested)

First replace the path-dep with the registry (TD-1): `lomboklocale = { version = "0.2", default-features = false }`, re-run all gates, then:

```bash
cargo login <token>        # token from the owner, never stored in the repo
cargo publish --dry-run
cargo publish
```

Other registries (PyPI/Packagist/go modules) follow their language's official procedure — the python/go/php ports do not exist yet (stubs), so they are not relevant in 0.1.0.

## 4. Post-release verification

1. `npm view lombokvalidator version` → `0.1.0`.
2. Clean directory: `npm init -y && npm install lombokvalidator` → run the smoke (ESM import + CJS require; example script lives in the engineering session, `smoke-npm-dist.mjs`).
3. Confirm `dist/cjs/package.json` is installed too (`ls node_modules/lombokvalidator/dist/cjs/`) — without it `require` fails under a `type: module` package.
4. (crates.io) `cargo add lombokvalidator` in a test project + `cargo test`.
5. Create the git tag `v0.1.0` + a GitHub Release with the Changelog as notes; the tag marks the exact published commit SHA.

## 5. Rollback

- npm: `npm deprecate lombokvalidator@0.1.0 "reason"` (do not unpublish once consumed; npm only allows unpublish ≤72h under strict conditions). Ship a patch release `0.1.1` with the fix, explained in the Changelog.
- crates.io: no deletion — yank (`cargo yank --vers 0.1.0`) then release a new version.
- Git: broken tag → delete it (`git push --delete origin v0.1.0`), fix, re-tag; never move a tag to a different SHA after a registry publish.

## 6. Post-release steps often missed

- Update badges/README: replace the "not yet published" sentence.
- Archive the SHA-256 of the vectors + the tgz hash in the release notes (reproducibility audit).
- Re-run `lombok-doctor-docs.sh` — documents with the old `[Version]` name stay; a new version means new documents are created (naming follows the version).
