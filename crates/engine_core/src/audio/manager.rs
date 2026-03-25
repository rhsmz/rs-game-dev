//! AudioManager（kira ラッパー）雛形。
//!
//! BGM / SE / Voice の 3 トラック構成を用意する。

use anyhow::anyhow;
use std::time::Duration;

use kira::sound::static_sound::StaticSoundData;
use kira::sound::static_sound::StaticSoundHandle;
/// `kira` の `AudioManager` が提供するミキサー。
use kira::track::{TrackBuilder, TrackHandle};

use kira::{
    AudioManager as KiraAudioManager, AudioManagerSettings, Decibels, DefaultBackend, Tween,
};

/// 音声管理（BGM / SE / Voice の 3 トラック）。
#[allow(dead_code)]
pub struct GameAudioManager {
    manager: KiraAudioManager<DefaultBackend>,
    bgm_track: TrackHandle,
    se_track: TrackHandle,
    voice_track: TrackHandle,
}

impl GameAudioManager {
    /// `kira` を初期化し、BGM / SE / Voice 用の 3 トラックを作成する。
    ///
    /// # Errors
    /// - `kira` の初期化やトラック生成に失敗した場合
    pub fn new() -> anyhow::Result<Self> {
        let settings = AudioManagerSettings::<DefaultBackend>::default();
        let mut manager =
            KiraAudioManager::<DefaultBackend>::new(settings).map_err(|e| anyhow!(e))?;

        // 雛形ではデフォルトの TrackBuilder で sub-track を確保する。
        // 後段タスクで effect/volume/tween 等を追加していく。
        let bgm_track = manager.add_sub_track(TrackBuilder::default()).map_err(|e| anyhow!(e))?;
        let se_track = manager.add_sub_track(TrackBuilder::default()).map_err(|e| anyhow!(e))?;
        let voice_track = manager.add_sub_track(TrackBuilder::default()).map_err(|e| anyhow!(e))?;

        Ok(Self { manager, bgm_track, se_track, voice_track })
    }

    /// 初期化済みかどうか（雛形）。
    #[must_use]
    pub const fn is_ready(&self) -> bool {
        true
    }

    /// BGM を再生する（雛形）。
    ///
    /// 現段階ではファイルロードではなく、呼び出し側で用意した `StaticSoundData` を
    /// そのまま `TrackHandle::play` へ渡す。
    ///
    /// # Errors
    /// - `kira` の再生要求が失敗した場合
    pub fn play_bgm(&mut self, sound_data: StaticSoundData) -> anyhow::Result<StaticSoundHandle> {
        let handle = self.bgm_track.play(sound_data).map_err(|e| anyhow!(e))?;
        Ok(handle)
    }

    /// BGM をファイルからロードする（雛形）。
    ///
    /// # Errors
    /// - ファイル読み込みやデコードが失敗した場合
    pub fn load_bgm_from_file(
        path: impl AsRef<std::path::Path>,
    ) -> anyhow::Result<StaticSoundData> {
        let sound_data = StaticSoundData::from_file(path).map_err(|e| anyhow!(e))?;
        Ok(sound_data)
    }

    /// BGM をループ再生する（雛形）。
    ///
    /// # Errors
    /// - `TrackHandle::play` が失敗した場合
    pub fn play_bgm_loop(
        &mut self,
        sound_data: &StaticSoundData,
        loop_start: f32,
        loop_end: f32,
    ) -> anyhow::Result<StaticSoundHandle> {
        let looped = sound_data.loop_region(f64::from(loop_start)..f64::from(loop_end));
        self.play_bgm(looped)
    }

    /// BGM をクロスフェードしながら切り替える（雛形）。
    ///
    /// 現時点では「既存トラックをサイレンスへフェードアウト → 新規を play → IDENTITY へフェードイン」
    /// という簡易モデルである。
    ///
    /// # Errors
    /// - `TrackHandle::play` が失敗した場合
    pub fn crossfade_bgm_to(
        &mut self,
        sound_data: StaticSoundData,
        fade_ms: u64,
    ) -> anyhow::Result<StaticSoundHandle> {
        let tween = Tween { duration: Duration::from_millis(fade_ms), ..Default::default() };

        // 先にフェードアウト（既存サウンドが対象）。
        self.bgm_track.set_volume(Decibels::SILENCE, tween);

        let handle = self.play_bgm(sound_data)?;

        // その後フェードイン。
        self.bgm_track.set_volume(Decibels::IDENTITY, tween);

        Ok(handle)
    }

    /// BGM を一時停止する（雛形）。
    pub fn pause_bgm(&mut self, fade_ms: u64) {
        let tween = Tween { duration: Duration::from_millis(fade_ms), ..Default::default() };
        self.bgm_track.pause(tween);
    }

    /// BGM を再開する（雛形）。
    pub fn resume_bgm(&mut self, fade_ms: u64) {
        let tween = Tween { duration: Duration::from_millis(fade_ms), ..Default::default() };
        self.bgm_track.resume(tween);
    }

    /// SE を再生する（雛形）。
    ///
    /// # Errors
    /// - `TrackHandle::play` が失敗した場合
    pub fn play_se(&mut self, sound_data: StaticSoundData) -> anyhow::Result<StaticSoundHandle> {
        let handle = self.se_track.play(sound_data).map_err(|e| anyhow!(e))?;
        Ok(handle)
    }

    /// SE のボリュームを変更する（雛形）。
    pub fn set_se_volume(&mut self, handle: &mut StaticSoundHandle, volume_db: f32, fade_ms: u64) {
        let tween = Tween { duration: Duration::from_millis(fade_ms), ..Default::default() };
        handle.set_volume(Decibels::from(volume_db), tween);
        let _ = self;
    }

    /// SE のパンを変更する（雛形）。
    pub fn set_se_panning(&mut self, handle: &mut StaticSoundHandle, panning: f32, fade_ms: u64) {
        let tween = Tween { duration: Duration::from_millis(fade_ms), ..Default::default() };
        handle.set_panning(panning, tween);
        let _ = self;
    }

    /// Voice を再生する（雛形）。
    ///
    /// # Errors
    /// - `TrackHandle::play` が失敗した場合
    pub fn play_voice(&mut self, sound_data: StaticSoundData) -> anyhow::Result<StaticSoundHandle> {
        let handle = self.voice_track.play(sound_data).map_err(|e| anyhow!(e))?;
        Ok(handle)
    }

    /// サンプルからリップシンク値を計算する（雛形）。
    #[must_use]
    pub fn calculate_lip_sync(&self, audio_samples: &[f32], frame_size: usize) -> f32 {
        crate::audio::calculate_lip_sync_value(audio_samples, frame_size)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use kira::Frame;
    use kira::sound::static_sound::StaticSoundSettings;

    #[test]
    #[ignore] // 音声デバイス/バックエンドに依存するため、デフォルト実行ではスキップする
    fn test_game_audio_manager_play_bgm_returns_ok() -> anyhow::Result<()> {
        let mut manager = GameAudioManager::new()?;

        // 無音に近い短いサンプルを生成して再生する。
        let frames: std::sync::Arc<[Frame]> = (0..32).map(|_| Frame::from_mono(0.0)).collect();

        let sound_data = StaticSoundData {
            sample_rate: 44_100,
            frames,
            settings: StaticSoundSettings::default(),
            slice: None,
        };

        let _handle = manager.play_bgm(sound_data)?;
        Ok(())
    }
}
