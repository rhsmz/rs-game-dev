//! Renderer System（雛形）。
//!
//! # 呼び出し側の契約（境界）
//! - **スレッド**: メインスレッド（または Filament を所有するスレッド）から呼ぶこと。
//!   [`GameView`] / [`UiView`] の `Send`/`Sync` 前提は [`SAFETY.md`](../../SAFETY.md) 参照。
//! - **前提**: [`RenderEngine::begin_frame`] が `true` のフレーム内で呼ぶ。`false` のフレームでは呼ばない。
//! - **終了**: 呼び出し後に必ず [`RenderEngine::end_frame`] を実行する。
//!
//! # 失敗・縮退時の動作（フェイルファストしない経路）
//! - `Camera3D` が 0 体: メッシュ提出は行わず、Game/UI ビューの `Renderer_render` のみ実行する（`debug` ログ）。
//! - [`crate::renderer::MeshRenderer::renderable_id`] が 0: **warn** し当該エンティティはスキップ（未ロード扱い）。
//! - Filament 縦スライス FFI が `false`: **warn** し当該メッシュのみスキップ（他メッシュ・ビュー描画は継続）。

use crate::ecs::world::World;
use crate::renderer::{GameView, RenderEngine, UiView};

#[cfg(feature = "filament")]
use crate::ecs::component::ComponentStorage;
#[cfg(feature = "filament")]
use crate::ecs::entity::Entity;
#[cfg(feature = "filament")]
use crate::ffi::filament_sys;
#[cfg(feature = "filament")]
use crate::renderer::mesh_submit_trace;
#[cfg(feature = "filament")]
use crate::renderer::render_trace;
#[cfg(feature = "filament")]
use crate::renderer::{Camera3D, MeshRenderer};

/// レンダリング処理。
///
/// `MeshRenderer` / `Camera3D` を走査し、Game View の `Scene` へ Filament 縦スライス投入を行う。
/// **Game View → UI View** の順で `Renderer_render` を呼ぶ。
///
/// # 呼び出し契約
/// - [`RenderEngine::begin_frame`] が `true` を返したフレームでのみ呼ぶこと。
/// - 終了後に [`RenderEngine::end_frame`] を呼ぶこと。
///
/// # カメラ欠如時
/// カメラが 1 体も無い場合はメッシュ系の投入をスキップし、ビューのみ描画する（フェイルファストしない）。
#[allow(clippy::missing_const_for_fn)]
pub fn render_system(
    world: &World,
    engine: &mut RenderEngine,
    game: &mut GameView,
    ui: &mut UiView,
) {
    #[cfg(feature = "filament")]
    {
        game.sync_framebuffer_from_engine(engine);
        ui.sync_framebuffer_from_engine(engine);

        let cam_count = world.get_storage::<Camera3D>().map_or(0, ComponentStorage::len);
        let mesh_count = world.get_storage::<MeshRenderer>().map_or(0, ComponentStorage::len);

        if cam_count == 0 {
            log::debug!(
                target: "engine_core::renderer",
                "render_system: no Camera3D entities; skipping mesh submission (views still rendered)"
            );
        } else {
            if cam_count > 1 {
                let first_cam =
                    world.get_storage::<Camera3D>().and_then(|s| s.iter().next()).map(|(e, _)| e);
                log::debug!(
                    target: "engine_core::renderer",
                    "render_system: {cam_count} Camera3D entities; using first in storage order only (entity={first_cam:?})"
                );
            }

            let engine_ptr = engine.filament_engine().as_ptr();
            let scene_ptr = game.filament_scene_ptr().as_ptr();

            if let Some(mesh_storage) = world.get_storage::<MeshRenderer>() {
                for (entity, mesh) in mesh_storage.iter() {
                    submit_one_mesh(entity, engine_ptr, scene_ptr, mesh);
                }
            }

            if mesh_count > 0 {
                log::info!(
                    target: "engine_core::renderer",
                    "render_system: vertical_slice frame — mesh_entities={mesh_count} camera_entities={cam_count} (GameView scene submit + GameView then UiView render order)"
                );
            } else {
                log::trace!(
                    target: "engine_core::renderer",
                    "render_system: mesh_entities=0 camera_entities={cam_count}"
                );
            }
        }

        // P0-1: Game View を先に、UI View を後に描画（アーキテクチャの Z オーダー方針に一致）。
        render_trace::record_render_pass("game_view");
        engine.render_filament_view(Some(game.filament_view_ptr()));
        render_trace::record_render_pass("ui_view");
        engine.render_filament_view(Some(ui.filament_view_ptr()));
    }

    #[cfg(not(feature = "filament"))]
    {
        let _ = (world, engine, game, ui);
    }
}

#[cfg(feature = "filament")]
fn submit_one_mesh(
    entity: Entity,
    engine_ptr: *mut filament_sys::Engine,
    scene_ptr: *mut filament_sys::Scene,
    mesh: &MeshRenderer,
) {
    if mesh.renderable_id == 0 {
        mesh_submit_trace::record_skipped_unloaded_mesh();
        log::warn!(
            target: "engine_core::renderer",
            "mesh submit skipped: renderable_id=0 (unloaded); entity={entity}",
        );
        return;
    }

    // SAFETY: `engine_ptr` / `scene_ptr` は `RenderEngine` / `GameView` の寿命内で有効。
    let ok = unsafe {
        filament_sys::Scene_submit_mesh_vertical_slice(engine_ptr, scene_ptr, mesh.renderable_id)
    };
    mesh_submit_trace::record_attach_result(ok);
    if !ok {
        log::warn!(
            target: "engine_core::renderer",
            "mesh submit failed: renderable_id={} entity={entity}",
            mesh.renderable_id,
        );
    }
}
