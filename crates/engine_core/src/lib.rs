//! # rs-game-dev Engine Core
//!
//! ゲームエンジンのコアロジック。
//!
//! - **ECS**: Entity Component System 基盤
//! - **Renderer**: Filament PBR レンダラー統合
//! - **Audio**: BGM/SE/Voice ミキシング
//! - **Asset**: 非同期ロード、キャッシュ、ローカライズ
//! - **Script**: タグベーススクリプト解析・実行
//! - **Scene**: シーン管理・遷移
//! - **Save**: `MessagePack` セーブ/ロード
//! - **RNG**: Dual RNG (演出用 + ロジック用)
//! - **Logging**: バイナリ向け `env_logger` 初期化（コンソール + ファイル）

pub mod asset;
pub mod audio;
pub mod ecs;
pub mod input;
pub mod logging;
pub mod renderer;
pub mod rng;
pub mod save;
pub mod scene;
pub mod script;

pub(crate) mod ffi;
pub(crate) mod util;
