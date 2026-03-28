//! ビュー（Game View / UI View）雛形。

use crate::ecs::component::Component;
#[cfg(feature = "filament")]
use crate::ffi::filament_sys;
use crate::renderer::RenderEngine;

#[cfg(feature = "filament")]
use anyhow::anyhow;

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
    pub camera: Camera3D,

    #[cfg(feature = "filament")]
    view: NonNull<filament_sys::View>,

    #[cfg(feature = "filament")]
    scene: NonNull<filament_sys::Scene>,
}

/// UI View（Layer 1: Ortho + 深度テスト無効想定）。
pub struct UiView {
    pub camera: CameraOrtho,

    #[cfg(feature = "filament")]
    view: NonNull<filament_sys::View>,

    #[cfg(feature = "filament")]
    scene: NonNull<filament_sys::Scene>,
}

impl GameView {
    /// Game View を初期化する。
    #[allow(clippy::missing_errors_doc)]
    pub fn new(engine: &RenderEngine) -> anyhow::Result<Self> {
        #[cfg(feature = "filament")]
        {
            let engine_ptr = engine.filament_engine().as_ptr();

            // SAFETY: `engine_ptr` は有効な Filament Engine。
            let scene_ptr = unsafe { filament_sys::Scene_create(engine_ptr) };
            let scene = NonNull::new(scene_ptr)
                .ok_or_else(|| anyhow!("Filament Scene_create returned null"))?;

            let view_ptr = unsafe { filament_sys::View_create(engine_ptr) };
            let Some(view) = NonNull::new(view_ptr) else {
                unsafe { filament_sys::Scene_destroy(scene.as_ptr()) };
                return Err(anyhow!("Filament View_create returned null"));
            };

            unsafe {
                filament_sys::View_setScene(view.as_ptr(), scene.as_ptr());
            }

            Ok(Self {
                camera: Camera3D { fov: 60.0_f32.to_radians(), near: 0.1, far: 1000.0 },
                view,
                scene,
            })
        }

        #[cfg(not(feature = "filament"))]
        {
            let _ = engine;
            Ok(Self { camera: Camera3D { fov: 60.0_f32.to_radians(), near: 0.1, far: 1000.0 } })
        }
    }

    /// Filament の `View` ハンドル。
    #[cfg(feature = "filament")]
    #[must_use]
    pub(crate) const fn filament_view_ptr(&self) -> NonNull<filament_sys::View> {
        self.view
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

impl Drop for GameView {
    fn drop(&mut self) {
        #[cfg(feature = "filament")]
        {
            // SAFETY: `View` を先に破棄し、その後 `Scene` を破棄する。
            unsafe {
                filament_sys::View_destroy(self.view.as_ptr());
                filament_sys::Scene_destroy(self.scene.as_ptr());
            }
        }
    }
}

impl UiView {
    /// UI View を初期化する。
    #[allow(clippy::missing_errors_doc)]
    pub fn new(engine: &RenderEngine) -> anyhow::Result<Self> {
        #[cfg(feature = "filament")]
        {
            let engine_ptr = engine.filament_engine().as_ptr();

            let scene_ptr = unsafe { filament_sys::Scene_create(engine_ptr) };
            let scene = NonNull::new(scene_ptr)
                .ok_or_else(|| anyhow!("Filament Scene_create returned null"))?;

            let view_ptr = unsafe { filament_sys::View_create(engine_ptr) };
            let Some(view) = NonNull::new(view_ptr) else {
                unsafe { filament_sys::Scene_destroy(scene.as_ptr()) };
                return Err(anyhow!("Filament View_create returned null"));
            };

            unsafe {
                filament_sys::View_setScene(view.as_ptr(), scene.as_ptr());
            }

            // オルソ投影・深度無効は後続で Filament API に接続する（現状はメタデータのみ保持）。
            Ok(Self { camera: CameraOrtho { near: 0.0, far: 1.0 }, view, scene })
        }

        #[cfg(not(feature = "filament"))]
        {
            let _ = engine;
            Ok(Self { camera: CameraOrtho { near: 0.0, far: 1.0 } })
        }
    }

    #[cfg(feature = "filament")]
    #[must_use]
    pub(crate) const fn filament_view_ptr(&self) -> NonNull<filament_sys::View> {
        self.view
    }
}

impl Drop for UiView {
    fn drop(&mut self) {
        #[cfg(feature = "filament")]
        {
            unsafe {
                filament_sys::View_destroy(self.view.as_ptr());
                filament_sys::Scene_destroy(self.scene.as_ptr());
            }
        }
    }
}
