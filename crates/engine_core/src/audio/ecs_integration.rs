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
#[derive(Debug, Clone)]
pub enum AudioCommand {
    PlayBgm(String),
    StopBgm { fade_ms: u64 },
    PlaySe(String),
    PlayVoice { voice_id: String, lip_sync: bool },
    SetVolume { track: AudioTrack, volume: f32 },
}
