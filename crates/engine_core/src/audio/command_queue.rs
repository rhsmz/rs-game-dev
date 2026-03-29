//! ECS リソースとして保持する `AudioCommand` キュー。

use std::collections::VecDeque;

use crate::ecs::entity::Entity;

use super::ecs_integration::AudioCommand;

/// キュー投入時に付与されるメタデータ付きコマンド。
#[derive(Debug, Clone)]
pub struct QueuedAudioCommand {
    /// キュー投入順の単調増加 ID（ログ・トレース用）。
    pub seq: u64,
    /// 発行元 ECS エンティティ（任意）。
    pub source_entity: Option<Entity>,
    /// 実コマンド。
    pub command: AudioCommand,
}

/// `AudioCommand` の蓄積用キュー（`World` リソース）。
#[derive(Debug)]
pub struct AudioCommandQueue {
    next_seq: u64,
    pub(crate) pending: VecDeque<QueuedAudioCommand>,
}

impl Default for AudioCommandQueue {
    fn default() -> Self {
        Self { next_seq: 1, pending: VecDeque::new() }
    }
}

impl AudioCommandQueue {
    /// コマンドをキュー末尾に追加する（`source_entity` なし）。
    pub fn push(&mut self, cmd: AudioCommand) {
        let seq = self.next_seq;
        self.next_seq = self.next_seq.wrapping_add(1);
        self.pending.push_back(QueuedAudioCommand { seq, source_entity: None, command: cmd });
    }

    /// 発行元エンティティを付与してキュー末尾に追加する。
    pub fn push_from(&mut self, source: Entity, cmd: AudioCommand) {
        let seq = self.next_seq;
        self.next_seq = self.next_seq.wrapping_add(1);
        self.pending.push_back(QueuedAudioCommand {
            seq,
            source_entity: Some(source),
            command: cmd,
        });
    }

    /// 保留中のコマンドが無いとき `true`。
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.pending.is_empty()
    }
}
