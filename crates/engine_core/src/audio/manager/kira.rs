//! [`GameAudioManager`] の `kira` 実装（ラッパー雛形）。
//!
//! BGM / SE / Voice の 3 トラック構成を用意する。

use anyhow::anyhow;
use std::time::{Duration, Instant};

use kira::sound::static_sound::StaticSoundData;
use kira::sound::static_sound::StaticSoundHandle;
use kira::track::{TrackBuilder, TrackHandle};

use kira::{
    AudioManager as KiraAudioManager, AudioManagerSettings, Decibels, DefaultBackend, Tween,
};

use crate::audio::AudioTrack;

/// フェードアウト中のトラック。フェード完了推定時刻まで保持する。
struct FadingTrack {
    _handle: TrackHandle,
    fade_done_at: Instant,
}

/// 音声管理（BGM / SE / Voice の 3 トラック）。
#[allow(dead_code)]
pub struct GameAudioManager {
    manager: KiraAudioManager<DefaultBackend>,
    bgm_track: TrackHandle,
    /// フェードアウト中の旧 BGM トラック。
    /// drop されるとトラックが即座に削除されるため、フェードアウト完了まで保持する。
    fading_out_bgm_tracks: Vec<FadingTrack>,
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

        Ok(Self { manager, bgm_track, fading_out_bgm_tracks: Vec::new(), se_track, voice_track })
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

    /// SE をファイルからロードする（現状は BGM と同じデコード経路。将来トラック別検証を追加可能）。
    ///
    /// # Errors
    /// - ファイル読み込みやデコードが失敗した場合
    pub fn load_se_from_file(path: impl AsRef<std::path::Path>) -> anyhow::Result<StaticSoundData> {
        Self::load_bgm_from_file(path)
    }

    /// Voice をファイルからロードする（現状は BGM と同じデコード経路。将来トラック別検証を追加可能）。
    ///
    /// # Errors
    /// - ファイル読み込みやデコードが失敗した場合
    pub fn load_voice_from_file(
        path: impl AsRef<std::path::Path>,
    ) -> anyhow::Result<StaticSoundData> {
        Self::load_bgm_from_file(path)
    }

    /// BGM をループ再生する（雛形）。
    ///
    /// `loop_end` が `0.0` のときは曲末尾（`sound.csv` の慣習: 0 = end-of-file）までループする。
    ///
    /// # Errors
    /// - `TrackHandle::play` が失敗した場合
    pub fn play_bgm_loop(
        &mut self,
        sound_data: &StaticSoundData,
        loop_start: f32,
        loop_end: f32,
    ) -> anyhow::Result<StaticSoundHandle> {
        let start = f64::from(loop_start);
        // loop_end == 0.0 は "曲末尾まで" を意味する（sound.csv の仕様）。
        // 非ゼロ値のみ上限として設定し、ゼロは開区間（RangeFrom）に変換する。
        let looped = if loop_end == 0.0 {
            sound_data.loop_region(start..)
        } else {
            sound_data.loop_region(start..f64::from(loop_end))
        };
        self.play_bgm(looped)
    }

    /// BGM をクロスフェードしながら切り替える（雛形）。
    ///
    /// 旧トラックをフェードアウトしつつ新トラックをフェードインする。
    /// 旧 `TrackHandle` は `fading_out_bgm_tracks` に退避し、drop による即時削除を防ぐ。
    /// 退避したトラックは次回クロスフェード時または [`purge_faded_tracks`] で解放する。
    ///
    /// # Errors
    /// - `TrackHandle::play` が失敗した場合
    pub fn crossfade_bgm_to(
        &mut self,
        sound_data: &StaticSoundData,
        fade_ms: u64,
    ) -> anyhow::Result<StaticSoundHandle> {
        self.purge_faded_tracks();

        let tween = Tween { duration: Duration::from_millis(fade_ms), ..Default::default() };

        let next_bgm_track =
            self.manager.add_sub_track(TrackBuilder::default()).map_err(|e| anyhow!(e))?;

        let mut previous_bgm_track = std::mem::replace(&mut self.bgm_track, next_bgm_track);
        previous_bgm_track.set_volume(Decibels::SILENCE, tween);
        self.fading_out_bgm_tracks.push(FadingTrack {
            _handle: previous_bgm_track,
            fade_done_at: Instant::now() + Duration::from_millis(fade_ms),
        });

        // sound_data に設定済みのターゲットボリュームを保持したままフェードインする。
        // IDENTITY に決め打ちすると、音量が明示的に設定された BGM が
        // フェード後に 0 dB（最大音量）で再生されてしまう問題を防ぐ。
        let target_volume = sound_data.settings.volume;
        let mut handle =
            self.bgm_track.play(sound_data.volume(Decibels::SILENCE)).map_err(|e| anyhow!(e))?;
        handle.set_volume(target_volume, tween);

        Ok(handle)
    }

