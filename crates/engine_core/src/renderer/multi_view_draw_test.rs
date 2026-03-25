//! マルチビュー描画テスト（雛形）。
//!
//! GPU/Filament 実描画に依存するため、`filament` feature 有効時のみコンパイルされる
//! smoke test として用意します。

#[cfg(all(test, feature = "filament"))]
mod tests {
    use crate::renderer::{GameView, RenderEngine, UiView};
    use raw_window_handle::HasRawWindowHandle;
    use winit::event_loop::EventLoop;
    use winit::window::WindowBuilder;

    #[test]
    fn test_multiview_render_smoke_filament_enabled_ok() -> anyhow::Result<()> {
        let event_loop = EventLoop::builder().build()?;
        let window = WindowBuilder::new()
            .with_title("rs-game-dev multiview smoke test")
            .build(&event_loop)?;

        let raw_handle = window.raw_window_handle();
        let mut engine = RenderEngine::new(raw_handle)?;

        let _game_view = GameView::new(&engine)?;
        let _ui_view = UiView::new(&engine)?;

        // 現時点の `RenderEngine` は雛形で描画処理も未実装のため、
        // フレーム開始/終了の呼び出しまでを smoke test とする。
        let _ = engine.begin_frame();
        engine.end_frame();

        Ok(())
    }
}
