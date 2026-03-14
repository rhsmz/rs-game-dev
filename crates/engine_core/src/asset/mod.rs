//! アセット管理モジュール。
//!
//! 非同期ロード、メモリキャッシュ、
//! ローカライズルーティング (言語別フォールバック) を提供する。

#[derive(Default)]
pub struct AssetManager;

impl AssetManager {
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}
