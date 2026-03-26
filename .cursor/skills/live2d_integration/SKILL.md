---
name: live2d_integration
description: Live2D Cubism SDK integration skill (FFI, render-to-texture, lip sync, motion)
---

# Live2D Integration Skill
## Overview
Integrate Live2D Cubism Native SDK (C++) into Rust via FFI. Use Filament render-to-texture for scene composition, support automatic lip sync via audio RMS analysis, and control expressions/motions via `.motion3.json`.

## Architecture
```text
Live2D Cubism SDK (C++)
    -> FFI (unsafe wrapper)
engine_core::ffi::live2d_sys  <- low-level bindings
    -> safe wrapper
engine_core::live2d           <- public API
    ->
Filament Render-to-Texture    <- composite into game scene
```

## FFI Bindings
### `live2d_sys` module
```rust
// crates/engine_core/src/ffi/live2d_sys.rs

#[link(name = "Live2DCubismCore")]
extern "C" {
    // SAFETY: SDK init function. Called once at application start.
    pub fn csmReviveMocInPlace(address: *mut u8, size: u32) -> *mut CsmMoc;
    pub fn csmGetSizeofModel(moc: *const CsmMoc) -> u32;
    pub fn csmInitializeModelInPlace(moc: *const CsmMoc, address: *mut u8, size: u32) -> *mut CsmModel;
    // ... other SDK functions
}
```

### Safe wrapper
```rust
// crates/engine_core/src/live2d/model.rs

pub struct Live2DModel {
    moc: NonNull<CsmMoc>,
    model: NonNull<CsmModel>,
    // ...
}

impl Live2DModel {
    pub fn load(data: &[u8]) -> Result<Self, Live2DError> {
        // SAFETY: Data size validated. Memory alignment guaranteed.
        unsafe {
            // Restore MOC and initialize model
        }
    }

    pub fn set_parameter(&mut self, id: &str, value: f32) {
        // Update parameters (mouth, eyes, body, etc.)
    }

    pub fn update(&mut self, dt: f32) {
        // Physics and motion update
    }
}

impl Drop for Live2DModel {
    fn drop(&mut self) {
        // SAFETY: Model lifetime is managed by this struct.
        unsafe { /* free memory */ }
    }
}
```

## Render-to-Texture Integration
1. Write Live2D rendering output to a dedicated texture.
2. Map the texture onto a quad in Filament 3D space.
3. Benefit from IBL / bloom / other post-processing effects.

```rust
pub struct Live2DRenderer {
    render_target: filament::RenderTarget,
    quad_entity: Entity,  // Quad in Filament space
}
```

## Automatic Lip Sync
Analyze audio RMS (root mean square) in real time and map to Live2D mouth parameters.
```rust
pub fn calculate_lip_sync(audio_samples: &[f32], frame_size: usize) -> f32 {
    let rms = (audio_samples.iter()
        .take(frame_size)
        .map(|s| s * s)
        .sum::<f32>() / frame_size as f32)
        .sqrt();
    // Map RMS to 0.0-1.0
    (rms * 10.0).clamp(0.0, 1.0)
}
```

## Motion and Expression Control
- `.motion3.json` playback: motion transitions with fade-in/out.
- Expression blending: weighted blend of multiple expressions.
- Script tag control:

```text
[chara id="reimu" motion="happy" expression="smile"]
[chara id="reimu" lip_sync="on"]
```

## ECS Integration
```rust
// Live2D component
pub struct Live2DComponent {
    pub model: Live2DModel,
    pub current_motion: Option<MotionId>,
    pub lip_sync_enabled: bool,
}

// Live2D update system
pub fn live2d_update_system(/* ECS query */) {
    // Physics, motion update, lip sync application
}
```

## Notes
- Live2D Cubism SDK requires a commercial license.
- All `unsafe` code is isolated in `ffi/live2d_sys.rs`.
- Every `unsafe` block must have a `// SAFETY:` comment.
