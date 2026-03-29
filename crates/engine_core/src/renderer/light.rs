//! IBL / ライティング（雛形）。
//!
//! 現時点では Filament 統合を行わず、必要な型と API 面だけを先に用意します。

use anyhow::anyhow;

use crate::renderer::RenderEngine;

/// HDR 環境マップ（雛形）。
pub struct HdrEnvironmentMap {
    pub path: String,
}

/// IBL 設定（雛形）。
#[derive(Debug, Clone, Copy)]
pub struct IblSettings {
    pub intensity: f32,
}

/// 方向光（雛形）。
#[derive(Debug, Clone, Copy)]
pub struct DirectionalLight {
    pub direction: [f32; 3],
    pub color: [f32; 3],
    pub intensity: f32,
}

/// SkyBox（雛形）。
pub struct SkyBox {
    pub environment_map: Option<HdrEnvironmentMap>,
    pub settings: IblSettings,
}

impl HdrEnvironmentMap {
    /// HDR 環境マップを読み込む（雛形）。
    #[allow(clippy::missing_errors_doc)]
    pub fn load_from_path(path: impl Into<String>) -> anyhow::Result<Self> {
        let path = path.into();
        // TODO: Filament の HDR ロード / IBL 生成に置き換える。
        if path.is_empty() {
            return Err(anyhow!("hdr environment map path is empty"));
        }

        Ok(Self { path })
    }
}

#[allow(dead_code)] // `apply_to` は Phase 2 のレンダラ統合まで未配線（API 面のみ先行）。
impl SkyBox {
    /// `SkyBox` を生成する（雛形）。
    #[must_use]
    #[allow(clippy::missing_const_for_fn)]
    pub fn new(settings: IblSettings) -> Self {
        Self { environment_map: None, settings }
    }

    /// 環境マップを設定する（雛形）。
    pub fn set_environment_map(&mut self, map: HdrEnvironmentMap) {
        self.environment_map = Some(map);
    }

    /// Filament 側へ反映する（crate 内・足場専用）。
    ///
    /// 公開型は先行しているが、Filament 接続は Phase 2 以降。外部クレートからは呼ばず、
    /// 将来 `engine_core` 内のシステムからのみ利用する想定。
    #[allow(clippy::missing_errors_doc)]
    #[allow(clippy::unnecessary_wraps)] // 将来 Filament 連携時にエラー返却する足場 API
    pub(crate) fn apply_to(&self, engine: &RenderEngine) -> anyhow::Result<()> {
        let _ = engine;
        log::trace!("SkyBox.apply_to (scaffold): intensity={}", self.settings.intensity);
        Ok(())
    }
}
