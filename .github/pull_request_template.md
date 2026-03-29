## 概要

<!-- 変更の目的と動作を簡潔に -->

## チェックリスト

- [ ] `cargo fmt --all -- --check` / `cargo clippy --workspace --all-targets -- -D warnings` / `cargo test --workspace`
- [ ] **unsafe / FFI を触れた場合** `crates/engine_core/SAFETY.md` のレビュー項目を確認した
- [ ] 公開 API の破壊的変更がある場合、意図と移行メモを書いた

## Unsafe / FFI（該当時のみ）

- 触ったファイル:
- `// SAFETY:` コメントとスレッド前提:
- 新規 `unsafe impl Send/Sync` の有無と根拠:
