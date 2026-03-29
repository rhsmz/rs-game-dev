//! `audio-kira` 無効時のスタブ（出力バックエンドをリンク・初期化しない）。

#![allow(clippy::missing_const_for_fn)] // スタブは将来 kira 実装と API を揃えるのみ

use crate::audio::AudioTrack;

/// 音声出力なしのプレースホルダ（`--no-default-features` やヘッドレス CI 向け）。
#[derive(Debug, Default)]
pub struct GameAudioManager;

impl GameAudioManager {
    /// 常に成功（デバイスを開かない）。
    ///
    /// # Errors
    /// 現状は常に `Ok`（失敗経路なし）。
    pub fn new() -> anyhow::Result<Self> {
        Ok(Self)
    }

    /// 初期化済みかどうか（雛形）。
    #[must_use]
    pub const fn is_ready(&self) -> bool {
        true
    }

    /// サンプルからリップシンク値を計算する（雛形）。
    #[must_use]
    pub fn calculate_lip_sync(&self, audio_samples: &[f32], frame_size: usize) -> f32 {
        crate::audio::calculate_lip_sync_value(audio_samples, frame_size)
    }

    /// トラック音量（スタブでは no-op）。
    pub fn set_track_volume_linear(&mut self, _track: AudioTrack, _linear: f32, _fade_ms: u64) {}

    /// BGM 停止相当（スタブでは no-op）。
    pub fn pause_bgm(&mut self, _fade_ms: u64) {}
}
