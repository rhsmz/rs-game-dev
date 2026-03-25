//! `Filament` の Engine ラッパー（雛形）。

use anyhow::anyhow;
use raw_window_handle::RawWindowHandle;

#[cfg(feature = "filament")]
use crate::ffi::filament_sys;

#[cfg(feature = "filament")]
use std::ptr::NonNull;

/// Filament のレンダリングエンジンラッパー。
pub struct RenderEngine {
    #[cfg(feature = "filament")]
    engine: NonNull<filament_sys::Engine>,

    #[cfg(feature = "filament")]
    renderer: NonNull<filament_sys::Renderer>,

    #[cfg(feature = "filament")]
    swap_chain: NonNull<filament_sys::SwapChain>,

    // 現時点の雛形では、非 Filament 環境でもコンパイルできるようにダミー領域を持つ。
    #[cfg(not(feature = "filament"))]
    _private: (),
}

impl RenderEngine {
    /// レンダリングエンジンを初期化する。
    ///
    /// # Errors
    /// - `filament` feature が無効な場合。
    /// - Filament の初期化が失敗した場合（`*_create` が null を返した場合など）。
    #[allow(clippy::missing_errors_doc)]
    pub fn new(window_handle: RawWindowHandle) -> anyhow::Result<Self> {
        #[cfg(feature = "filament")]
        {
            let _ = window_handle;
            // SAFETY: ここで呼ぶ `*_create` は Filament が提供する FFI の
            //              オブジェクト生成関数であり、戻り値を null チェックして NonNull に変換する。
            let engine_ptr = unsafe { filament_sys::Engine_create() };
            let engine = NonNull::new(engine_ptr)
                .ok_or_else(|| anyhow!("Filament Engine_create returned null"))?;

            let renderer_ptr = unsafe { filament_sys::Renderer_create(engine.as_ptr()) };
            let renderer = NonNull::new(renderer_ptr).ok_or_else(|| {
                // SAFETY: `engine` は直前で生成済みなので、ここで破棄してリークを防ぐ。
                unsafe { filament_sys::Engine_destroy(engine.as_ptr()) };
                anyhow!("Filament Renderer_create returned null")
            })?;

            let swap_chain_ptr = unsafe { filament_sys::SwapChain_create(engine.as_ptr()) };
            let swap_chain = NonNull::new(swap_chain_ptr).ok_or_else(|| {
                // SAFETY: `renderer` と `engine` は生成済みなので、ここで破棄してリークを防ぐ。
                unsafe { filament_sys::Renderer_destroy(renderer.as_ptr()) };
                unsafe { filament_sys::Engine_destroy(engine.as_ptr()) };
                anyhow!("Filament SwapChain_create returned null")
            })?;

            Ok(Self { engine, renderer, swap_chain })
        }

        #[cfg(not(feature = "filament"))]
        {
            let _ = window_handle;
            Err(anyhow!("filament feature is disabled"))
        }
    }

    /// フレーム開始。現時点では雛形のため常に `false` を返す。
    #[must_use]
    #[allow(clippy::missing_const_for_fn)]
    pub fn begin_frame(&mut self) -> bool {
        false
    }

    /// フレーム終了（雛形）。
    #[allow(clippy::missing_const_for_fn)]
    pub fn end_frame(&mut self) {}

    /// リサイズ（雛形）。
    #[allow(clippy::missing_const_for_fn)]
    pub fn resize(&mut self, width: u32, height: u32) {
        let _ = (width, height, self);
    }
}

impl Drop for RenderEngine {
    fn drop(&mut self) {
        #[cfg(feature = "filament")]
        {
            // SAFETY: すべて `NonNull` として保持しており、寿命は `RenderEngine` の所有者に一致する。
            //         破棄順は依存関係を考慮して SwapChain -> Renderer -> Engine とする。
            unsafe { filament_sys::SwapChain_destroy(self.swap_chain.as_ptr()) };
            unsafe { filament_sys::Renderer_destroy(self.renderer.as_ptr()) };
            unsafe { filament_sys::Engine_destroy(self.engine.as_ptr()) };
        }
    }
}
