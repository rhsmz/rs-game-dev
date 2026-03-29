//! `Filament` PBR レンダラー統合モジュール。
//!
//! マルチビュー・レンダリング (Game View + UI View)、
//! IBL ライティング、ポストプロセスを提供する。

mod engine;
pub use engine::RenderEngine;

mod view;
pub use view::{Camera3D, GameView, UiView};

mod light;
pub use light::{DirectionalLight, HdrEnvironmentMap, IblSettings, SkyBox};

mod sprite;
pub use sprite::{Sprite, SpriteRenderer, Z_BACKGROUND, Z_SPRITE_DEFAULT};

mod render_target;
pub use render_target::RenderTarget;

mod post_process;
pub use post_process::{PostProcessPipeline, PostProcessSettings};

mod components;
pub use components::MeshRenderer;

mod render_system;
pub use render_system::render_system;

#[cfg(feature = "filament")]
mod render_trace;
#[cfg(feature = "filament")]
pub use render_trace::{reset_render_pass_trace, take_render_pass_trace};
