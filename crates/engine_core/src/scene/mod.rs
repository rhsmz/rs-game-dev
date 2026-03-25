//! シーン管理モジュール。
//!
//! `Scene` トレイトによるゲームパート抽象化、
//! スタックベースのシーン遷移、非同期ロードを提供する。

pub struct SceneManager;

impl Default for SceneManager {
    fn default() -> Self {
        Self::new()
    }
}

impl SceneManager {
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}