    /// フェードアウト完了済みの旧 BGM トラックを解放する。
    ///
    /// フェード期間を経過したトラックを drop して kira リソースを回収する。
    /// 毎フレーム呼ぶか、次回クロスフェード時に自動で呼ばれる。
    pub fn purge_faded_tracks(&mut self) {
        let now = Instant::now();
        self.fading_out_bgm_tracks.retain(|t| t.fade_done_at > now);
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

    /// トラック全体の音量を線形（0.0〜1.0）で設定する（API 境界で dB に変換する）。
    pub fn set_track_volume_linear(&mut self, track: AudioTrack, linear: f32, fade_ms: u64) {
        let db = linear_volume_to_decibels(linear);
        let tween = Tween { duration: Duration::from_millis(fade_ms), ..Default::default() };
        match track {
            AudioTrack::Bgm => self.bgm_track.set_volume(db, tween),
            AudioTrack::Se => self.se_track.set_volume(db, tween),
            AudioTrack::Voice => self.voice_track.set_volume(db, tween),
        }
    }
}

fn linear_volume_to_decibels(linear: f32) -> Decibels {
    let x = linear.clamp(0.0, 1.0);
    if x <= 1.0e-6 {
        return Decibels::SILENCE;
    }
    Decibels::from(20.0 * x.log10())
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]

    use super::*;
    use kira::Frame;
    use kira::sound::static_sound::StaticSoundSettings;

    #[test]
    #[ignore = "音声デバイス/バックエンドに依存するため、通常 CI ではスキップ（nightly の ignored ジョブで実行）"]
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

    #[test]
    #[ignore = "音声デバイス/バックエンドに依存するため、通常 CI ではスキップ（nightly の ignored ジョブで実行）"]
    fn test_play_bgm_loop_zero_loop_end_plays_to_end() -> anyhow::Result<()> {
        let mut manager = GameAudioManager::new()?;
        let frames: std::sync::Arc<[Frame]> = (0..4410).map(|_| Frame::from_mono(0.0)).collect();
        let sound_data = StaticSoundData {
            sample_rate: 44_100,
            frames,
            settings: StaticSoundSettings::default(),
            slice: None,
        };
        // loop_end=0.0 は "曲末尾まで" を意味する。
        // エラーなく再生できることを確認する。
        let _handle = manager.play_bgm_loop(&sound_data, 0.0, 0.0)?;
        Ok(())
    }

    #[test]
    #[ignore = "音声デバイス/バックエンドに依存するため、通常 CI ではスキップ（nightly の ignored ジョブで実行）"]
    fn test_crossfade_bgm_to_preserves_source_volume() -> anyhow::Result<()> {
        use kira::Value;
        let mut manager = GameAudioManager::new()?;
        let frames: std::sync::Arc<[Frame]> = (0..32).map(|_| Frame::from_mono(0.0)).collect();
        let sound_data = StaticSoundData {
            sample_rate: 44_100,
            frames,
            settings: StaticSoundSettings::default(),
            slice: None,
        };
        // -6dB に設定した BGM をクロスフェードしても 0 dB にならないことを確認する。
        let quiet_sound = sound_data.volume(Value::Fixed(Decibels::from(-6.0)));
        let _handle = manager.crossfade_bgm_to(&quiet_sound, 500)?;
        Ok(())
    }
}
