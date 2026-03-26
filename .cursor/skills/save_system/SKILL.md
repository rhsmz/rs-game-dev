---
name: save_system
description: MessagePack-based save/load system implementation skill
---

# Save System Implementation Skill
## Overview
Serialize/deserialize game progress using MessagePack (`rmp-serde`). Save data includes script position, global variables, Live2D/3D state, minigame data, and thumbnails.

## Save Data Structure
```rust
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct SaveFile {
    /// Save data format version (for migration)
    pub version: u32,
    /// Save timestamp (UNIX timestamp)
    pub timestamp: u64,
    /// Script position (file name, line number)
    pub script_pos: (String, usize),
    /// Global variables (affinity, flags, etc.)
    pub global_vars: HashMap<String, i32>,
    /// Live2D parameter state (for pose restoration)
    pub live2d_state: Vec<Param>,
    /// Serialized minigame-specific data
    pub mini_game_blob: Vec<u8>,
    /// Thumbnail image (JPEG binary)
    pub thumbnail: Option<Vec<u8>>,
    /// LogicRng seed state (for replay reproducibility)
    pub rng_seed: [u8; 32],
}

#[derive(Serialize, Deserialize)]
pub struct Param {
    pub id: String,
    pub value: f32,
}
```

## Serialization / Deserialization
```rust
/// MessagePack serialization (named fields for version compatibility)
pub fn save_to_bytes(save: &SaveFile) -> Result<Vec<u8>, SaveError> {
    // to_vec_named: preserve field names (better cross-version compatibility)
    // to_vec: compact array format (size-first use case)
    rmp_serde::to_vec_named(save).map_err(SaveError::Serialize)
}

pub fn load_from_bytes(data: &[u8]) -> Result<SaveFile, SaveError> {
    rmp_serde::from_slice(data).map_err(SaveError::Deserialize)
}
```

## File I/O
```rust
use std::path::Path;

pub fn save_to_file(save: &SaveFile, path: &Path) -> Result<(), SaveError> {
    let bytes = save_to_bytes(save)?;
    std::fs::write(path, bytes).map_err(SaveError::Io)?;
    Ok(())
}

pub fn load_from_file(path: &Path) -> Result<SaveFile, SaveError> {
    let bytes = std::fs::read(path).map_err(SaveError::Io)?;
    load_from_bytes(&bytes)
}
```

## Save Slot Management
```rust
pub struct SaveManager {
    save_dir: PathBuf,
    max_slots: usize,
}

impl SaveManager {
    pub fn new(save_dir: PathBuf, max_slots: usize) -> Self {
        Self { save_dir, max_slots }
    }

    /// List save slots (with thumbnail and timestamp)
    pub fn list_slots(&self) -> Result<Vec<SaveSlotInfo>, SaveError> {
        // Scan .sav files under save_dir
        // Read header only and return thumbnail/timestamp
        todo!()
    }

    /// Save to specific slot
    pub fn save(&self, slot: usize, save: &SaveFile) -> Result<(), SaveError> {
        let path = self.save_dir.join(format!("slot_{:03}.sav", slot));
        save_to_file(save, &path)
    }

    /// Load from specific slot
    pub fn load(&self, slot: usize) -> Result<SaveFile, SaveError> {
        let path = self.save_dir.join(format!("slot_{:03}.sav", slot));
        load_from_file(&path)
    }

    /// Auto save
    pub fn auto_save(&self, save: &SaveFile) -> Result<(), SaveError> {
        let path = self.save_dir.join("auto_save.sav");
        save_to_file(save, &path)
    }
}
```

## Version Migration
Handle format changes via the `version` field in save data.
```rust
pub fn migrate(mut save: SaveFile) -> Result<SaveFile, SaveError> {
    match save.version {
        1 => {
            // v1 -> v2: add rng_seed field
            // Set default values
            save.version = 2;
            migrate(save)  // recursive migration chain
        }
        2 => Ok(save),  // latest version
        _ => Err(SaveError::UnsupportedVersion(save.version)),
    }
}
```

## Error Type
```rust
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SaveError {
    #[error("serialization error: {0}")]
    Serialize(#[from] rmp_serde::encode::Error),
    #[error("deserialization error: {0}")]
    Deserialize(#[from] rmp_serde::decode::Error),
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("unsupported save version: {0}")]
    UnsupportedVersion(u32),
}
```

## Thumbnail Generation
Capture the current game screen on save, encode as JPEG, and include in save data.
```rust
pub fn capture_thumbnail(renderer: &Renderer, width: u32, height: u32) -> Vec<u8> {
    // Read Filament framebuffer
    // Resize (e.g., low resolution 128x72)
    // JPEG encode
    todo!()
}
```

## ECS Integration
```rust
pub enum SaveCommand {
    Save(usize),       // slot number
    Load(usize),
    AutoSave,
    QuickSave,
    QuickLoad,
}

// Save system
pub fn save_system(/* ECS query */) {
    // Check SaveCommand
    // Build SaveFile from current ECS state
    // Write file through SaveManager
}
```

## Notes
- Do not include sensitive data in save files (anti-cheat strategy is separate).
- Always run `migrate()` on load to ensure version compatibility.
- Use around 70% thumbnail compression quality to control file size.
