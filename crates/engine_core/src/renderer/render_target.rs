//! Render-to-Texture（雛形）。
//!
//! `Live2D` 合成用のオフスクリーンターゲットを想定する。

use anyhow::anyhow;

use crate::renderer::RenderEngine;

/// オフスクリーンレンダターゲット（雛形）。
#[derive(Debug, Clone, Copy)]
pub struct RenderTarget {
    pub width: u32,
    pub height: u32,
}

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

    /// `Live2D` 合成に使うための反映（雛形）。
    #[allow(clippy::missing_errors_doc)]
    pub fn apply_for_live2d(&self, _engine: &RenderEngine) -> anyhow::Result<()> {
        Err(anyhow!("RenderTarget.apply_for_live2d is not implemented yet"))
    }
}

impl Default for RenderTarget {
    fn default() -> Self {
        Self { width: 1, height: 1 }
    }
}
