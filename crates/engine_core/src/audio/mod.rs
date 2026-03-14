//! サウンドシステムモジュール。
//!
//! BGM / SE / Voice のミキシングとクロスフェード制御。
//! 音声 RMS 解析によるリップシンク連携を提供する。
//!
//! バックエンド: `kira` or `rodio`

pub struct AudioManager;

impl AudioManager {
    pub fn new() -> Self {
        Self
    }
}
