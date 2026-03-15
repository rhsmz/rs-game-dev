//! FFI (Foreign Function Interface) 境界モジュール。
//!
//! C/C++ ライブラリ (`Filament`, `Live2D` `Cubism` SDK) の
//! バインディングを隔離し、安全な Rust API でラップする。

#[allow(dead_code)]
pub struct FfiBinding;

#[allow(dead_code)]
impl FfiBinding {
    pub fn new() -> Self {
        Self
    }
}
