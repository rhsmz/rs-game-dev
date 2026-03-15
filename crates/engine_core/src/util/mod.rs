//! 内部ユーティリティモジュール。
//!
//! クレート内で共有されるヘルパー関数・型。

pub struct UtilHelper;

impl UtilHelper {
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}

impl Default for UtilHelper {
    fn default() -> Self {
        Self::new()
    }
}
