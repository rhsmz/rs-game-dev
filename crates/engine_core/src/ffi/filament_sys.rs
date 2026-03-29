//! Filament 向け **安定 C ABI**（`engine_core` 専用シム）。
//!
//! [Google Filament](https://github.com/google/filament) の公開 API は C++ のみであり、
//! `Engine_create` のようなプレーン C シンボルは公式バイナリには含まれません。
//! ここに宣言している名前は **Rust 側の約束**であり、実体はスタブまたは C++ ブリッジが
//! `extern "C"` でエクスポートします（公式エクスポートとの一致確認は `filament_stub.c` /
//! `filament_bridge.cpp` とこのファイルの突き合わせで行う）。
//!
//! | リンク方法 | 実装 |
//! |------------|------|
//! | `FILAMENT_LIB_DIR` **未設定** | `crates/engine_core/stub/filament_stub.c`（開発用スタブ、GPU 非依存） |
//! | `FILAMENT_LIB_DIR` **設定** | `crates/engine_core/stub/filament_bridge.cpp` が公式 C++ API を呼び出し、下記シンボルをエクスポート |
//!
//! **Windows MSVC:** プリビルトの `mdd`（デバッグ CRT）を `cargo test` のデバッグプロファイルで
//! そのままリンクすると、`__imp__CrtDbgReport` など CRT 関連の未解決が出ることがあります。
//! 実 GPU での検証は `lib/x86_64/md`（または `mt`）と `cargo test --release`、または CI 向けに
//! スタブ（`FILAMENT_LIB_DIR` 未設定）のデバッグ `cargo test` を使うと安定します。
//!
//! 破棄系は公式の [`Engine::destroy`](https://github.com/google/filament/blob/main/filament/include/filament/Engine.h) に合わせ、
//! 対象リソースを所有している `Engine*` を必ず渡します（スタブ・ブリッジ共通 ABI）。

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

/// `Entity` + `Camera` を束ねるブリッジ内部ハンドル（不透明）。
#[repr(C)]
pub struct ViewCameraBinding {
    _private: [u8; 0],
}

unsafe extern "C" {
    /// `filament::Engine::create()` に相当。
    pub(crate) fn Engine_create() -> *mut Engine;

    /// `filament::Engine::destroy(Engine*)` に相当。
    pub(crate) fn Engine_destroy(engine: *mut Engine);

    /// `engine->createScene()` に相当。
    pub(crate) fn Scene_create(engine: *mut Engine) -> *mut Scene;

    /// `engine->destroy(Scene*)` に相当。
    pub(crate) fn Scene_destroy(engine: *mut Engine, scene: *mut Scene);

    /// `engine->createView()` に相当。
    pub(crate) fn View_create(engine: *mut Engine) -> *mut View;

    /// `engine->destroy(View*)` に相当。
    pub(crate) fn View_destroy(engine: *mut Engine, view: *mut View);

    /// `engine->createRenderer()` に相当。
    pub(crate) fn Renderer_create(engine: *mut Engine) -> *mut Renderer;

    /// `engine->destroy(Renderer*)` に相当。
    pub(crate) fn Renderer_destroy(engine: *mut Engine, renderer: *mut Renderer);

    /// ヘッドレス `engine->createSwapChain(w, h)`（スモーク用。ウィンドウ連携は後続）。
    pub(crate) fn SwapChain_create(engine: *mut Engine) -> *mut SwapChain;

    /// `engine->destroy(SwapChain*)` に相当。
    pub(crate) fn SwapChain_destroy(engine: *mut Engine, swap_chain: *mut SwapChain);

    /// `view->setScene(scene)` に相当。
    pub(crate) fn View_setScene(view: *mut View, scene: *mut Scene);

    /// `renderer->beginFrame(swapChain)` に相当。
    pub(crate) fn Renderer_beginFrame(swap_chain: *mut SwapChain, renderer: *mut Renderer) -> bool;

    /// `renderer->endFrame()` に相当。
    pub(crate) fn Renderer_endFrame(renderer: *mut Renderer);

    /// 公式に直接対応する API が無いため、ブリッジでは no-op。ビューポート更新は別 API で行う予定。
    pub(crate) fn SwapChain_resize(swap_chain: *mut SwapChain, width: u32, height: u32);

    /// `renderer->render(view)` に相当。
    pub(crate) fn Renderer_render(renderer: *mut Renderer, view: *mut View);

    /// Game View 用透視カメラを生成し `View` に関連付ける。
    pub(crate) fn ViewCamera_create_game(
        engine: *mut Engine,
        view: *mut View,
        width: u32,
        height: u32,
        fov_y_radians: f32,
        near_plane: f32,
        far_plane: f32,
    ) -> *mut ViewCameraBinding;

    /// UI View 用オルソカメラを生成し、`TRANSLUCENT` / ポストプロセスオフ等を設定する。
    pub(crate) fn ViewCamera_create_ui(
        engine: *mut Engine,
        view: *mut View,
        width: u32,
        height: u32,
        near_plane: f32,
        far_plane: f32,
    ) -> *mut ViewCameraBinding;

    pub(crate) fn ViewCamera_update_game(
        engine: *mut Engine,
        view: *mut View,
        binding: *mut ViewCameraBinding,
        width: u32,
        height: u32,
        fov_y_radians: f32,
        near_plane: f32,
        far_plane: f32,
    );

    pub(crate) fn ViewCamera_update_ui(
        engine: *mut Engine,
        view: *mut View,
        binding: *mut ViewCameraBinding,
        width: u32,
        height: u32,
        near_plane: f32,
        far_plane: f32,
    );

    /// `View` からカメラを切り離し、`Camera` コンポーネントと Entity を破棄する。
    pub(crate) fn ViewCamera_destroy(
        engine: *mut Engine,
        view: *mut View,
        binding: *mut ViewCameraBinding,
    );
}
