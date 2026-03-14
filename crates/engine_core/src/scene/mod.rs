//! シーン管理モジュール。
//!
//! `Scene` トレイトによるゲームパート抽象化、
//! スタックベースのシーン遷移、非同期ロードを提供する。

#[derive(Default)]
pub struct SceneManager;

impl SceneManager {
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}
