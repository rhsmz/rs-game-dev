//! ビュー（Game View / UI View）雛形。

use crate::ecs::component::Component;
#[cfg(feature = "filament")]
use crate::ffi::filament_sys;
use crate::renderer::RenderEngine;

#[cfg(feature = "filament")]
use std::ptr::NonNull;

/// 3D 視点用カメラ情報。
#[derive(Debug, Clone, Copy)]
pub struct Camera3D {
    pub fov: f32,
    pub near: f32,
    pub far: f32,
}

impl Component for Camera3D {}

/// 2D UI 用（疑似）オルソカメラ情報。
#[derive(Debug, Clone, Copy)]
pub struct CameraOrtho {
    pub near: f32,
    pub far: f32,
}

/// Game View（Layer 0: Perspective + PBR/IBL 想定）。
pub struct GameView {
    #[cfg(feature = "filament")]
    view: Option<NonNull<filament_sys::View>>,
    #[cfg(feature = "filament")]
    scene: Option<NonNull<filament_sys::Scene>>,

    pub camera: Camera3D,
}

/// UI View（Layer 1: Ortho + 深度テスト無効想定）。
pub struct UiView {
    #[cfg(feature = "filament")]
    view: Option<NonNull<filament_sys::View>>,
    #[cfg(feature = "filament")]
    scene: Option<NonNull<filament_sys::Scene>>,

    pub camera: CameraOrtho,
}

impl GameView {
    /// Game View を初期化する（雛形）。
    #[allow(clippy::missing_const_for_fn)]
    #[allow(clippy::missing_errors_doc)]
    pub fn new(_engine: &RenderEngine) -> anyhow::Result<Self> {
        #[cfg(feature = "filament")]
        {
            // TODO: Filament 依存の `View_create` / `Scene_create` 等をここで行う。
            //       現時点では smoke test 用に camera 情報のみ初期化する。
            let _ = _engine;
            Ok(Self {
                camera: Camera3D { fov: 60.0_f32.to_radians(), near: 0.1, far: 1000.0 },
                view: None,
                scene: None,
            })
        }

        #[cfg(not(feature = "filament"))]
        {
            Ok(Self { camera: Camera3D { fov: 60.0_f32.to_radians(), near: 0.1, far: 1000.0 } })
        }
    }
}

// SAFETY: Filament ポインタはメインスレッドでのみ操作する前提。
// ECS リソース登録のために Send + Sync を明示する。
#[cfg(feature = "filament")]
unsafe impl Send for GameView {}
#[cfg(feature = "filament")]
unsafe impl Sync for GameView {}
#[cfg(feature = "filament")]
unsafe impl Send for UiView {}
#[cfg(feature = "filament")]
unsafe impl Sync for UiView {}

impl UiView {
    /// UI View を初期化する（雛形）。
    #[allow(clippy::missing_const_for_fn)]
    #[allow(clippy::missing_errors_doc)]
    pub fn new(_engine: &RenderEngine) -> anyhow::Result<Self> {
        #[cfg(feature = "filament")]
        {
            // TODO: UI 用 Ortho 設定を Filament API に反映する。
            //       現時点では smoke test 用に camera 情報のみ初期化する。
            let _ = _engine;
            Ok(Self { camera: CameraOrtho { near: 0.0, far: 1.0 }, view: None, scene: None })
        }

        #[cfg(not(feature = "filament"))]
        {
            Ok(Self { camera: CameraOrtho { near: 0.0, far: 1.0 } })
        }
    }
}
