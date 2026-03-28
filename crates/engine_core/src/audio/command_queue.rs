//! ECS リソースとして保持する `AudioCommand` キュー。

use std::collections::VecDeque;

use super::ecs_integration::AudioCommand;

/// `AudioCommand` の蓄積用キュー（`World` リソース）。
#[derive(Debug, Default)]
pub struct AudioCommandQueue {
    pub(crate) pending: VecDeque<AudioCommand>,
}

impl AudioCommandQueue {
    /// コマンドをキュー末尾に追加する。
    pub fn push(&mut self, cmd: AudioCommand) {
        self.pending.push_back(cmd);
    }

    /// 保留中のコマンドが無いとき `true`。
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.pending.is_empty()
    }
}
