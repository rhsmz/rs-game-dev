//! `filament` feature 有効時の 1 フレーム垂直スライス smoke（別プロセスで実行される統合テスト）。
//!
//! `winit` の `EventLoop` はプロセスあたり 1 回のみのため、ユニットテスト群と同居させない。
//!
//! Windows で公式プリビルトを `FILAMENT_LIB_DIR` に指定する場合、デバッグ CRT（`mdd`）と
//! `cargo test` デバッグプロファイルの組み合わせでリンクが失敗することがある。実バイナリ検証は
//! `lib/x86_64/md` と `cargo test -p engine_core --features filament --test filament_smoke --release` を推奨。
//!
//! ## Phase 2 readiness（P0-1）との対応
//! - 1 フレーム: `begin_frame` → `render_system`（GameView → UiView の順）→ `end_frame`
//! - スワップチェーンリサイズ: ウィンドウの `inner_size` を `RenderEngine::resize` に渡す
//! - `ENGINE_CORE_RENDER_TRACE=1` 時は描画パス順（game_view → ui_view）をバッファに記録し、テストで検証する
//! - ECS: `Camera3D` + `MeshRenderer` を投入し、メッシュ幾何の Filament バインドは Phase 2 以降（現状はビュー描画とログで縦切りを検証）

use engine_core::ecs::world::World;
use engine_core::renderer::{
    Camera3D, GameView, MeshRenderer, RenderEngine, UiView, render_system, reset_render_pass_trace,
    take_render_pass_trace,
};
use raw_window_handle::HasWindowHandle;
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::window::{Window, WindowAttributes, WindowId};

struct FilamentOneFrameApp {
    window: Option<Window>,
    test_result: anyhow::Result<()>,
}

impl ApplicationHandler for FilamentOneFrameApp {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        self.test_result = (|| -> anyhow::Result<()> {
            let attrs = WindowAttributes::default().with_title("engine_core filament smoke");
            let window = event_loop.create_window(attrs)?;
            let size = window.inner_size();
            let handle = window.window_handle()?.as_raw();

            let mut engine = RenderEngine::new(handle)?;
            engine.resize(size.width, size.height);
            let mut game_view = GameView::new(&engine)?;
            let mut ui_view = UiView::new(&engine)?;

            let mut world = World::new();
            let cam = world.spawn();
            world.insert_component(
                cam,
                Camera3D { fov: 60.0_f32.to_radians(), near: 0.1, far: 100.0 },
            );
            let mesh = world.spawn();
            world.insert_component(mesh, MeshRenderer { renderable_id: 1 });

            assert!(engine.begin_frame(), "begin_frame must succeed (dev stub or real Filament)");
            render_system(&world, &mut engine, &mut game_view, &mut ui_view);
            engine.end_frame();

            self.window = Some(window);
            Ok(())
        })();
        event_loop.exit();
    }

    fn window_event(&mut self, _event_loop: &ActiveEventLoop, _id: WindowId, _event: WindowEvent) {}
}

fn build_event_loop() -> anyhow::Result<EventLoop<()>> {
    let mut builder = EventLoop::builder();
    #[cfg(target_os = "windows")]
    {
        use winit::platform::windows::EventLoopBuilderExtWindows;
        builder.with_any_thread(true);
    }
    #[cfg(target_os = "linux")]
    {
        use winit::platform::x11::EventLoopBuilderExtX11;
        builder.with_any_thread(true);
    }
    Ok(builder.build()?)
}

#[test]
fn test_filament_one_frame_vertical_slice() -> anyhow::Result<()> {
    reset_render_pass_trace();
    // SAFETY: テストは単一スレッドで、他コードと `ENGINE_CORE_RENDER_TRACE` を共有しない。
    unsafe {
        std::env::set_var("ENGINE_CORE_RENDER_TRACE", "1");
    }
    let event_loop = build_event_loop()?;
    let mut app = FilamentOneFrameApp { window: None, test_result: Ok(()) };
    event_loop.run_app(&mut app)?;
    let trace = take_render_pass_trace();
    unsafe {
        std::env::remove_var("ENGINE_CORE_RENDER_TRACE");
    }
    app.test_result?;
    assert_eq!(
        trace,
        vec!["game_view".to_string(), "ui_view".to_string()],
        "GameView を先に、UiView を後に `Renderer_render` する（P0-1 描画順保証）"
    );
    Ok(())
}

/// 3D と UI が同一スワップチェーンに重なる際のピクセル一致検証は、ゴールデン画像ハーネス導入後に有効化する。
#[test]
#[ignore = "visual regression: golden image harness not wired yet (Game/UI overlap)"]
fn test_game_ui_overlap_visual_regression_placeholder() {}
