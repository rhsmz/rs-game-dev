//! 2D スプライト描画（雛形）。
//!
//! Filament 側では「3D 空間内の Quad に 2D テクスチャを貼る」想定。
//! 現時点ではデータ構造と API 面のみを先に用意する。

use crate::renderer::RenderEngine;

/// 背景レイヤーの Z（背景は常に最背面扱い）。
pub const Z_BACKGROUND: f32 = 10.0;

/// デフォルトのスプライト Z（実装側の都合で調整）。
pub const Z_SPRITE_DEFAULT: f32 = 0.0;

/// 2D スプライト（雛形）。
#[derive(Debug, Clone)]
pub struct Sprite {
    pub texture_path: String,
    pub z: f32,
    pub width: f32,
    pub height: f32,
    /// アンカー（0.0〜1.0, 左上が(0,0)の想定）。
    pub anchor: (f32, f32),
}

impl Sprite {
    /// スプライトを生成する（雛形）。
    #[allow(clippy::missing_const_for_fn)]
    pub fn new(texture_path: impl Into<String>, z: f32, width: f32, height: f32) -> Self {
        Self { texture_path: texture_path.into(), z, width, height, anchor: (0.5, 0.5) }
    }
}

/// スプライト描画の雛形レンダラ。
pub struct SpriteRenderer {
    sprites: Vec<Sprite>,
}

impl SpriteRenderer {
    /// 新しい `SpriteRenderer` を作成する。
    #[must_use]
    pub const fn new() -> Self {
        Self { sprites: Vec::new() }
    }

    /// スプライトを投入する。
    pub fn submit(&mut self, sprite: Sprite) {
        self.sprites.push(sprite);
    }

    /// 現在投入されているスプライトを描画する（足場段階）。
    ///
    /// Filament への Quad 投入は未実装。投入済みスプライトはクリアされ、エラーにはしない。
    #[allow(clippy::missing_errors_doc)]
    pub fn render(&mut self, engine: &mut RenderEngine) -> anyhow::Result<()> {
        let _ = engine;
        let count = self.sprites.len();
        self.sprites.clear();
        log::trace!("SpriteRenderer.render (scaffold): cleared {count} sprites");
        Ok(())
    }
}

impl Default for SpriteRenderer {
    fn default() -> Self {
        Self::new()
    }
}
