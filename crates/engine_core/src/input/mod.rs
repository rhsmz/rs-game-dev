//! 入力抽象化モジュール。
//!
//! キーボード、マウス、ゲームパッドの統一入力管理。
//! アクションマッピングによる抽象入力を提供する。

pub struct InputManager;

impl Default for InputManager {
    fn default() -> Self {
        Self::new()
    }
}

impl InputManager {
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}
