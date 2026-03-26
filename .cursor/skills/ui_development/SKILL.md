---
name: ui_development
description: MVVM + signal-driven UI development skill (Taffy layout, SDF text, Glassmorphism)
---

# UI Development Skill
## Overview
UI in this engine is built with MVVM architecture and a signal-driven reactive system. It uses Taffy (Flexbox) for layout, SDF for text rendering, and Glassmorphism materials.

## MVVM Layers
| Layer | Responsibility | Implementation |
|----------|------|----------|
| View | Layout calculation and rendering | Taffy + Filament Ortho |
| ViewModel | UI state management | `Signal<T>` reactive |
| Model | ECS game data | Component / Resource |

## Signal-Driven Reactive System
Inspired by Floem. Recompute/re-render only UI components whose values changed.
```rust
// Signal definition
pub struct Signal<T> {
    value: T,
    subscribers: Vec<Callback>,
}

impl<T: PartialEq> Signal<T> {
    pub fn get(&self) -> &T {
        &self.value
    }

    pub fn set(&mut self, new_value: T) {
        if self.value != new_value {
            self.value = new_value;
            // Notify subscribers of the change
            for cb in &self.subscribers {
                cb.notify();
            }
        }
    }

    pub fn subscribe(&mut self, callback: Callback) {
        self.subscribers.push(callback);
    }
}
```

## Taffy Layout
Compute UI node coordinates and sizes using the Flexbox algorithm.
```rust
use taffy::prelude::*;

pub fn build_message_box(taffy: &mut TaffyTree) -> NodeId {
    let text_node = taffy.new_leaf(Style {
        size: Size { width: percent(1.0), height: auto() },
        padding: Rect::all(length(16.0)),
        ..Default::default()
    }).unwrap();

    let container = taffy.new_with_children(
        Style {
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            size: Size { width: percent(0.8), height: auto() },
            position: Position::Absolute,
            inset: Rect {
                bottom: length(32.0),
                left: percent(0.1),
                ..Rect::auto()
            },
            ..Default::default()
        },
        &[text_node],
    ).unwrap();

    container
}
```

## SDF Text Rendering
Use signed distance fields for crisp text rendering without quality loss.
### Flow
1. Pre-generate an SDF texture atlas from font glyphs.
2. Sample SDF in a custom Filament material.
3. Render sharp text at arbitrary scales.
```rust
pub struct SdfTextRenderer {
    atlas: SdfAtlas,
    material: FilamentMaterial,
}

impl SdfTextRenderer {
    pub fn render_text(&self, text: &str, position: Vec2, font_size: f32, color: Color) {
        // Create per-glyph quads and render with SDF material
    }
}
```

## Glassmorphism Material
Glassmorphism effect: blurred background + translucency + thin border.
```rust
pub struct GlassMaterial {
    pub blur_radius: f32,      // background blur strength
    pub opacity: f32,          // opacity (0.0-1.0)
    pub border_radius: f32,    // corner radius
    pub border_width: f32,     // border width
    pub tint_color: Color,     // tint color
}
```

### Implementation
1. Capture Game View output as texture.
2. Generate a Gaussian-blurred background texture.
3. Use it as UI panel background.

## Core UI Components
### Message Window
```rust
pub struct MessageWindow {
    pub text: Signal<String>,
    pub speaker: Signal<Option<String>>,
    pub is_visible: Signal<bool>,
    style: GlassMaterial,
    layout: NodeId,
}
```

### Choice Panel
```rust
pub struct ChoicePanel {
    pub choices: Signal<Vec<ChoiceItem>>,
    pub selected_index: Signal<Option<usize>>,
    style: GlassMaterial,
}
```

### Status Bar
```rust
pub struct StatusBar {
    pub hp: Signal<f32>,
    pub mp: Signal<f32>,
    pub name: Signal<String>,
}
```

## ECS <-> UI Communication
### UI -> ECS (command queue)
```rust
pub enum UiCommand {
    ChoiceSelected(usize),
    MenuAction(MenuAction),
    SliderChanged { id: String, value: f32 },
}
```

### ECS -> UI (signal updates)
```rust
// UI sync system
pub fn ui_sync_system(/* ECS query */) {
    // Detect component changes -> update signals
    // Example: detect HP change -> status_bar.hp.set(new_hp)
}
```

## Render Layer
Render UI in `UI View (Layer 1)` with orthographic camera, depth testing disabled, Z = -10.0.
