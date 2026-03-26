---
name: minigame_plugin
description: Plugin implementation skill for minigames (2D/3D action, RPG, puzzle, RTS)
---

# Minigame Plugin Skill
## Overview
This engine inserts minigames into ADV parts, inspired by Koihime Musou and TWINKLE CRUSADERS. Each minigame is an independent plugin implementing the `Scene` trait.

## Supported Minigame Types
| Type | Description | Example |
|--------|------|-----|
| 2D Action | Side-scrolling, shooting, etc. | Combat section |
| 3D Action | 3D combat/exploration | Boss fight |
| RPG | Turn-based / real-time battle | War section |
| Puzzle | Logic, matching, etc. | Quiz/riddle section |
| RTS | Real-time strategy | Army command |

## Architecture
```text
ADV Scene
  |- Script execution detects minigame command
  |- Push MiniGame Scene
  |     |- Dedicated ECS components/systems
  |     |- Dedicated assets
  |     '- Return MiniGameResult on finish
  '- Pop back to ADV Scene -> continue script with result
```

## Implementation Steps
### 1. Create minigame module
```rust
// crates/engine_core/src/scene/minigame/battle_scene.rs

use crate::scene::{Scene, SceneContext, RenderContext, SceneTransition};

pub struct BattleScene {
    config: BattleConfig,
    state: BattleState,
}

/// Minigame result
pub enum MiniGameResult {
    Victory,
    Defeat,
    Draw,
    Score(u32),
}
```

### 2. Implement Scene trait
```rust
impl Scene for BattleScene {
    fn on_enter(&mut self, ctx: &mut SceneContext) {
        // Register minigame-specific ECS components/systems
        // Play dedicated BGM
        // Setup LogicRng (ChaCha8) state
    }

    fn update(&mut self, ctx: &mut SceneContext, dt: f32) -> SceneTransition {
        // Update game logic
        // Check end conditions
        if self.state.is_finished() {
            ctx.set_minigame_result(self.state.result());
            return SceneTransition::Pop;
        }
        SceneTransition::None
    }

    fn on_exit(&mut self, ctx: &mut SceneContext) {
        // Cleanup minigame-specific ECS resources
    }

    fn required_assets(&self) -> Vec<AssetDescriptor> {
        // Minigame-specific assets
        vec![]
    }

    fn render(&self, ctx: &RenderContext) {
        // Minigame-specific rendering
    }
}
```

### 3. ADV part integration
Launch minigame via script tags:
```text
[minigame type="battle" config="battle_01"]
[if result=="victory"]
  [text]Victory![/text]
[else]
  [text]Defeat...[/text]
[/if]
```

### 4. RNG management
- Use `LogicRng` (ChaCha8) for deterministic game logic.
- Include LogicRng seed in save data on scene entry (replay reproducibility).
- Use `VisualRng` (Xoshiro256++) for visual effects (particles, etc.).

## Directory Structure
```text
crates/engine_core/src/scene/minigame/
|- mod.rs              # Common minigame interface
|- battle_scene.rs     # 2D/3D action battle
|- rpg_battle.rs       # RPG battle
|- puzzle_scene.rs     # Puzzle
'- rts_scene.rs        # RTS
```

## Test Strategy
- Exhaustive test coverage for win/lose/draw conditions per minigame.
- Test replay reproducibility with fixed LogicRng seeds.
- Integration tests for result handoff to ADV flow.
