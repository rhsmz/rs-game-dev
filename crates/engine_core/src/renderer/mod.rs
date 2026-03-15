//! `Filament` PBR レンダラー統合モジュール。
//!
//! マルチビュー・レンダリング (Game View + UI View)、
//! IBL ライティング、ポストプロセスを提供する。

pub struct RendererManager;

impl RendererManager {
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}

impl Default for RendererManager {
    fn default() -> Self {
        Self::new()
    }
}
