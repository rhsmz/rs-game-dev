## Summary

As p1a of Phase 1 (workspace initialization), this PR defines the root `Cargo.toml` and workspace members (`engine_core`, `script_editor`, `game_player`). Since the same content already exists in `develop`, this PR is for history alignment and plan traceability.

## Changes

- [x] Defined `[workspace]` and `members` in root `Cargo.toml`
- [x] Set `resolver = "2"` and `[workspace.package]` (version, edition, license, rust-version)
- [x] Defined shared dependencies in `[workspace.dependencies]` (serde, rmp-serde, anyhow, thiserror, log, env_logger)
- [x] Defined shared Clippy lint settings in `[workspace.lints.clippy]`

## Test Plan

```bash
cargo check --workspace
```

## Related Issue

<!-- If applicable: Closes #<number> -->
