---
name: release_workflow
description: Guides release builds and distribution packaging from version bump through asset archive creation.
---

# Release Workflow

## Prerequisites
- All tests pass (see test workflow).
- Version number has been updated.

## Steps

### 1. Update versions
Update versions in each crate's `Cargo.toml` following semantic versioning.

```toml
[package]
version = "X.Y.Z"
```

### 2. Run all tests
```bash
cargo fmt --all -- --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace
```

### 3. Build release
```bash
cargo build --workspace --release
```

### 4. Verify `game_player` distribution binary
```bash
ls target/release/game_player*
```

Release binaries are generated under `target/release/`.

### 5. Create asset archive
Archive the `assets/` directory for distribution.

```bash
cargo run -p game_player --release -- --pack-assets ./assets -o ./dist/assets.pak
```

### 6. Distribution package structure
```text
dist/
├── game_player.exe    # (Windows) main binary
├── assets.pak         # asset archive
├── config.toml        # runtime settings
└── README.txt         # user-facing notes
```

### 7. Create Git tag
```bash
git tag -a v{VERSION} -m "Release v{VERSION}"
git push origin v{VERSION}
```

## Versioning Rules
- MAJOR: breaking changes (e.g., save-data compatibility loss).
- MINOR: new backward-compatible features.
- PATCH: bug fixes.
