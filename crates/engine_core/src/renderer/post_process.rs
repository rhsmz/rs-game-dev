//! ポストプロセス（雛形）。
//!
//! `Bloom` / `Tone Mapping` / `Gaussian Blur` を後段で適用する想定。

use crate::renderer::RenderEngine;

/// ポストプロセス設定（雛形）。
#[derive(Debug, Clone, Copy)]
pub struct PostProcessSettings {
    pub bloom_enabled: bool,
    pub bloom_strength: f32,
    pub tone_mapping_enabled: bool,
    pub gaussian_blur_enabled: bool,
    pub gaussian_blur_sigma: f32,
}

impl PostProcessSettings {
    // v0.1 では `Default` 実装のみ提供し、詳細は後続タスクで拡張する。
}

impl Default for PostProcessSettings {
    fn default() -> Self {
        Self {
            bloom_enabled: false,
            bloom_strength: 1.0,
            tone_mapping_enabled: true,
            gaussian_blur_enabled: false,
            gaussian_blur_sigma: 8.0,
        }
    }
}

/// ポストプロセスパイプライン（雛形）。
pub struct PostProcessPipeline {
    pub settings: PostProcessSettings,
}

#[allow(dead_code)] // `apply_to` は Phase 2 のポストプロセス連携まで未配線。
impl PostProcessPipeline {
    /// パイプラインを生成する（雛形）。
    #[must_use]
    #[allow(clippy::missing_const_for_fn)]
    pub fn new(settings: PostProcessSettings) -> Self {
        Self { settings }
    }

    /// Filament 側へ反映する（crate 内・足場専用）。
    #[allow(clippy::missing_errors_doc)]
    #[allow(clippy::unnecessary_wraps)]
    pub(crate) fn apply_to(&self, engine: &RenderEngine) -> anyhow::Result<()> {
        let _ = engine;
        log::trace!(
            "PostProcessPipeline.apply_to (scaffold): bloom={} tone_map={}",
            self.settings.bloom_enabled,
            self.settings.tone_mapping_enabled
        );
        Ok(())
    }
}

impl Default for PostProcessPipeline {
    fn default() -> Self {
        Self::new(PostProcessSettings::default())
    }
}
