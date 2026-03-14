//! セーブ/ロードモジュール。
//!
//! `MessagePack` (`rmp-serde`) ベースのセーブデータ管理。
//! スロット管理、バージョンマイグレーションを提供する。

pub struct SaveManager;

impl SaveManager {
    pub fn new() -> Self {
        Self
    }
}
