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

pub mod asset;
pub mod audio;
pub mod ecs;
pub mod input;
pub mod renderer;
pub mod rng;
pub mod save;
pub mod scene;
pub mod script;

pub(crate) mod ffi;
pub(crate) mod util;

/// Dummy minimal EngineCore representation
pub struct EngineCore;

impl EngineCore {
    #[allow(clippy::new_without_default)]
    #[must_use]
    pub fn new() -> Self {
        Self
    }

    pub fn set_asset_source<T>(&mut self, _archive: Option<T>) {
        log::info!("Asset source set.");
    }

    pub fn push_scene<T>(&mut self, _scene: T) {
        log::info!("Scene pushed.");
    }

    pub fn process_input(&mut self) {
        // Mock input processing
    }

    pub fn update(&mut self, _dt: f32) {
        // Mock update logic
    }

    pub fn render(&mut self) {
        // Mock rendering logic
    }

    #[must_use]
    pub fn should_quit(&self) -> bool {
        // For testing/mocking, hardcode to true so it doesn't infinite loop.
        // In a real application, this would read from window events.
        true
    }
}

/// Dummy TitleScene representation
pub struct TitleScene;

impl TitleScene {
    #[allow(clippy::new_without_default)]
    #[must_use]
    pub fn new() -> Self {
        Self
    }
}
