//! サウンドシステムモジュール。
//!
//! BGM / SE / Voice のミキシングとクロスフェード制御。
//! 音声 RMS 解析によるリップシンク連携を提供する。
//!
//! ## バックエンドと feature
//!
//! - **既定**: `audio-kira` が有効で [`GameAudioManager`] が `kira`（`cpal`）をリンクする。
//! - **Linux**: 実行時に ALSA 等が必要になることがある。ヘッドレス CI では `libasound2-dev` を入れるか、
//!   `cargo test -p engine_core --no-default-features` でスタブマネージャのみを検証する。
//! - **`--no-default-features`**: `engine_core` 単体では `kira` をリンクせずスタブ [`GameAudioManager`] でビルドする。
//!   他クレートが `engine_core` に `default-features = true`（既定）のまま依存すると、ワークスペース全体の feature 統合で `audio-kira` が付くことがある。CI では `cargo test -p engine_core --no-default-features` でスタブ経路を検証する。
//!
//! ファイルパスが存在しない再生コマンドは **warn ログのうえスキップ**する（I/O 層の仕様）。
//!
//! バックエンド切り替えの拡張: `kira` or `rodio`（将来）。

mod lip_sync;
pub use lip_sync::calculate_lip_sync_value;

mod sound_meta;
pub use sound_meta::{
    SoundMeta, SoundType, deserialize_sound_meta_from_csv_reader, deserialize_sound_meta_from_str,
};

mod ecs_integration;
pub use ecs_integration::{AudioCommand, AudioSource, AudioTrack};

mod command_queue;
pub use command_queue::AudioCommandQueue;

mod command_dispatch;
pub use command_dispatch::{audio_command_system, drain_audio_command_queue_with};

mod manager;
pub use manager::GameAudioManager;

/// 互換のための別名（段階的移行用）。
pub type AudioManager = GameAudioManager;
