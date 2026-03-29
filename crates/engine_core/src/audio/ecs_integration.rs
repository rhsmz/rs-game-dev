//! Audio と ECS を接続するための型（雛形）。

use crate::ecs::component::Component;

/// AudioSource（ECS コンポーネント）。
///
/// 現段階では「どのサウンドを、どの音量で鳴らしているか」という状態のみを保持する。
#[derive(Debug, Clone)]
pub struct AudioSource {
    pub sound_id: String,
    pub volume: f32,
    pub playing: bool,
}

impl Component for AudioSource {}

/// BGM/SE/Voice のどのトラックに対して操作するか。
#[derive(Debug, Clone, Copy)]
pub enum AudioTrack {
    Bgm,
    Se,
    Voice,
}

/// Audio を操作するためのコマンド（ECS → `AudioSystem` 連携用）。
///
/// ## ファイルパスが解決できない場合
/// `audio_command_system` は **warn ログを出して当該コマンドをスキップ**する（ゲーム継続優先）。
/// 解決済みパスだがデコード／再生に失敗した場合も同様にスキップし、[`crate::audio::AudioLoadError`] 種別をログへ載せる。
#[derive(Debug, Clone)]
pub enum AudioCommand {
    PlayBgm(String),
    StopBgm { fade_ms: u64 },
    PlaySe(String),
    PlayVoice { voice_id: String, lip_sync: bool },
    SetVolume { track: AudioTrack, volume: f32 },
}
