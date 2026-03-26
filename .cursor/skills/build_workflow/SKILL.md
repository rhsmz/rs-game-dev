---
name: build_workflow
description: Guides project build steps including workspace builds, FFI link setup, and environment variable checks.
---

# Build Workflow

## Prerequisites
- Rust toolchain (stable) is installed.
- Filament SDK is built and paths are configured.
- Live2D Cubism Native SDK is downloaded.

## Steps

### 1. Check dependencies
```bash
rustup show
cargo --version
```

### 2. Build full workspace (Debug)
```bash
cargo build --workspace
```

### 3. Build specific crates
```bash
cargo build -p engine_core
cargo build -p script_editor
cargo build -p game_player
```

### 4. Release build
```bash
cargo build --workspace --release
```

### 5. FFI library link configuration
Configure C/C++ library linking (Filament, Live2D SDK) in `build.rs`.

```rust
// crates/engine_core/build.rs
fn main() {
    // Filament
    println!("cargo:rustc-link-search=native={}", env!("FILAMENT_LIB_DIR"));
    println!("cargo:rustc-link-lib=static=filament");

    // Live2D Cubism
    println!("cargo:rustc-link-search=native={}", env!("CUBISM_LIB_DIR"));
    println!("cargo:rustc-link-lib=static=Live2DCubismCore");
}
```

### 6. Environment variables
| Variable | Description |
|--------|------|
| `FILAMENT_LIB_DIR` | Filament library directory |
| `CUBISM_LIB_DIR` | Live2D Cubism SDK library directory |

## Troubleshooting
- If you hit link errors, verify SDK build config (Debug/Release) and target architecture (x64) match.
- On Windows, use the MSVC toolchain.
