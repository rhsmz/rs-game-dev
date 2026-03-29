//! `AudioCommand` を `GameAudioManager` へ適用する。
//!
//! # 同一フレーム内の優先ルール（競合コマンド）
//!
//! **追加順＝適用順（厳密 FIFO）**。キューに `StopBgm` を先に積み、その後に `PlayBgm` を積めば、
//! ドレイン時は **必ず Stop → Play** の順でマネージャに渡る。逆順を積めば逆の結果になる。
//! 「Stop を Play より常に優先」などの暗黙の再並べ替えは行わない。
//!
//! # 観測用ログ（warn / trace）
//!
//! `target = "engine_core::audio"` を統一する。異常系 warn 行には次のキーを含める（grep・監視向け）:
//! - `seq=` … [`QueuedAudioCommand::seq`]
//! - `source=` … 発行元 `Entity` の表示、または `-`
//! - `op=` … `PlayBgm` / `PlaySe` / `PlayVoice` / `StopBgm` / `SetVolume` など
//! - `err_kind=` … `skip_missing_file` / `io` / `decode` / `backend`
//!
//! グローバルロガーへ流す本文の完全一致はテストしないが、必須キーを含む行は `format_audio_warn_message` の単体テストで検証する。
//! 手動確認は `RUST_LOG=engine_core::audio=trace` 等を参照。

#[cfg(feature = "audio-kira")]
use std::path::Path;

use crate::ecs::entity::Entity;
use crate::ecs::world::World;

use super::command_queue::{AudioCommandQueue, QueuedAudioCommand};
#[cfg(feature = "audio-kira")]
use super::ecs_integration::AudioCommand;
#[cfg(feature = "audio-kira")]
use super::error::AudioLoadError;
use super::manager::GameAudioManager;

/// `GameAudioManager` 不在時に一度に捨てるコマンド数がこの値以上なら、追加で warn を出す（運用監視用）。
const AUDIO_QUEUE_DROP_BULK_WARN_THRESHOLD: usize = 16;

fn format_source_entity(source: Option<Entity>) -> String {
    source.map_or_else(|| "-".to_string(), |e| e.to_string())
}

/// 異常系 warn 1 行の本文（手動 `RUST_LOG` 確認と単体テストで共有）。
#[cfg(any(test, feature = "audio-kira"))]
#[must_use]
fn format_audio_warn_message(
    seq: u64,
    source: Option<Entity>,
    op: &str,
    err_kind: &str,
    msg: &str,
) -> String {
    format!("seq={seq} source={} op={op} err_kind={err_kind} {msg}", format_source_entity(source),)
}

#[cfg(feature = "audio-kira")]
#[allow(clippy::missing_const_for_fn)] // 将来 `const` 化可能だが現状は可読性優先
fn audio_load_error_kind(e: &AudioLoadError) -> &'static str {
    match e {
        AudioLoadError::Io(_) => "io",
        AudioLoadError::Decode(_) => "decode",
        AudioLoadError::Backend(_) => "backend",
    }
}

#[cfg(feature = "audio-kira")]
fn log_audio_warn(seq: u64, source: Option<Entity>, op: &str, err_kind: &str, msg: &str) {
    let line = format_audio_warn_message(seq, source, op, err_kind, msg);
    log::warn!(target: "engine_core::audio", "{line}");
}

