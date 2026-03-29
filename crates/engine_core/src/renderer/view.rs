//! ビュー（Game View / UI View）雛形。
//!
//! `unsafe impl Send/Sync` の前提と禁止事項は [`SAFETY.md`](../../SAFETY.md) を参照。

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
    engine: NonNull<filament_sys::Engine>,

    #[cfg(feature = "filament")]
    view: NonNull<filament_sys::View>,

    #[cfg(feature = "filament")]
    scene: NonNull<filament_sys::Scene>,

    #[cfg(feature = "filament")]
    camera_binding: NonNull<filament_sys::ViewCameraBinding>,
}

/// UI View（Layer 1: オルソ投影。深度の無効化は主に `MaterialInstance` で行う）。
///
/// Filament の `View` には深度テスト／深度書き込みを一括で切る API がないため、
/// ここではスワップチェーンへの合成向けに `TRANSLUCENT`・ポストプロセスオフなどを設定する。
pub struct UiView {
    pub camera: CameraOrtho,

    #[cfg(feature = "filament")]
    engine: NonNull<filament_sys::Engine>,

    #[cfg(feature = "filament")]
    view: NonNull<filament_sys::View>,

    #[cfg(feature = "filament")]
    scene: NonNull<filament_sys::Scene>,

    #[cfg(feature = "filament")]
    camera_binding: NonNull<filament_sys::ViewCameraBinding>,
}

#[cfg(feature = "filament")]
fn framebuffer_dims(engine: &RenderEngine) -> (u32, u32) {
    let (w, h) = engine.framebuffer_dimensions();
    (w.max(1), h.max(1))
}

#[cfg(feature = "filament")]
fn ui_clip_planes(ortho: CameraOrtho) -> (f32, f32) {
    let n = ortho.near;
    let mut f = ortho.far;
    if !n.is_finite() || !f.is_finite() {
        return (0.0, 1.0);
    }
    if f <= n {
        f = n + 1e-3;
    }
    (n, f)
}

impl GameView {
    /// Game View を初期化する。
    #[allow(clippy::missing_errors_doc)]
    #[allow(clippy::missing_const_for_fn)]
    pub fn new(engine: &RenderEngine) -> anyhow::Result<Self> {
        #[cfg(feature = "filament")]
        {
            let engine_nn = engine.filament_engine();
            let engine_ptr = engine_nn.as_ptr();
            let (w, h) = framebuffer_dims(engine);

            // SAFETY: `engine_ptr` は有効な Filament Engine。
            let scene_ptr = unsafe { filament_sys::Scene_create(engine_ptr) };
            let scene = NonNull::new(scene_ptr)
                .ok_or_else(|| anyhow!("Filament Scene_create returned null"))?;

            let view_ptr = unsafe { filament_sys::View_create(engine_ptr) };
            let Some(view) = NonNull::new(view_ptr) else {
                unsafe { filament_sys::Scene_destroy(engine_ptr, scene.as_ptr()) };
                return Err(anyhow!("Filament View_create returned null"));
            };

            unsafe {
                filament_sys::View_setScene(view.as_ptr(), scene.as_ptr());
            }

            let camera = Camera3D { fov: 60.0_f32.to_radians(), near: 0.1, far: 1000.0 };
            let binding_ptr = unsafe {
                filament_sys::ViewCamera_create_game(
                    engine_ptr,
                    view.as_ptr(),
                    w,
                    h,
                    camera.fov,
                    camera.near,
                    camera.far,
                )
            };
            let Some(camera_binding) = NonNull::new(binding_ptr) else {
                unsafe {
                    filament_sys::View_destroy(engine_ptr, view.as_ptr());
                    filament_sys::Scene_destroy(engine_ptr, scene.as_ptr());
                }
                return Err(anyhow!("Filament ViewCamera_create_game returned null"));
            };

            Ok(Self { camera, engine: engine_nn, view, scene, camera_binding })
        }

        #[cfg(not(feature = "filament"))]
        {
            let _ = engine;
            Ok(Self { camera: Camera3D { fov: 60.0_f32.to_radians(), near: 0.1, far: 1000.0 } })
        }
    }

