//! 音声キューの FIFO と `Play` / `Stop` / `SetVolume` 順序を、出力デバイスなしで検証する。

use engine_core::audio::{
    AudioCommand, AudioCommandQueue, AudioTrack, drain_audio_command_queue_with,
};
use engine_core::ecs::world::World;

#[test]
fn test_drain_audio_queue_play_stop_set_volume_order() -> anyhow::Result<()> {
    let mut world = World::new();
    let mut q = AudioCommandQueue::default();
    q.push(AudioCommand::PlayBgm("bgm.ogg".into()));
    q.push(AudioCommand::StopBgm { fade_ms: 100 });
    q.push(AudioCommand::PlaySe("se.wav".into()));
    q.push(AudioCommand::SetVolume { track: AudioTrack::Bgm, volume: 0.5 });
    q.push(AudioCommand::PlayVoice { voice_id: "voice.wav".into(), lip_sync: false });
    world.insert_resource(q);

    let mut seen = Vec::new();
    drain_audio_command_queue_with(&mut world, |cmd| {
        seen.push(cmd);
        Ok(())
    })?;

    assert_eq!(seen.len(), 5);
    assert!(matches!(seen[0], AudioCommand::PlayBgm(_)));
    assert!(matches!(seen[1], AudioCommand::StopBgm { .. }));
    assert!(matches!(seen[2], AudioCommand::PlaySe(_)));
    assert!(matches!(seen[3], AudioCommand::SetVolume { .. }));
    assert!(matches!(seen[4], AudioCommand::PlayVoice { .. }));

    let q_after = world
        .get_resource::<AudioCommandQueue>()
        .ok_or_else(|| anyhow::anyhow!("AudioCommandQueue should be reinserted after drain"))?;
    assert!(q_after.is_empty());
    Ok(())
}
