//! `AudioCommand` を `GameAudioManager` へ適用する。

use std::path::Path;

use crate::ecs::world::World;

use super::command_queue::AudioCommandQueue;
use super::ecs_integration::AudioCommand;
use super::manager::GameAudioManager;

fn apply_audio_command(manager: &mut GameAudioManager, cmd: AudioCommand) -> anyhow::Result<()> {
    match cmd {
        AudioCommand::PlayBgm(path_or_id) => {
            let p = Path::new(&path_or_id);
            if !p.exists() {
                log::warn!(
                    "PlayBgm: path does not exist ({path_or_id:?}); skipping (asset resolution is not wired)"
                );
                return Ok(());
            }
            let data = GameAudioManager::load_bgm_from_file(p)?;
            let _handle = manager.play_bgm(data)?;
            Ok(())
        }
        AudioCommand::StopBgm { fade_ms } => {
            manager.pause_bgm(fade_ms);
            Ok(())
        }
        AudioCommand::PlaySe(path_or_id) => {
            let p = Path::new(&path_or_id);
            if !p.exists() {
                log::warn!(
                    "PlaySe: path does not exist ({path_or_id:?}); skipping (asset resolution is not wired)"
                );
                return Ok(());
            }
            let data = GameAudioManager::load_bgm_from_file(p)?;
            let _handle = manager.play_se(data)?;
            Ok(())
        }
        AudioCommand::PlayVoice { voice_id, lip_sync } => {
            if lip_sync {
                log::trace!("PlayVoice: lip_sync requested for {voice_id:?} (not wired yet)");
            }
            let p = Path::new(&voice_id);
            if !p.exists() {
                log::warn!(
                    "PlayVoice: path does not exist ({voice_id:?}); skipping (asset resolution is not wired)"
                );
                return Ok(());
            }
            let data = GameAudioManager::load_bgm_from_file(p)?;
            let _handle = manager.play_voice(data)?;
            Ok(())
        }
        AudioCommand::SetVolume { track, volume } => {
            manager.set_track_volume_linear(track, volume, 0);
            Ok(())
        }
    }
}

/// キュー内のコマンドを順に適用する（デバイス非依存テスト用シンク差し替え）。
trait AudioCommandSink {
    /// 1 件のコマンドを処理する。
    fn dispatch(&mut self, cmd: AudioCommand) -> anyhow::Result<()>;
}

impl AudioCommandSink for GameAudioManager {
    fn dispatch(&mut self, cmd: AudioCommand) -> anyhow::Result<()> {
        apply_audio_command(self, cmd)
    }
}

#[derive(Debug, Default)]
#[cfg_attr(not(test), allow(dead_code))] // ユニットテストでのみ使用（本番バイナリでは未使用）。
struct RecordingAudioSink {
    log: Vec<AudioCommand>,
}

impl AudioCommandSink for RecordingAudioSink {
    fn dispatch(&mut self, cmd: AudioCommand) -> anyhow::Result<()> {
        self.log.push(cmd);
        Ok(())
    }
}

fn dispatch_audio_commands_to_sink<S: AudioCommandSink>(
    queue: &mut AudioCommandQueue,
    sink: &mut S,
) {
    for cmd in queue.pending.drain(..) {
        if let Err(e) = sink.dispatch(cmd) {
            log::warn!("audio command failed: {e:#}");
        }
    }
}

/// `AudioCommandQueue` を空にし、[`GameAudioManager`] へ反映する。
///
/// # 推奨ステージ
/// ECS の `PostUpdate`（ロジック確定後に音声へ反映することを推奨）。
///
/// # 同一フレームの優先度
/// キューは FIFO。`StopBgm` を先に積めば、その後の `PlayBgm` が意図どおり効く。
///
/// `GameAudioManager` リソースが無い場合、保留コマンドは破棄し警告ログを出す。
pub fn audio_command_system(world: &mut World) {
    let Some(mut queue) = world.remove_resource::<AudioCommandQueue>() else {
        return;
    };

    let Some(manager) = world.get_resource_mut::<GameAudioManager>() else {
        if !queue.is_empty() {
            log::warn!(
                "audio_command_system: dropping {} audio command(s): GameAudioManager resource missing",
                queue.pending.len()
            );
            queue.pending.clear();
        }
        world.insert_resource(queue);
        return;
    };

    dispatch_audio_commands_to_sink(&mut queue, manager);
    world.insert_resource(queue);
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]
    #![allow(clippy::expect_used)]

    use super::*;
    use crate::audio::AudioTrack;
    use crate::ecs::world::World;

    #[test]
    fn test_dispatch_recording_sink_fifo_order() {
        let mut q = AudioCommandQueue::default();
        q.push(AudioCommand::StopBgm { fade_ms: 10 });
        q.push(AudioCommand::PlayBgm("x".into()));

        let mut sink = RecordingAudioSink::default();
        dispatch_audio_commands_to_sink(&mut q, &mut sink);

        assert_eq!(sink.log.len(), 2);
        assert!(matches!(sink.log[0], AudioCommand::StopBgm { .. }));
        assert!(matches!(sink.log[1], AudioCommand::PlayBgm(_)));
        assert!(q.is_empty());
    }

    #[test]
    fn test_audio_command_system_drops_when_no_manager() {
        let mut world = World::new();
        let mut q = AudioCommandQueue::default();
        q.push(AudioCommand::StopBgm { fade_ms: 0 });
        world.insert_resource(q);

        audio_command_system(&mut world);

        let q_after = world.get_resource::<AudioCommandQueue>().expect("queue reinserted");
        assert!(q_after.is_empty());
    }

    /// P0-3: ECS リソース上のキュー → `audio_command_system` → `GameAudioManager` の縦切り（ローカル検証用）。
    #[test]
    #[ignore = "音声デバイス/バックエンドに依存するため、デフォルト実行ではスキップする"]
    fn test_audio_ecs_set_volume_end_to_end() {
        let mut world = World::new();
        let manager = GameAudioManager::new().expect("kira backend");
        world.insert_resource(manager);
        let mut q = AudioCommandQueue::default();
        q.push(AudioCommand::SetVolume { track: AudioTrack::Bgm, volume: 0.42 });
        world.insert_resource(q);

        audio_command_system(&mut world);

        let q_after = world.get_resource::<AudioCommandQueue>().expect("queue reinserted");
        assert!(q_after.is_empty());
    }
}