    /// `RenderEngine::resize` 後の解像度へ投影・ビューポートを合わせる（毎フレーム呼んでもよい）。
    #[cfg(feature = "filament")]
    #[allow(clippy::missing_const_for_fn)]
    pub fn sync_framebuffer_from_engine(&mut self, engine: &RenderEngine) {
        let (w, h) = framebuffer_dims(engine);
        let cam = self.camera;
        // SAFETY: 各ポインタは `GameView` 生成時に有効と検証済み。
        unsafe {
            filament_sys::ViewCamera_update_game(
                self.engine.as_ptr(),
                self.view.as_ptr(),
                self.camera_binding.as_ptr(),
                w,
                h,
                cam.fov,
                cam.near,
                cam.far,
            );
        }
    }

    #[cfg(not(feature = "filament"))]
    #[allow(clippy::missing_const_for_fn, clippy::unused_self)]
    pub fn sync_framebuffer_from_engine(&mut self, _engine: &RenderEngine) {}

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
            // SAFETY: `View` の `Camera` を先に切り離してから `View` / `Scene` を破棄する。
            unsafe {
                filament_sys::ViewCamera_destroy(
                    self.engine.as_ptr(),
                    self.view.as_ptr(),
                    self.camera_binding.as_ptr(),
                );
                filament_sys::View_destroy(self.engine.as_ptr(), self.view.as_ptr());
                filament_sys::Scene_destroy(self.engine.as_ptr(), self.scene.as_ptr());
            }
        }
    }
}

impl UiView {
    /// UI View を初期化する。
    #[allow(clippy::missing_errors_doc)]
    #[allow(clippy::missing_const_for_fn)]
    pub fn new(engine: &RenderEngine) -> anyhow::Result<Self> {
        #[cfg(feature = "filament")]
        {
            let engine_nn = engine.filament_engine();
            let engine_ptr = engine_nn.as_ptr();
            let (w, h) = framebuffer_dims(engine);

            let scene_ptr = unsafe { filament_sys::Scene_create(engine_ptr) };
            let scene = NonNull::new(scene_ptr)
                .ok_or_else(|| anyhow!("Filament Scene_create returned null"))?;

            let view_ptr = unsafe { filament_sys::View_create(engine_ptr) };
            let Some(view) = NonNull::new(view_ptr) else {
                unsafe { filament_sys::Scene_destroy(engine_ptr, scene.as_ptr()) };
                return Err(anyhow!("Filament View_create returned null"));
            };

            unsafe {
                filament_sys::View_setScene(view.as_ptr(), scene.as_ptr());
            }

            let camera = CameraOrtho { near: 0.0, far: 1.0 };
            let (nz, fz) = ui_clip_planes(camera);
            let binding_ptr = unsafe {
                filament_sys::ViewCamera_create_ui(engine_ptr, view.as_ptr(), w, h, nz, fz)
            };
            let Some(camera_binding) = NonNull::new(binding_ptr) else {
                unsafe {
                    filament_sys::View_destroy(engine_ptr, view.as_ptr());
                    filament_sys::Scene_destroy(engine_ptr, scene.as_ptr());
                }
                return Err(anyhow!("Filament ViewCamera_create_ui returned null"));
            };

            Ok(Self { camera, engine: engine_nn, view, scene, camera_binding })
        }

        #[cfg(not(feature = "filament"))]
        {
            let _ = engine;
            Ok(Self { camera: CameraOrtho { near: 0.0, far: 1.0 } })
        }
    }

    #[cfg(feature = "filament")]
    #[allow(clippy::missing_const_for_fn)]
    pub fn sync_framebuffer_from_engine(&mut self, engine: &RenderEngine) {
        let (w, h) = framebuffer_dims(engine);
        let (nz, fz) = ui_clip_planes(self.camera);
        // SAFETY: 各ポインタは `UiView` 生成時に有効と検証済み。
        unsafe {
            filament_sys::ViewCamera_update_ui(
                self.engine.as_ptr(),
                self.view.as_ptr(),
                self.camera_binding.as_ptr(),
                w,
                h,
                nz,
                fz,
            );
        }
    }

    #[cfg(not(feature = "filament"))]
    #[allow(clippy::missing_const_for_fn, clippy::unused_self)]
    pub fn sync_framebuffer_from_engine(&mut self, _engine: &RenderEngine) {}

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
                filament_sys::ViewCamera_destroy(
                    self.engine.as_ptr(),
                    self.view.as_ptr(),
                    self.camera_binding.as_ptr(),
                );
                filament_sys::View_destroy(self.engine.as_ptr(), self.view.as_ptr());
                filament_sys::Scene_destroy(self.engine.as_ptr(), self.scene.as_ptr());
            }
        }
    }
}
