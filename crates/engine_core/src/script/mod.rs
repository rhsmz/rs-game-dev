//! スクリプトエンジンモジュール。
//!
//! タグベース構文のパーサおよび実行ステートマシン。
//! ADV パートの演出・分岐を記述する。

pub struct ScriptManager;

impl Default for ScriptManager {
    fn default() -> Self {
        Self::new()
    }
}

impl ScriptManager {
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}
