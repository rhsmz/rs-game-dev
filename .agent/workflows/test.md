---
description: テスト・品質チェックの実行手順
---

# テストワークフロー

## 手順

### 1. フォーマットチェック

```bash
// turbo
cargo fmt --all -- --check
```

フォーマットが崩れている場合は自動修正:

```bash
cargo fmt --all
```

### 2. Clippy (静的解析)

```bash
// turbo
cargo clippy --workspace --all-targets -- -D warnings
```

### 3. ユニットテスト

```bash
// turbo
cargo test --workspace
```

### 4. 特定クレートのテスト

```bash
cargo test -p engine_core
cargo test -p script_editor
cargo test -p game_player
```

### 5. ドキュメンテーションテスト

```bash
// turbo
cargo test --workspace --doc
```

### 6. 統合テスト

```bash
cargo test --workspace --test '*'
```

### 7. 全チェック一括実行

```bash
cargo fmt --all -- --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace
```

## テスト記述ルール

- テスト関数名: `test_<対象>_<条件>_<期待結果>`
- ユニットテスト: 同一ファイルの `#[cfg(test)] mod tests` 内に記述
- 統合テスト: `tests/` ディレクトリに配置
- ECS 関連テストはモック World を作成して実行
- FFI 境界のテストは `#[ignore]` アトリビュートで通常実行から除外可能に

## CI 推奨構成

上記手順 1〜5 を CI パイプラインに組み込む。
