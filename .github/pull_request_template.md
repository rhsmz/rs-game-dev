## 概要

<!-- 変更の目的と動作を簡潔に -->

## チェックリスト

- [ ] `cargo fmt --all -- --check` / `cargo clippy --workspace --all-targets -- -D warnings` / `cargo test --workspace`
- [ ] タスク完了に伴い `TASKS.MD` の DoD 欄（検証コマンド・失敗時挙動）を更新した（該当時）
- [ ] **unsafe / FFI を触れた場合** `crates/engine_core/SAFETY.md` のレビュー項目を確認した
- [ ] 公開 API の破壊的変更がある場合、意図と移行メモを書いた

## 検証証跡（該当時・Phase 1/2 readiness）

<!-- 縦スライスと本番要件を分けて記載。`#[ignore]` のみでは完了扱いにしない。 -->

| 区分 | 実行したコマンド / テスト名 | 結果（パスした日付・ログの場所） |
|------|----------------------------|----------------------------------|
| 縦スライス | 例: `cargo test -p engine_core --test audio_command_drain` | |
| 本番要件 | 例: nightly `cargo test -p engine_core --features audio-kira -- --ignored` | |

## Unsafe / FFI（該当時のみ）

- 触ったファイル:
- `// SAFETY:` コメントとスレッド前提:
- 新規 `unsafe impl Send/Sync` の有無と根拠:
