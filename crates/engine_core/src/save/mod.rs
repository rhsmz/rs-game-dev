//! セーブ/ロードモジュール。
//!
//! `MessagePack` (`rmp-serde`) ベースのセーブデータ管理。
//! スロット管理、バージョンマイグレーションを提供する。

#[derive(Default)]
pub struct SaveManager;

impl SaveManager {
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}
