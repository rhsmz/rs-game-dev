---
description: プロジェクトのビルド手順
---

# ビルドワークフロー

## 前提条件

- Rust toolchain (stable) がインストール済み
- Filament SDK がビルド済みでパスが設定済み
- Live2D Cubism Native SDK がダウンロード済み

## 手順

### 1. 依存関係の確認

```bash
rustup show
cargo --version
```

### 2. ワークスペース全体のビルド (Debug)

```bash
// turbo
cargo build --workspace
```

### 3. 特定クレートのみビルド

```bash
cargo build -p engine_core
cargo build -p script_editor
cargo build -p game_player
```

### 4. リリースビルド

```bash
cargo build --workspace --release
```

### 5. FFI ライブラリのリンク設定

C/C++ ライブラリ（Filament, Live2D SDK）のリンクは `build.rs` で設定する。

```rust
// crates/engine_core/build.rs
fn main() {
    // Filament
    println!("cargo:rustc-link-search=native={}", env!("FILAMENT_LIB_DIR"));
    println!("cargo:rustc-link-lib=static=filament");

    // Live2D Cubism
    println!("cargo:rustc-link-search=native={}", env!("CUBISM_LIB_DIR"));
    println!("cargo:rustc-link-lib=static=Live2DCubismCore");
}
```

### 6. 環境変数

| 変数名 | 説明 |
|--------|------|
| `FILAMENT_LIB_DIR` | Filament ライブラリディレクトリ |
| `CUBISM_LIB_DIR` | Live2D Cubism SDK ライブラリディレクトリ |

## トラブルシューティング

- リンクエラーが出る場合、SDK のビルド構成（Debug/Release）とターゲット（x64）が一致しているか確認
- Windows の場合、MSVC ツールチェーンを使用すること
