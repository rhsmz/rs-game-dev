//! `Filament` PBR レンダラー統合モジュール。
//!
//! マルチビュー・レンダリング (Game View + UI View)、
//! IBL ライティング、ポストプロセスを提供する。
//!
//! ## `MeshRenderer` と Filament `Renderable`（対応方針・P0-1 残）
//!
//! 現状の縦スライスでは ECS の [`MeshRenderer`] が **`renderable_id: u32`** のみ保持し、
//! [`render_system`] が FFI の `Scene_submit_mesh_vertical_slice` へ渡す（`ffi::filament_sys`）。
//! - **スタブ**（`FILAMENT_LIB_DIR` 未設定）: 幾何は作らず受理のみ。
//! - **ブリッジ**（公式プリビルトリンク時）: `Engine::getDefaultMaterial` + 三角形を `RenderableManager` で `Scene` に追加（ID ごとに 1 エンティティ、詳細は `stub/filament_bridge.cpp`）。
//!
//! Phase 2 以降で目指す対応表（設計メモ）:
//!
//! 1. **安定 ID**: アセットロード完了時に `RenderEngine` 側（または専用レジストリ）で `utils::Entity` +
//!    `RenderableManager` エントリを生成し、ECS には opaque な **`MeshHandle`（新タイプ）** または現 `u32` の意味を
//!    「レジストリインデックス」に固定する。
//! 2. **ライフサイクル**: エンティティデスポーン／アンロード時に `Scene` から detach → Filament 側 destroy を
//!    `Drop` または明示システムで対に実行する（`GameView` の `Scene` 寿命と整合）。
//! 3. **検証**: `render_system` は「ロード済み・レジストリに存在する」ハンドルのみ提出し、欠損は warn + skip（現状の `renderable_id==0` と同階層のポリシー）。
//!
//! 詳細な Readiness 記録はリポジトリの [`plan/15_phase2_readiness_plan.md`](../../../../plan/15_phase2_readiness_plan.md) を参照。

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

#[cfg(feature = "filament")]
mod mesh_submit_trace;
#[cfg(feature = "filament")]
pub use mesh_submit_trace::{reset_mesh_submit_trace, take_mesh_submit_trace};
