---
description: リリースビルドおよび配布パッケージの作成手順
---

# リリースワークフロー

## 前提条件

- 全テストがパスしていること（`/test` ワークフロー参照）
- バージョン番号が更新済みであること

## 手順

### 1. バージョン更新

各クレートの `Cargo.toml` のバージョンを更新する。セマンティックバージョニングに従う。

```toml
[package]
version = "X.Y.Z"
```

### 2. 全テスト実行

```bash
// turbo
cargo fmt --all -- --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace
```

### 3. リリースビルド

```bash
cargo build --workspace --release
```

### 4. game_player の配布バイナリ確認

```bash
// turbo
ls target/release/game_player*
```

リリースバイナリは `target/release/` に生成される。

### 5. アセットアーカイブ作成

配布用に `assets/` ディレクトリをアーカイブ化する。

```bash
# アーカイブ形式はプロジェクトのアセットアーカイバーで作成
cargo run -p game_player --release -- --pack-assets ./assets -o ./dist/assets.pak
```

### 6. 配布パッケージ構成

```text
dist/
├── game_player.exe    # (Windows) メインバイナリ
├── assets.pak         # アセットアーカイブ
├── config.toml        # ランタイム設定
└── README.txt         # ユーザー向け説明
```

### 7. Git タグ作成

```bash
git tag -a v{VERSION} -m "Release v{VERSION}"
git push origin v{VERSION}
```

## バージョニングルール

- **MAJOR**: 破壊的変更（セーブデータ互換性の喪失など）
- **MINOR**: 新機能追加（後方互換）
- **PATCH**: バグ修正
