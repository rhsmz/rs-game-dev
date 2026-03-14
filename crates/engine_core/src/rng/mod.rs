//! Dual RNG (乱数エンジン) モジュール。
//!
//! - **`VisualRng`** (`Xoshiro256++`): 演出用の非決定論的乱数
//! - **`LogicRng`** (`ChaCha8`): ロジック用の決定論的乱数 (リプレイ再現可能)

#[derive(Default)]
pub struct RngManager;

impl RngManager {
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}
