//! マルチビュー描画テスト（雛形）。
//!
//! GPU/Filament 実描画に依存するため、`filament` feature 有効時のみコンパイルされる
//! smoke test として用意します。

#[cfg(all(test, feature = "filament"))]
mod tests {
    use crate::renderer::{GameView, RenderEngine, UiView};
    use raw_window_handle::HasWindowHandle;
    use winit::application::ApplicationHandler;
    use winit::event::WindowEvent;
    use winit::event_loop::{ActiveEventLoop, EventLoop};
    use winit::window::{Window, WindowAttributes, WindowId};

    struct MultiViewSmokeApp {
        window: Option<Window>,
        test_result: anyhow::Result<()>,
    }

    impl ApplicationHandler for MultiViewSmokeApp {
        fn resumed(&mut self, event_loop: &ActiveEventLoop) {
            self.test_result = (|| -> anyhow::Result<()> {
                let attrs =
                    WindowAttributes::default().with_title("rs-game-dev multiview smoke test");
                let window = event_loop.create_window(attrs)?;
                let handle = window.window_handle()?.as_raw();

                let mut engine = RenderEngine::new(handle)?;
                let _game_view = GameView::new(&engine)?;
                let _ui_view = UiView::new(&engine)?;

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
    fn test_multiview_render_smoke_filament_enabled_ok() -> anyhow::Result<()> {
        let event_loop = EventLoop::builder().build()?;
        let mut app = MultiViewSmokeApp { window: None, test_result: Ok(()) };
        event_loop.run_app(&mut app)?;
        app.test_result
    }
}
