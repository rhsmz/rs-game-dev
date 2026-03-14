//! スクリプトエンジンモジュール。
//!
//! タグベース構文のパーサおよび実行ステートマシン。
//! ADV パートの演出・分岐を記述する。

#[derive(Default)]
pub struct ScriptManager;

impl ScriptManager {
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}