#[cfg(feature = "audio-kira")]
#[allow(clippy::too_many_lines)] // コマンド種別ごとの分岐を 1 箇所に集約
#[allow(clippy::unnecessary_wraps)] // `AudioCommandSink` とシグネチャを揃える
fn apply_audio_command(
    manager: &mut GameAudioManager,
    item: QueuedAudioCommand,
) -> anyhow::Result<()> {
    let seq = item.seq;
    let source = item.source_entity;
    match item.command {
        AudioCommand::PlayBgm(path_or_id) => {
            let p = Path::new(&path_or_id);
            if !p.exists() {
                log_audio_warn(
                    seq,
                    source,
                    "PlayBgm",
                    "skip_missing_file",
                    &format!("path does not exist ({path_or_id:?}); skipping"),
                );
                return Ok(());
            }
            let data = match GameAudioManager::load_sound_from_file(p) {
                Ok(d) => d,
                Err(e) => {
                    log_audio_warn(
                        seq,
                        source,
                        "PlayBgm",
                        audio_load_error_kind(&e),
                        &format!("load failed: {e}"),
                    );
                    return Ok(());
                }
            };
            if let Err(e) = manager.play_bgm(data) {
                log_audio_warn(seq, source, "PlayBgm", "backend", &format!("play failed: {e:?}"));
            }
            Ok(())
        }
        AudioCommand::StopBgm { fade_ms } => {
            log::trace!(
                target: "engine_core::audio",
                "seq={seq} source={} op=StopBgm fade_ms={fade_ms}",
                format_source_entity(source),
            );
            manager.pause_bgm(fade_ms);
            Ok(())
        }
        AudioCommand::PlaySe(path_or_id) => {
            let p = Path::new(&path_or_id);
            if !p.exists() {
                log_audio_warn(
                    seq,
                    source,
                    "PlaySe",
                    "skip_missing_file",
                    &format!("path does not exist ({path_or_id:?}); skipping"),
                );
                return Ok(());
            }
            let data = match GameAudioManager::load_sound_from_file(p) {
                Ok(d) => d,
                Err(e) => {
                    log_audio_warn(
                        seq,
                        source,
                        "PlaySe",
                        audio_load_error_kind(&e),
                        &format!("load failed: {e}"),
                    );
                    return Ok(());
                }
            };
            if let Err(e) = manager.play_se(data) {
                log_audio_warn(seq, source, "PlaySe", "backend", &format!("play failed: {e:?}"));
            }
            Ok(())
        }
        AudioCommand::PlayVoice { voice_id, lip_sync } => {
            if lip_sync {
                log::trace!(
                    target: "engine_core::audio",
                    "seq={seq} source={} op=PlayVoice lip_sync=requested voice_id={voice_id:?} (not wired yet)",
                    format_source_entity(source),
                );
            }
            let p = Path::new(&voice_id);
            if !p.exists() {
                log_audio_warn(
                    seq,
                    source,
                    "PlayVoice",
                    "skip_missing_file",
                    &format!("path does not exist ({voice_id:?}); skipping"),
                );
                return Ok(());
            }
            let data = match GameAudioManager::load_sound_from_file(p) {
                Ok(d) => d,
                Err(e) => {
                    log_audio_warn(
                        seq,
                        source,
                        "PlayVoice",
                        audio_load_error_kind(&e),
                        &format!("load failed: {e}"),
                    );
                    return Ok(());
                }
            };
            if let Err(e) = manager.play_voice(data) {
                log_audio_warn(seq, source, "PlayVoice", "backend", &format!("play failed: {e:?}"));
            }
            Ok(())
        }
        AudioCommand::SetVolume { track, volume } => {
            log::trace!(
                target: "engine_core::audio",
                "seq={seq} source={} op=SetVolume track={track:?} volume={volume}",
                format_source_entity(source),
            );
            manager.set_track_volume_linear(track, volume, 0);
            Ok(())
        }
    }
}

#[cfg(not(feature = "audio-kira"))]
#[allow(clippy::needless_pass_by_value)] // `AudioCommandSink` と所有権移動のシグネチャを揃える
#[allow(clippy::unnecessary_wraps)]
fn apply_audio_command(
    _manager: &mut GameAudioManager,
    item: QueuedAudioCommand,
) -> anyhow::Result<()> {
    log::trace!(
        target: "engine_core::audio",
        "seq={} source={} audio-kira disabled; ignoring {:?}",
        item.seq,
        format_source_entity(item.source_entity),
        item.command
    );
    Ok(())
}

/// キュー内のコマンドを順に適用する（デバイス非依存テスト用シンク差し替え）。
trait AudioCommandSink {
    /// 1 件のコマンドを処理する。
    fn dispatch(&mut self, item: QueuedAudioCommand) -> anyhow::Result<()>;
}

impl AudioCommandSink for GameAudioManager {
    fn dispatch(&mut self, item: QueuedAudioCommand) -> anyhow::Result<()> {
        apply_audio_command(self, item)
    }
}

