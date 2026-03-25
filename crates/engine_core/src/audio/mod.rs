//! サウンドシステムモジュール。
//!
//! BGM / SE / Voice のミキシングとクロスフェード制御。
//! 音声 RMS 解析によるリップシンク連携を提供する。
//!
//! バックエンド: `kira` or `rodio`

mod manager;
pub use manager::GameAudioManager;

mod lip_sync;
pub use lip_sync::calculate_lip_sync_value;

mod sound_meta;
pub use sound_meta::{
    SoundMeta, SoundType, deserialize_sound_meta_from_csv_reader, deserialize_sound_meta_from_str,
};

mod ecs_integration;
pub use ecs_integration::{AudioCommand, AudioSource, AudioTrack};

/// 互換のための別名（段階的移行用）。
pub type AudioManager = GameAudioManager;
