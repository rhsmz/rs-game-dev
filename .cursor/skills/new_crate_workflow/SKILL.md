---
name: new_crate_workflow
description: Cargo Workspace に新しいクレートを追加する手順を案内する。`cargo init --lib` でクレートを作成し、ルート `Cargo.toml` に `members` を追加してビルド/テスト/ドキュメントまで行いたいときに使用する。
---

# 新クレート追加ワークフロー

## 手順

### 1. クレートディレクトリ作成
```bash
cargo init --lib crates/{crate_name}
```

### 2. Workspace に登録
ルート `Cargo.toml` の `members` に新クレートを追加する。

```toml
[workspace]
members = [
    "crates/engine_core",
    "crates/script_editor",
    "crates/game_player",
    "crates/{crate_name}",  # 追加
]
```

### 3. 依存関係の設定
新クレートの `Cargo.toml` で workspace 共通依存を利用する。

```toml
[dependencies]
serde = { workspace = true }
anyhow = { workspace = true }
engine_core = { path = "../engine_core" }
```

### 4. 依存方向の確認
以下のルールに違反しないことを確認:
- `engine_core` は他の内部クレートに依存しない
- `script_editor` と `game_player` は互いに依存しない
- 循環依存は厳禁

### 5. ビルド検証
```bash
cargo build --workspace
```

### 6. テスト検証
```bash
cargo test -p {crate_name}
```

### 7. ドキュメント
- 新クレートの `lib.rs` にモジュールレベルの `//!` doc コメントを記述
- README.md のクレート構成セクションに説明を追記
- `GEMINI.md` の Crate Structure セクションに追記

## チェックリスト
- [ ] `crates/{name}/` 作成済み
- [ ] ルート `Cargo.toml` に登録済み
- [ ] 依存方向ルール遵守
- [ ] `cargo build --workspace` 成功
- [ ] `cargo test -p {name}` 成功
- [ ] ドキュメント更新済み
