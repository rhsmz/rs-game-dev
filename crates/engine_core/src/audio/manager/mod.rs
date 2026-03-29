//! [`GameAudioManager`] の実装切り替え。
//!
//! - `audio-kira` 有効: `kira` 経由で実出力。
//! - 無効: スタブ（デバイス非初期化）。`cargo test -p engine_core --no-default-features` 向け。

#[cfg(feature = "audio-kira")]
mod kira;
#[cfg(feature = "audio-kira")]
pub use kira::GameAudioManager;

#[cfg(not(feature = "audio-kira"))]
mod stub;
#[cfg(not(feature = "audio-kira"))]
pub use stub::GameAudioManager;
