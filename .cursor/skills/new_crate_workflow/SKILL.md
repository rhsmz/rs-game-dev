---
name: new_crate_workflow
description: Guides adding a new crate to the Cargo workspace, from cargo init to build, test, and documentation.
---

# New Crate Workflow

## Steps

### 1. Create crate directory
```bash
cargo init --lib crates/{crate_name}
```

### 2. Register in workspace
Add the new crate to `members` in the root `Cargo.toml`.

```toml
[workspace]
members = [
    "crates/engine_core",
    "crates/script_editor",
    "crates/game_player",
    "crates/{crate_name}",  # added
]
```

### 3. Configure dependencies
Use workspace-shared dependencies in the new crate's `Cargo.toml`.

```toml
[dependencies]
serde = { workspace = true }
anyhow = { workspace = true }
engine_core = { path = "../engine_core" }
```

### 4. Verify dependency direction
Ensure the following rules are respected:
- `engine_core` must not depend on other internal crates.
- `script_editor` and `game_player` must not depend on each other.
- Circular dependencies are prohibited.

### 5. Build validation
```bash
cargo build --workspace
```

### 6. Test validation
```bash
cargo test -p {crate_name}
```

### 7. Documentation
- Add a module-level `//!` doc comment to the new crate's `lib.rs`.
- Update the crate structure section in `README.md`.
- Update the Crate Structure section in `GEMINI.md`.

## Checklist
- [ ] `crates/{name}/` created
- [ ] Added to root `Cargo.toml`
- [ ] Dependency-direction rules satisfied
- [ ] `cargo build --workspace` passes
- [ ] `cargo test -p {name}` passes
- [ ] Documentation updated
