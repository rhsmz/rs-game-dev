## 概要

フェイズ1（ワークスペース初期化）の p1a として、ルートの `Cargo.toml` とワークスペースメンバー（engine_core, script_editor, game_player）を定義した。既存の develop には同内容が含まれているため、本 PR は履歴整理・計画との対応付け用です。

## 変更内容

- [x] ルート `Cargo.toml` に `[workspace]` と `members` を定義
- [x] `resolver = "2"`、`[workspace.package]`（version, edition, license, rust-version）を設定
- [x] `[workspace.dependencies]` で共通依存（serde, rmp-serde, anyhow, thiserror, log, env_logger）を定義
- [x] `[workspace.lints.clippy]` で Clippy の共通リント設定を定義

## テスト方法

```bash
cargo check --workspace
```

## 関連 Issue

<!-- 該当する Issue があれば Closes #<番号> -->
