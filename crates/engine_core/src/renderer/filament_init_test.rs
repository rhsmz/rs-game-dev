//! Filament 初期化の smoke test（feature-gate）。
//!
//! 現段階では実 Filament バイナリが無い前提で、`filament` feature
//! 有効時にコンパイルされる雛形として用意する。

#[cfg(all(test, feature = "filament"))]
mod tests {
    use crate::renderer::RenderEngine;
    use raw_window_handle::HasWindowHandle;
    use winit::application::ApplicationHandler;
    use winit::event::WindowEvent;
    use winit::event_loop::{ActiveEventLoop, EventLoop};
    use winit::window::{Window, WindowAttributes, WindowId};

    struct FilamentSmokeApp {
        window: Option<Window>,
        test_result: anyhow::Result<()>,
    }

    impl ApplicationHandler for FilamentSmokeApp {
        fn resumed(&mut self, event_loop: &ActiveEventLoop) {
            self.test_result = (|| -> anyhow::Result<()> {
                let attrs =
                    WindowAttributes::default().with_title("rs-game-dev filament smoke test");
                let window = event_loop.create_window(attrs)?;
                let handle = window.window_handle()?.as_raw();

                let mut engine = RenderEngine::new(handle)?;
                let _ = engine.begin_frame();
                engine.end_frame();

                self.window = Some(window);
                Ok(())
            })();
            event_loop.exit();
        }

        fn window_event(
            &mut self,
            _event_loop: &ActiveEventLoop,
            _id: WindowId,
            _event: WindowEvent,
        ) {
        }
    }

    #[test]
    fn test_filament_engine_initialization_smoke() -> anyhow::Result<()> {
        let event_loop = EventLoop::builder().build()?;
        let mut app = FilamentSmokeApp { window: None, test_result: Ok(()) };
        event_loop.run_app(&mut app)?;
        app.test_result
    }
}
