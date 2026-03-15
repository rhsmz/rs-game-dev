//! FFI (Foreign Function Interface) 境界モジュール。
//!
//! C/C++ ライブラリ (`Filament`, `Live2D` `Cubism` SDK) の
//! バインディングを隔離し、安全な Rust API でラップする。

pub struct FfiBinding;

impl FfiBinding {
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}

impl Default for FfiBinding {
    fn default() -> Self {
        Self::new()
    }
}
