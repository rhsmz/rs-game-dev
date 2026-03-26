---
name: test_workflow
description: Guides test and quality checks (cargo fmt, cargo clippy, cargo test) for formatting, static analysis, and unit tests.
---

# Test Workflow

## Steps

### 1. Format check
```bash
cargo fmt --all -- --check
```

Auto-fix if formatting is broken:
```bash
cargo fmt --all
```

### 2. Clippy (static analysis)
```bash
cargo clippy --workspace --all-targets -- -D warnings
```

### 3. Unit tests
```bash
cargo test --workspace
```

### 4. Test specific crates
```bash
cargo test -p engine_core
cargo test -p script_editor
cargo test -p game_player
```

### 5. Documentation tests
```bash
cargo test --workspace --doc
```

### 6. Integration tests
```bash
cargo test --workspace --test '*'
```

### 7. Run all checks in one command
```bash
cargo fmt --all -- --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace
```

## Test writing rules
- Test function name: `test_<target>_<condition>_<expected_result>`.
- Unit tests: write in `#[cfg(test)] mod tests` within the same file.
- Integration tests: place under `tests/` directory.
- ECS-related tests should run against a mocked World.
- FFI-boundary tests can be excluded from normal runs with `#[ignore]`.
