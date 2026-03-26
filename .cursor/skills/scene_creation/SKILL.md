---
name: scene_creation
description: Skill for implementing the Scene trait and creating scene transitions
---

# Scene Creation Skill
## Overview
All game parts in this engine (ADV part, minigames, title screen, etc.) implement the `Scene` trait. This skill defines creation steps and best practices for new scenes.

## Scene Trait Definition
```rust
use crate::asset::AssetDescriptor;
use crate::scene::{SceneContext, RenderContext, SceneTransition};

pub trait Scene: Send + Sync {
    /// Initialize scene (called after asset loading completes)
    fn on_enter(&mut self, ctx: &mut SceneContext);

    /// Per-frame update. Returns the next scene transition.
    fn update(&mut self, ctx: &mut SceneContext, dt: f32) -> SceneTransition;

    /// Issue render commands
    fn render(&self, ctx: &RenderContext);

    /// Cleanup when leaving the scene
    fn on_exit(&mut self, ctx: &mut SceneContext);

    /// Assets required by this scene (for preloading)
    fn required_assets(&self) -> Vec<AssetDescriptor>;
}
```

## SceneTransition Enum
```rust
pub enum SceneTransition {
    /// Continue current scene
    None,
    /// Transition to specified scene
    Push(Box<dyn Scene>),
    /// End current scene and return to previous scene
    Pop,
    /// Discard current scene and replace with specified scene
    Replace(Box<dyn Scene>),
    /// Quit game
    Quit,
}
```

## New Scene Creation Steps

### 1. Create file

Create a scene module under `crates/engine_core/src/scene/`.

```rust
// crates/engine_core/src/scene/title_scene.rs

pub struct TitleScene {
    // Scene-specific state
}

impl TitleScene {
    pub fn new() -> Self {
        Self { }
    }
}

impl Scene for TitleScene {
    fn on_enter(&mut self, ctx: &mut SceneContext) {
        // Play BGM, setup UI, etc.
    }

    fn update(&mut self, ctx: &mut SceneContext, dt: f32) -> SceneTransition {
        // Check input -> decide scene transition
        SceneTransition::None
    }

    fn render(&self, ctx: &RenderContext) {
        // Issue render commands
    }

    fn on_exit(&mut self, ctx: &mut SceneContext) {
        // Release resources
    }

    fn required_assets(&self) -> Vec<AssetDescriptor> {
        vec![
            // List required assets
        ]
    }
}
```

### 2. Register module
Add it to `crates/engine_core/src/scene/mod.rs`.

### 3. Tests
- Verify lifecycle: `on_enter` -> `update` -> `on_exit`.
- Verify each `SceneTransition` variant is handled correctly.

## Scene Transition Flow
```
current_scene.on_exit()
    |
full asset purge
    |
new_scene.required_assets() -> async load
    |
after loading, new_scene.on_enter()
    |
main loop calls new_scene.update() / render()
```

## Best Practices
- Register scene-specific ECS components/systems in `on_enter`, remove them in `on_exit`.
- Declare heavy assets in `required_assets()` and load asynchronously on a loading screen.
- Pass data between scenes via message passing through `SceneContext`.
