//! Renderer System（雛形）。

use crate::ecs::world::World;
use crate::renderer::{GameView, RenderEngine, UiView};

#[cfg(feature = "filament")]
use crate::ecs::component::ComponentStorage;
#[cfg(feature = "filament")]
use crate::renderer::{Camera3D, MeshRenderer};

/// レンダリング処理。
///
/// `MeshRenderer` / `Camera3D` を走査し、将来的に Filament へメッシュを投入する。
/// 現状は **Game View → UI View** の順で `Renderer_render` を呼ぶ（マルチビュー順序の検証用）。
///
/// # 呼び出し契約
/// - [`RenderEngine::begin_frame`] が `true` を返したフレームでのみ呼ぶこと。
/// - 終了後に [`RenderEngine::end_frame`] を呼ぶこと。
///
/// # カメラ欠如時
/// カメラが 1 体も無い場合はメッシュ系の投入をスキップし、ビューのみ描画する（フェイルファストしない）。
pub fn render_system(world: &World, engine: &mut RenderEngine, game: &GameView, ui: &UiView) {
    #[cfg(feature = "filament")]
    {
        let cam_count = world.get_storage::<Camera3D>().map_or(0, ComponentStorage::len);
        let mesh_count = world.get_storage::<MeshRenderer>().map_or(0, ComponentStorage::len);

        if cam_count == 0 {
            log::debug!(
                "render_system: no Camera3D entities; skipping mesh submission (views still rendered)"
            );
        } else {
            log::trace!("render_system: mesh_entities={mesh_count} camera_entities={cam_count}");
        }

        engine.render_filament_view(Some(game.filament_view_ptr()));
        engine.render_filament_view(Some(ui.filament_view_ptr()));
    }

    #[cfg(not(feature = "filament"))]
    {
        let _ = (world, engine, game, ui);
    }
}
