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

    /// 直近の `resize` に基づくスワップチェーン相当の描画解像度（`View` の投影・ビューポート同期用）。
    #[cfg(feature = "filament")]
    framebuffer_width: u32,

    #[cfg(feature = "filament")]
    framebuffer_height: u32,

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
                unsafe { filament_sys::Renderer_destroy(engine.as_ptr(), renderer.as_ptr()) };
                unsafe { filament_sys::Engine_destroy(engine.as_ptr()) };
                anyhow!("Filament SwapChain_create returned null")
            })?;

            log::debug!(
                target: "engine_core::renderer",
                "RenderEngine: Filament Engine, Renderer, SwapChain initialized (resize + viewport to be applied per frame)"
            );
            Ok(Self { engine, renderer, swap_chain, framebuffer_width: 4, framebuffer_height: 4 })
        }

        #[cfg(not(feature = "filament"))]
        {
            let _ = window_handle;
            Err(anyhow!("filament feature is disabled"))
        }
    }

    /// フレーム開始。`SwapChain` / `Renderer` に委譲する。
    ///
    /// `false` のときは描画をスキップし、呼び出し側は `render_system` を呼ばないこと。
    #[must_use]
    #[allow(clippy::missing_const_for_fn)] // `#[cfg(feature)]` 分岐で const 化できない
    pub fn begin_frame(&mut self) -> bool {
        #[cfg(feature = "filament")]
        {
            // SAFETY: `swap_chain` / `renderer` は `NonNull` かつ `RenderEngine` 寿命内。
            let ok = unsafe {
                filament_sys::Renderer_beginFrame(self.swap_chain.as_ptr(), self.renderer.as_ptr())
            };
            if !ok {
                log::warn!("RenderEngine::begin_frame returned false; skip rendering this frame");
            }
            ok
        }
        #[cfg(not(feature = "filament"))]
        {
            false
        }
    }

    /// フレーム終了。提出 / プレゼントに相当する処理を行う。
    #[allow(clippy::missing_const_for_fn)]
    pub fn end_frame(&mut self) {
        #[cfg(feature = "filament")]
        {
            // SAFETY: `renderer` は `NonNull` かつ `RenderEngine` 寿命内。
            unsafe {
                filament_sys::Renderer_endFrame(self.renderer.as_ptr());
            }
        }
    }

    /// スワップチェーンおよびビューポートのリサイズ。
    #[allow(clippy::missing_const_for_fn)]
    pub fn resize(&mut self, width: u32, height: u32) {
        #[cfg(feature = "filament")]
        {
            self.framebuffer_width = width.max(1);
            self.framebuffer_height = height.max(1);
            // SAFETY: `swap_chain` は `NonNull` かつ `RenderEngine` 寿命内。
            unsafe {
                filament_sys::SwapChain_resize(self.swap_chain.as_ptr(), width, height);
            }
            log::trace!("RenderEngine::resize {width}x{height}");
        }
        #[cfg(not(feature = "filament"))]
        {
            let _ = (width, height);
        }
    }

    /// 単一 `View` を描画する（[`Self::begin_frame`] 成功後、[`Self::end_frame`] 前に呼ぶ）。
    #[cfg(feature = "filament")]
    pub(crate) fn render_filament_view(&mut self, view: Option<NonNull<filament_sys::View>>) {
        let Some(view) = view else {
            return;
        };
        // SAFETY: `renderer` / `view` は有効な Filament ハンドル。
        unsafe {
            filament_sys::Renderer_render(self.renderer.as_ptr(), view.as_ptr());
        }
    }

    /// 内部の Filament `Engine` ポインタ（`View` / `Scene` 生成用）。
    #[cfg(feature = "filament")]
    pub(crate) const fn filament_engine(&self) -> NonNull<filament_sys::Engine> {
        self.engine
    }

    /// `ViewCamera_update_*` へ渡すフレームバッファサイズ（少なくとも 1x1）。
    #[cfg(feature = "filament")]
    #[must_use]
    pub(crate) const fn framebuffer_dimensions(&self) -> (u32, u32) {
        (self.framebuffer_width, self.framebuffer_height)
    }
}

impl Drop for RenderEngine {
    fn drop(&mut self) {
        #[cfg(feature = "filament")]
        {
            // SAFETY: すべて `NonNull` として保持しており、寿命は `RenderEngine` の所有者に一致する。
            //         破棄順は依存関係を考慮して SwapChain -> Renderer -> Engine とする。
            unsafe {
                filament_sys::SwapChain_destroy(self.engine.as_ptr(), self.swap_chain.as_ptr());
            }
            unsafe {
                filament_sys::Renderer_destroy(self.engine.as_ptr(), self.renderer.as_ptr());
            }
            unsafe { filament_sys::Engine_destroy(self.engine.as_ptr()) };
        }
    }
}
