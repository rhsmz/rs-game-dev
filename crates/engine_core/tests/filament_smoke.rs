//! `filament` feature 有効時の 1 フレーム垂直スライス smoke（別プロセスで実行される統合テスト）。
//!
//! `winit` の `EventLoop` はプロセスあたり 1 回のみのため、ユニットテスト群と同居させない。

use engine_core::ecs::world::World;
use engine_core::renderer::{
    Camera3D, GameView, MeshRenderer, RenderEngine, UiView, render_system,
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
            let handle = window.window_handle()?.as_raw();

            let mut engine = RenderEngine::new(handle)?;
            let game_view = GameView::new(&engine)?;
            let ui_view = UiView::new(&engine)?;

            let mut world = World::new();
            let cam = world.spawn();
            world.insert_component(
                cam,
                Camera3D { fov: 60.0_f32.to_radians(), near: 0.1, far: 100.0 },
            );
            let mesh = world.spawn();
            world.insert_component(mesh, MeshRenderer { renderable_id: 1 });

            assert!(engine.begin_frame(), "begin_frame must succeed (dev stub or real Filament)");
            render_system(&world, &mut engine, &game_view, &ui_view);
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
    let event_loop = build_event_loop()?;
    let mut app = FilamentOneFrameApp { window: None, test_result: Ok(()) };
    event_loop.run_app(&mut app)?;
    app.test_result
}
