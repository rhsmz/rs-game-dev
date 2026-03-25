//! Filament 初期化の smoke test（feature-gate）。
//!
//! 現段階では実 Filament バイナリが無い前提で、`filament` feature
//! 有効時にコンパイルされる雛形として用意する。

#[cfg(all(test, feature = "filament"))]
mod tests {
    use crate::renderer::RenderEngine;
    use raw_window_handle::HasRawWindowHandle;

    use winit::event_loop::EventLoop;
    use winit::window::WindowBuilder;

    #[test]
    fn test_filament_engine_initialization_smoke() -> anyhow::Result<()> {
        let event_loop = EventLoop::builder().build()?;

        let window = WindowBuilder::new()
            .with_title("rs-game-dev filament smoke test")
            .build(&event_loop)?;

        let raw_handle = window.raw_window_handle();
        let mut engine = RenderEngine::new(raw_handle)?;

        // 初期化直後の基本フローだけを呼ぶ。
        let _ = engine.begin_frame();
        engine.end_frame();

        Ok(())
    }
}
