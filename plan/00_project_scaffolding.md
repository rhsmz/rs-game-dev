# 00: Project Scaffolding — Cargo Workspace 初期構築

## 概要
Cargo Workspace を構築し、3つの主要クレート（`engine_core`, `script_editor`, `game_player`）の骨組みを作成する。

## 前提条件
- Rust toolchain (stable) インストール済み

## タスク

### 1. ルート Cargo.toml 作成
```toml
[workspace]
members = [
    "crates/engine_core",
    "crates/script_editor",
    "crates/game_player",
]
resolver = "2"

[workspace.package]
version = "0.1.0"
edition = "2024"
license = "MIT"
rust-version = "1.85"

[workspace.dependencies]
serde = { version = "1", features = ["derive"] }
rmp-serde = "1"
anyhow = "1"
thiserror = "2"
log = "0.4"
env_logger = "0.11"
```

### 2. engine_core クレート作成
```bash
cargo init --lib crates/engine_core
```

`crates/engine_core/src/lib.rs` にモジュール宣言の骨組みを作成:
```rust
//! rs-game-dev Engine Core
//!
//! ゲームエンジンのコアロジック: ECS、レンダリング、サウンド、アセット管理、スクリプト解析

pub mod ecs;
pub mod renderer;
pub mod audio;
pub mod asset;
pub mod script;
pub mod scene;
pub mod input;
pub mod save;
pub mod rng;

pub(crate) mod ffi;
pub(crate) mod util;
```

各サブモジュール用のディレクトリと `mod.rs` を作成（最初は空 or 最小限のスタブ）:
- `src/ecs/mod.rs`
- `src/renderer/mod.rs`
- `src/audio/mod.rs`
- `src/asset/mod.rs`
- `src/script/mod.rs`
- `src/scene/mod.rs`
- `src/input/mod.rs`
- `src/save/mod.rs`
- `src/rng/mod.rs`
- `src/ffi/mod.rs`
- `src/util/mod.rs`

### 3. script_editor クレート作成
```bash
cargo init --lib crates/script_editor
```

`crates/script_editor/src/lib.rs`:
```rust
//! rs-game-dev Script Editor
//!
//! スクリプト編集、ライブプレビュー、ビジュアル編集ツール

pub mod ui;
pub mod preview;
pub mod inspector;
pub mod project;
```

### 4. game_player クレート作成
```bash
cargo init crates/game_player
```

`crates/game_player/src/main.rs`:
```rust
//! rs-game-dev Game Player
//!
//! デバッグ機能を省いた配布用軽量ランタイム

fn main() {
    env_logger::init();
    log::info!("Game Player starting...");
    // TODO: メインループ実装
}
```

### 5. Clippy / Lint 設定
ルート `Cargo.toml` に追加:
```toml
[workspace.lints.clippy]
pedantic = { level = "warn", priority = -1 }
nursery = { level = "warn", priority = -1 }
unwrap_used = "deny"
expect_used = "warn"
```

### 6. rustfmt 設定
`rustfmt.toml` を作成:
```toml
edition = "2024"
max_width = 100
use_field_init_shorthand = true
```

### 7. .gitignore 設定
```
/target
*.swp
*.swo
.DS_Store
```

### 8. ビルド検証
```bash
cargo build --workspace
cargo test --workspace
cargo clippy --workspace -- -D warnings
```

## 成果物
- `Cargo.toml` (ルート)
- `crates/engine_core/` (lib クレート + 11 サブモジュール骨組み)
- `crates/script_editor/` (lib クレート + 4 サブモジュール骨組み)
- `crates/game_player/` (bin クレート)
- `rustfmt.toml`
- `.gitignore`

## 完了条件
- `cargo build --workspace` が成功
- `cargo clippy --workspace -- -D warnings` が警告なし
