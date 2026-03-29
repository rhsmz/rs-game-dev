//! Filament C API bindings（雛形）。
//!
//! 現時点では Filament ライブラリ本体が無い前提でもコンパイル可能な
//! `extern "C"` 宣言・opaque 型のみを提供します。
//!
//! 実際に呼び出すラッパー（安全 API）は、以降の `renderer/` 側で実装します。

#![allow(dead_code)]
#![allow(non_camel_case_types)]

/// Filament `Engine` の opaque 型。
#[repr(C)]
pub struct Engine {
    _private: [u8; 0],
}

/// Filament `Renderer` の opaque 型。
#[repr(C)]
pub struct Renderer {
    _private: [u8; 0],
}

/// Filament `Scene` の opaque 型。
#[repr(C)]
pub struct Scene {
    _private: [u8; 0],
}

/// Filament `View` の opaque 型。
#[repr(C)]
pub struct View {
    _private: [u8; 0],
}

/// Filament `SwapChain` の opaque 型。
#[repr(C)]
pub struct SwapChain {
    _private: [u8; 0],
}

// NOTE: 実際の C API シンボル名は Filament のバージョン/ビルド設定に依存します。
//       ここでは後続実装のための足場として、代表的な関数名を宣言します。
unsafe extern "C" {
    /// Filament Engine を生成する。
    pub(crate) fn Engine_create() -> *mut Engine;

    /// Filament Engine を破棄する。
    pub(crate) fn Engine_destroy(engine: *mut Engine);

    /// Filament Scene を生成する。
    pub(crate) fn Scene_create(engine: *mut Engine) -> *mut Scene;

    /// Filament Scene を破棄する。
    pub(crate) fn Scene_destroy(scene: *mut Scene);

    /// Filament View を生成する。
    pub(crate) fn View_create(engine: *mut Engine) -> *mut View;

    /// Filament View を破棄する。
    pub(crate) fn View_destroy(view: *mut View);

    /// Filament Renderer を生成する。
    pub(crate) fn Renderer_create(engine: *mut Engine) -> *mut Renderer;

    /// Filament Renderer を破棄する。
    pub(crate) fn Renderer_destroy(renderer: *mut Renderer);

    /// Filament `SwapChain` を生成する。
    pub(crate) fn SwapChain_create(engine: *mut Engine) -> *mut SwapChain;

    /// Filament `SwapChain` を破棄する。
    pub(crate) fn SwapChain_destroy(swap_chain: *mut SwapChain);

    /// `View` に `Scene` を関連付ける。
    pub(crate) fn View_setScene(view: *mut View, scene: *mut Scene);

    /// フレーム描画を開始する。成功時は非ゼロ（C の `bool` 相当）。
    pub(crate) fn Renderer_beginFrame(swap_chain: *mut SwapChain, renderer: *mut Renderer) -> bool;

    /// フレームを終了しスワップチェーンへ提出する。
    pub(crate) fn Renderer_endFrame(renderer: *mut Renderer);

    /// スワップチェーンのバックバッファをリサイズする。
    pub(crate) fn SwapChain_resize(swap_chain: *mut SwapChain, width: u32, height: u32);

    /// 単一 `View` を描画する（`beginFrame` / `endFrame` の間で呼ぶ）。
    pub(crate) fn Renderer_render(renderer: *mut Renderer, view: *mut View);
}
