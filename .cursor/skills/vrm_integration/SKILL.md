---
name: vrm_integration
description: VRM/FBX 3D model integration skill (SpringBone, BlendShape, GLB conversion)
---

# VRM/FBX Integration Skill
## Overview
Integrate VRM (VirtualCast standard) and FBX 3D models into the Filament renderer. Support SpringBone secondary-motion simulation, BlendShape-based expression control, and editor-side GLB preprocessing.

## Supported Formats
| Format | Use | Handling |
|-------------|------|------|
| VRM (.vrm) | Virtual character | Direct load with SpringBone support |
| FBX (.fbx) | Generic 3D model | Pre-convert to GLB in editor |
| GLB (.glb) | Runtime format | Fast load |

## VRM Loader
```rust
// crates/engine_core/src/vrm/loader.rs

pub struct VrmModel {
    pub mesh: FilamentMesh,
    pub skeleton: Skeleton,
    pub spring_bones: Vec<SpringBone>,
    pub blend_shapes: BlendShapeProxy,
    pub animations: Vec<Animation>,
}

impl VrmModel {
    pub fn load(data: &[u8]) -> Result<Self, VrmError> {
        // Parse glTF 2.0 -> extract VRM extension data
        // Mesh -> convert to Filament VertexBuffer / IndexBuffer
        // Build skeleton
        // Extract SpringBone parameters
        // Load BlendShape definitions
        Ok(Self { /* ... */ })
    }
}
```

## SpringBone Simulation
Per-frame simulation of VRM secondary motion (hair, clothing, accessories).
```rust
// crates/engine_core/src/vrm/spring_bone.rs

pub struct SpringBone {
    pub stiffness: f32,     // stiffness
    pub gravity_power: f32, // gravity
    pub drag_force: f32,    // drag
    pub hit_radius: f32,    // collision radius
    bones: Vec<SpringBoneNode>,
}

impl SpringBone {
    pub fn update(&mut self, dt: f32, center: &Transform) {
        for bone in &mut self.bones {
            // Verlet integration physics
            // Collider collision detection
            // Bone position update
        }
    }
}
```

## BlendShape (MorphTarget)
Map VRM `BlendShapeProxy` to Filament MorphTargets.
```rust
pub struct BlendShapeProxy {
    presets: HashMap<BlendShapePreset, Vec<BlendShapeBinding>>,
}

#[derive(Hash, Eq, PartialEq)]
pub enum BlendShapePreset {
    Joy,
    Angry,
    Sorrow,
    Fun,
    Blink,
    BlinkL,
    BlinkR,
    // VRM standard presets
}

impl BlendShapeProxy {
    pub fn set_value(&mut self, preset: BlendShapePreset, weight: f32) {
        // Update MorphTarget weights
    }
}
```

## FBX -> GLB Conversion
Pre-convert FBX to GLB in the editor (`script_editor`); runtime loads GLB only.
```text
[script_editor]
  FBX import -> preserve animations -> GLB export
      |
[game_player / engine_core]
  Fast GLB load
```

## ECS Integration
```rust
// 3D model component
pub struct Model3DComponent {
    pub model: VrmModel,
    pub current_animation: Option<AnimationId>,
    pub blend_shape_state: HashMap<BlendShapePreset, f32>,
}

// SpringBone update system
pub fn spring_bone_update_system(/* ECS query */) {
    // Physics update for all VRM SpringBones
}

// BlendShape update system
pub fn blend_shape_update_system(/* ECS query */) {
    // Apply expression parameters to MorphTargets
}
```

## Script Integration
```text
[chara3d id="warrior" animation="idle"]
[chara3d id="warrior" expression="joy" weight="0.8"]
[chara3d id="warrior" animation="attack" blend="0.3"]
```

## Notes
- Follow VRM 1.0 specification.
- If SpringBone is expensive, reduce update frequency via LOD.
- FBX->GLB conversion is a `script_editor` feature (runtime does not handle FBX).
