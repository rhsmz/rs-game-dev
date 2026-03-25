//! Renderer System（雛形）。

use crate::ecs::world::World;
use crate::renderer::RenderEngine;

/// レンダリング処理（雛形）。
///
/// 今後、`MeshRenderer` 等のコンポーネントを走査し、`RenderEngine` へ描画コマンドを発行する。
pub const fn render_system(_world: &World, _engine: &mut RenderEngine) {
    // TODO: Filament による実描画のためのクエリ/コマンドバッファ生成を実装する。
    // 現時点では雛形であり、処理は行わない。
}
