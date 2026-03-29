//! Render-to-Texture（雛形）。
//!
//! `Live2D` 合成用のオフスクリーンターゲットを想定する。

use crate::renderer::RenderEngine;

/// オフスクリーンレンダターゲット（雛形）。
#[derive(Debug, Clone, Copy)]
pub struct RenderTarget {
    pub width: u32,
    pub height: u32,
}

#[allow(dead_code)] // `apply_for_live2d` は Phase 2 の RTT 連携まで未配線。
impl RenderTarget {
    /// 新しい `RenderTarget` を作成する（雛形）。
    #[must_use]
    #[allow(clippy::missing_const_for_fn)]
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    /// サイズ変更（雛形）。
    #[allow(clippy::missing_const_for_fn)]
    pub fn resize(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
    }

    /// `Live2D` 合成に使うための反映（crate 内・足場専用）。
    #[allow(clippy::missing_errors_doc)]
    #[allow(clippy::unnecessary_wraps)]
    pub(crate) fn apply_for_live2d(self, engine: &RenderEngine) -> anyhow::Result<()> {
        let _ = engine;
        log::trace!("RenderTarget.apply_for_live2d (scaffold): {}x{}", self.width, self.height);
        Ok(())
    }
}

impl Default for RenderTarget {
    fn default() -> Self {
        Self { width: 1, height: 1 }
    }
}
