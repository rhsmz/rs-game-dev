//! サウンドシステムモジュール。
//!
//! BGM / SE / Voice のミキシングとクロスフェード制御。
//! 音声 RMS 解析によるリップシンク連携を提供する。
//!
//! バックエンド: `kira` or `rodio`

pub struct AudioManager;

impl Default for AudioManager {
    fn default() -> Self {
        Self::new()
    }
}

impl AudioManager {
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}