#[derive(Debug, Default)]
#[cfg_attr(not(test), allow(dead_code))] // ユニットテストでのみ使用（本番バイナリでは未使用）。
struct RecordingAudioSink {
    log: Vec<QueuedAudioCommand>,
}

impl AudioCommandSink for RecordingAudioSink {
    fn dispatch(&mut self, item: QueuedAudioCommand) -> anyhow::Result<()> {
        self.log.push(item);
        Ok(())
    }
}

fn dispatch_audio_commands_to_sink<S: AudioCommandSink>(
    queue: &mut AudioCommandQueue,
    sink: &mut S,
) {
    for item in queue.pending.drain(..) {
        if let Err(e) = sink.dispatch(item) {
            log::warn!(target: "engine_core::audio", "audio command dispatch error: {e:#}");
        }
    }
}

/// `AudioCommandQueue` を一時的に取り出し、各コマンドを `f` に渡してからキューを戻す（検証・ツール向け）。
///
/// `GameAudioManager` を介さないため、出力デバイスに依存しない。本番ループでは [`audio_command_system`] を使うこと。
///
/// # Errors
/// - `f` が `Err` を返したとき、その時点で処理を打ち切り、未処理コマンドはキューに戻さない（既に drain 済み分は失う）。
///   テストでは `f` を常に成功させることを推奨する。
pub fn drain_audio_command_queue_with(
    world: &mut World,
    mut f: impl FnMut(QueuedAudioCommand) -> anyhow::Result<()>,
) -> anyhow::Result<()> {
    let Some(mut queue) = world.remove_resource::<AudioCommandQueue>() else {
        return Ok(());
    };
    for item in queue.pending.drain(..) {
        f(item)?;
    }
    world.insert_resource(queue);
    Ok(())
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
            let dropped = queue.pending.len();
            let first_seq = queue.pending.front().map(|q| q.seq);
            log::warn!(
                target: "engine_core::audio",
                "audio_command_system: dropping {dropped} audio command(s): GameAudioManager resource missing (first_seq={first_seq:?})",
            );
            if dropped >= AUDIO_QUEUE_DROP_BULK_WARN_THRESHOLD {
                log::warn!(
                    target: "engine_core::audio",
                    "audio_command_system: bulk_drop threshold exceeded (drop_count={dropped} threshold={AUDIO_QUEUE_DROP_BULK_WARN_THRESHOLD})",
                );
            }
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
    use crate::audio::{AudioCommand, AudioTrack};
    use crate::ecs::world::World;

    #[test]
    fn test_format_audio_warn_message_includes_required_keys() {
        let mut w = World::new();
        let e = w.spawn();
        let line = format_audio_warn_message(
            42,
            Some(e),
            "PlayBgm",
            "skip_missing_file",
            "path does not exist (\"nope\"); skipping",
        );
        assert!(line.contains("seq=42"), "{line}");
        assert!(line.contains("op=PlayBgm"), "{line}");
        assert!(line.contains("err_kind=skip_missing_file"), "{line}");
        assert!(line.contains("source=Entity("), "{line}");

        let line_dash = format_audio_warn_message(1, None, "PlaySe", "decode", "boom");
        assert!(line_dash.contains("seq=1"), "{line_dash}");
        assert!(line_dash.contains("source=-"), "{line_dash}");
        assert!(line_dash.contains("op=PlaySe"), "{line_dash}");
        assert!(line_dash.contains("err_kind=decode"), "{line_dash}");
    }

    #[test]
    fn test_dispatch_recording_sink_fifo_order() {
        let mut q = AudioCommandQueue::default();
        q.push(AudioCommand::StopBgm { fade_ms: 10 });
        q.push(AudioCommand::PlayBgm("x".into()));

        let mut sink = RecordingAudioSink::default();
        dispatch_audio_commands_to_sink(&mut q, &mut sink);

        assert_eq!(sink.log.len(), 2);
        assert_eq!(sink.log[0].seq, 1);
        assert_eq!(sink.log[1].seq, 2);
        assert!(matches!(sink.log[0].command, AudioCommand::StopBgm { .. }));
        assert!(matches!(sink.log[1].command, AudioCommand::PlayBgm(_)));
        assert!(q.is_empty());
    }

    #[test]
    fn test_push_from_sets_source_entity() {
        let mut world = World::new();
        let e = world.spawn();
        let mut q = AudioCommandQueue::default();
        q.push_from(e, AudioCommand::StopBgm { fade_ms: 0 });

        let mut sink = RecordingAudioSink::default();
        dispatch_audio_commands_to_sink(&mut q, &mut sink);

        assert_eq!(sink.log[0].source_entity, Some(e));
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

    /// マネージャ不在時に大量ドロップしてもキューが空になり、bulk 閾値ロジックがパニックしない。
    #[test]
    fn test_audio_command_system_drops_bulk_without_manager() {
        let mut world = World::new();
        let mut q = AudioCommandQueue::default();
        for i in 0_u64..20 {
            q.push(AudioCommand::StopBgm { fade_ms: i });
        }
        world.insert_resource(q);

        audio_command_system(&mut world);

        let q_after = world.get_resource::<AudioCommandQueue>().expect("queue reinserted");
        assert!(q_after.is_empty());
    }

    /// `StopBgm` → `PlayBgm` → `StopBgm` → `PlayBgm` を積んだ順で drain する（暗黙の並べ替えなし）。
    #[test]
    fn test_fifo_stop_play_stop_play_policy() {
        let mut q = AudioCommandQueue::default();
        q.push(AudioCommand::StopBgm { fade_ms: 1 });
        q.push(AudioCommand::PlayBgm("a".into()));
        q.push(AudioCommand::StopBgm { fade_ms: 2 });
        q.push(AudioCommand::PlayBgm("b".into()));

        let mut sink = RecordingAudioSink::default();
        dispatch_audio_commands_to_sink(&mut q, &mut sink);

        assert_eq!(sink.log.len(), 4);
        assert!(matches!(sink.log[0].command, AudioCommand::StopBgm { fade_ms: 1 }));
        assert!(matches!(sink.log[1].command, AudioCommand::PlayBgm(_)));
        assert!(matches!(sink.log[2].command, AudioCommand::StopBgm { fade_ms: 2 }));
        assert!(matches!(sink.log[3].command, AudioCommand::PlayBgm(_)));
    }

    /// `PlaySe` と `SetVolume` が混在しても投入順を維持する。
    #[test]
    fn test_fifo_play_se_set_volume_play_voice() {
        let mut q = AudioCommandQueue::default();
        q.push(AudioCommand::PlaySe("s".into()));
        q.push(AudioCommand::SetVolume { track: AudioTrack::Se, volume: 0.3 });
        q.push(AudioCommand::PlayVoice { voice_id: "v".into(), lip_sync: false });

        let mut sink = RecordingAudioSink::default();
        dispatch_audio_commands_to_sink(&mut q, &mut sink);

        assert_eq!(sink.log.len(), 3);
        assert!(matches!(sink.log[0].command, AudioCommand::PlaySe(_)));
        assert!(matches!(
            sink.log[1].command,
            AudioCommand::SetVolume { track: AudioTrack::Se, .. }
        ));
        assert!(matches!(sink.log[2].command, AudioCommand::PlayVoice { .. }));
    }

    /// `audio-kira` オフ時: スタブマネージャでキューが確実に空になる（CI `--no-default-features`）。
    #[cfg(not(feature = "audio-kira"))]
    #[test]
    fn test_audio_ecs_set_volume_drains_with_stub_manager() {
        let mut world = World::new();
        let manager = GameAudioManager::new().expect("stub backend");
        world.insert_resource(manager);
        let mut q = AudioCommandQueue::default();
        q.push(AudioCommand::SetVolume { track: AudioTrack::Bgm, volume: 0.42 });
        world.insert_resource(q);

        audio_command_system(&mut world);

        let q_after = world.get_resource::<AudioCommandQueue>().expect("queue reinserted");
        assert!(q_after.is_empty());
    }

    /// 実 `kira` バックエンド＋出力デバイスが必要。nightly の `cargo test -- --ignored` 向け。
    #[cfg(feature = "audio-kira")]
    #[test]
    #[ignore = "音声デバイス/バックエンドに依存するため、通常 CI ではスキップ（nightly の ignored ジョブで実行）"]
    fn test_audio_ecs_set_volume_end_to_end_kira() {
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
