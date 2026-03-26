---
name: git_commit
description: Semantic commit messages, phase-based feature branches, and PR workflow skill
---

# Git Commit Skill
## Overview
This project uses **Semantic Commit Messages** and phase-based feature branches. Changes are merged into `main` through pull requests.

**Important**: The `<summary>`, body, and footer of commit messages must be written in **Japanese**. Only `<type>` and `<scope>` remain in English.
PR titles and body text must also be written in **Japanese**.

---
## 1. Commit Message Convention
### Format
```
<type>(<scope>): <summary in Japanese>

[optional body in Japanese]

[optional footer]
```

### type list
| type       | Usage                                            |
|------------|--------------------------------------------------|
| `feat`     | New feature                                      |
| `fix`      | Bug fix                                          |
| `refactor` | Refactoring without behavior change              |
| `perf`     | Performance improvement                          |
| `test`     | Add or update tests                              |
| `docs`     | Documentation/comment only changes               |
| `style`    | Formatting, whitespace, semicolons (no logic)    |
| `build`    | Build system / dependency changes (Cargo.toml)   |
| `ci`       | CI/CD configuration changes                      |
| `chore`    | Other maintenance tasks                          |
| `revert`   | Revert a previous commit                         |

### scope list (this project)
| scope           | Target crate / module                            |
|-----------------|--------------------------------------------------|
| `engine_core`   | `crates/engine_core` overall                     |
| `script_editor` | `crates/script_editor` overall                   |
| `game_player`   | `crates/game_player` overall                     |
| `ecs`           | ECS foundation module                            |
| `scene`         | Scene management module                          |
| `render`        | Rendering engine (Filament integration)          |
| `audio`         | Sound system                                     |
| `asset`         | Asset management                                 |
| `script`        | Script parser / state machine                    |
| `ui`            | UI layout / rendering                            |
| `live2d`        | Live2D Cubism SDK integration                    |
| `vrm`           | VRM/FBX model integration                        |
| `save`          | Save/load system                                 |
| `minigame`      | Minigame plugin                                  |
| `workspace`     | Cargo workspace / root config                    |
| `agent`         | Rules/skills/workflows under `.agent/`           |

### Commit message examples
```
feat(ecs): World・Entity・Component の基本型を定義
fix(scene): on_exit 後にアセットが解放されない問題を修正
refactor(render): CommandBuffer 生成処理を関数に抽出
docs(agent): git_commit スキルを追加
build(workspace): engine_core クレートを Cargo.toml に追加
test(save): セーブ/ロードのラウンドトリップテストを追加
```

### Body and footer guidelines

- Use the body to explain **why** the change was made when the summary alone is not enough. Write the body in **Japanese**.
- For breaking changes, include a line starting with `BREAKING CHANGE:` in the body.
- Reference related issues with `Closes #<number>` in the footer.

```
feat(script): タグベーススクリプトのジャンプ命令を実装

分岐演出に必要な goto/gosub 命令を追加した。
ステートマシンのスタックを利用して gosub の復帰先を管理する。

Closes #42
```

---
## 2. Feature Branch Strategy
### Branch naming

```
feature/<phase>/<task-description>
```

| Element              | Description                                                        |
|----------------------|--------------------------------------------------------------------|
| `<phase>`            | `p1a`, `p1b` ... phase number + sub-phase letter (e.g. `p2a`)     |
| `<task-description>` | English kebab-case string describing the feature (1 PR scope)      |

**Examples:**

```
feature/p1a/cargo-workspace-init
feature/p1b/gitignore-rustfmt-config
feature/p2a/ecs-world-entity-types
feature/p2b/ecs-component-storage
feature/p2c/ecs-system-scheduler
feature/p3a/scene-trait-definition
feature/p3b/scene-manager-stack
feature/p3c/scene-async-transition
```

### Creating a branch

```bash
# Branch from latest main
git switch main
git pull origin main
git switch -c feature/<phase>/<task-description>
```

### Commit granularity guidelines

- 1 commit = **one logical, cohesive change** (types only, implementation only, tests only, etc.)
- Commit only in buildable state (`cargo check` must pass).
- Aim for **3-10 commits** per PR (complete a feature unit).
- Manage WIP commits with `git commit --fixup` or `git stash`; clean up with `rebase -i` before PR.

---
## 3. PR (Pull Request) Creation
### PR title

Use the same format as commit messages (summary in Japanese).

```
feat(ecs): ECS 基盤の World・Entity・Component を実装
```

### PR body template

PR body must be written in **Japanese**.

```markdown
## 概要

<!-- このPRで何をしたか、なぜ必要か -->

## 変更内容

- [ ] 項目 1
- [ ] 項目 2

## テスト方法

```bash
cargo test -p engine_core
```

## 関連 Issue

Closes #<番号>
```

### Creating PR with GitHub CLI

```bash
# Push feature branch
git push -u origin feature/<phase>/<task-description>

# Create PR (gh command)
gh pr create \
  --title "feat(ecs): ECS 基盤の World・Entity・Component を実装" \
  --body-file .cursor/skills/git_commit/pr_template.md \
  --base main \
  --head feature/<phase>/<task-description>
```

---
## 4. Phase Branch Plan

Each phase is subdivided into feature sub-branches (a/b/c...).
1 sub-branch = 1 PR = 1 feature/responsibility completed.

### P1: Workspace Initialization

| Branch                                | Key content                                       |
|--------------------------------------|---------------------------------------------------|
| `feature/p1a/cargo-workspace-init`   | Root `Cargo.toml`, workspace member definition     |
| `feature/p1b/gitignore-rustfmt`      | `.gitignore`, `rustfmt.toml`, `clippy.toml`        |
| `feature/p1c/crate-stub-modules`     | `lib.rs` / `main.rs` stubs for each crate          |

### P2: ECS Foundation

| Branch                                | Key content                                       |
|--------------------------------------|---------------------------------------------------|
| `feature/p2a/ecs-world-entity`       | `World`, `Entity` (ID generation/reuse) types      |
| `feature/p2b/ecs-component-storage`  | `ComponentStorage` (SparseSet / Dense Array)       |
| `feature/p2c/ecs-system-scheduler`   | `System` trait, scheduler, execution order control |
| `feature/p2d/ecs-query-api`          | `Query<T>` / `QueryMut<T>` query API              |
| `feature/p2e/ecs-event-bus`          | Event queue, `EventReader` / `EventWriter`         |

### P3: Scene Management

| Branch                                | Key content                                       |
|--------------------------------------|---------------------------------------------------|
| `feature/p3a/scene-trait-definition` | `Scene` trait, `SceneTransition` enum              |
| `feature/p3b/scene-manager-stack`    | `SceneManager` (stack: push/pop/replace)           |
| `feature/p3c/scene-context`          | `SceneContext` (message passing infrastructure)    |
| `feature/p3d/scene-async-transition` | Async transitions, loading screen                  |

### P4: Asset Management

| Branch                                | Key content                                       |
|--------------------------------------|---------------------------------------------------|
| `feature/p4a/asset-descriptor`       | `AssetDescriptor`, asset identifiers/types         |
| `feature/p4b/asset-loader-core`      | `AssetLoader` trait, sync load infrastructure      |
| `feature/p4c/asset-async-loading`    | Async loading, progress notification               |
| `feature/p4d/asset-cache`            | Asset cache, reference counting/release            |

### P5: Rendering Engine (Filament)

| Branch                                | Key content                                       |
|--------------------------------------|---------------------------------------------------|
| `feature/p5a/filament-ffi-bindings`  | Filament C++ -> Rust FFI bindings                  |
| `feature/p5b/render-command-buffer`  | `CommandBuffer`, render command queue              |
| `feature/p5c/render-multi-view`      | Game view (Layer 0) + UI view (Layer 1)            |
| `feature/p5d/render-pbr-materials`   | PBR materials, IBL configuration                   |
| `feature/p5e/render-post-process`    | Post-processing (bloom, tone mapping, etc.)        |

### P6: Sound System

| Branch                                | Key content                                       |
|--------------------------------------|---------------------------------------------------|
| `feature/p6a/audio-backend-kira`     | `kira` integration, AudioManager init              |
| `feature/p6b/audio-bgm-player`       | BGM playback, crossfade                            |
| `feature/p6c/audio-se-voice`         | SE / Voice channels, mixer                         |

### P7: Script Engine

| Branch                                | Key content                                       |
|--------------------------------------|---------------------------------------------------|
| `feature/p7a/script-tokenizer`       | Tag-based tokenizer, lexical analysis              |
| `feature/p7b/script-parser-ast`      | Parser, AST definition                             |
| `feature/p7c/script-state-machine`   | State machine, instruction execution loop          |
| `feature/p7d/script-commands-basic`  | Basic commands (text, wait, jump, label)           |
| `feature/p7e/script-commands-adv`    | Advanced commands (branch, call, gosub, variable)  |

### P8: UI System

| Branch                                | Key content                                       |
|--------------------------------------|---------------------------------------------------|
| `feature/p8a/ui-taffy-layout`        | Taffy Flexbox layout engine integration            |
| `feature/p8b/ui-signal-reactive`     | Signal-driven reactive system                      |
| `feature/p8c/ui-sdf-text`            | SDF text rendering                                 |
| `feature/p8d/ui-glassmorphism`       | Glassmorphism materials, blur effects              |
| `feature/p8e/ui-animation`           | UI animation, transitions                          |

### P9: Live2D Integration

| Branch                                | Key content                                       |
|--------------------------------------|---------------------------------------------------|
| `feature/p9a/live2d-ffi-core`        | Cubism Core C++ FFI, model loading                 |
| `feature/p9b/live2d-render-texture`  | Render-to-Texture pipeline                         |
| `feature/p9c/live2d-motion`          | Motion playback and blending                       |
| `feature/p9d/live2d-lipsync`         | Lip sync (audio analysis -> parameter mapping)     |

### P10: VRM/FBX Integration

| Branch                                | Key content                                       |
|--------------------------------------|---------------------------------------------------|
| `feature/p10a/vrm-loader`            | VRM/GLB loading, `gltf` crate integration          |
| `feature/p10b/vrm-spring-bone`       | SpringBone physics simulation                      |
| `feature/p10c/vrm-blend-shape`       | BlendShape / MorphTarget control                   |
| `feature/p10d/fbx-import`            | FBX -> GLB conversion pipeline                     |

### P11: Save System

| Branch                                | Key content                                       |
|--------------------------------------|---------------------------------------------------|
| `feature/p11a/save-data-schema`      | Save data structs, `serde` implementation          |
| `feature/p11b/save-msgpack-io`       | MessagePack (`rmp-serde`) serialization/IO         |
| `feature/p11c/save-migration`        | Version migration mechanism                        |
| `feature/p11d/save-slot-manager`     | Slot management, thumbnail/metadata                |

### P12: Minigame Plugin

| Branch                                | Key content                                       |
|--------------------------------------|---------------------------------------------------|
| `feature/p12a/minigame-plugin-trait` | `MinigamePlugin` trait, registration/invocation API|
| `feature/p12b/minigame-2d-action`    | 2D action minigame prototype                       |
| `feature/p12c/minigame-rpg-battle`   | RPG battle minigame prototype                      |
| `feature/p12d/minigame-puzzle`       | Puzzle minigame prototype                          |
| `feature/p12e/minigame-rts`          | RTS minigame prototype                             |

---
## 5. Pre-Commit Checklist

Before committing, verify the following:

- [ ] `cargo fmt --all` completed
- [ ] `cargo clippy --all-targets -- -D warnings` passes with no errors/warnings
- [ ] `cargo check --all-targets` passes with no build errors
- [ ] Related tests pass (`cargo test`)
- [ ] Commit message follows this skill's format
- [ ] Scope is set correctly

---
## 6. Best Practices

- **Atomic commits**: Do not include unrelated changes in one commit.
- **Rebase before PR**: If main has advanced, rebase with `git rebase main` before opening a PR.
- **Squash carefully**: Preserve meaningful commit history. Prefer `Rebase and merge` over `Squash and merge`.
- **Draft PR**: Use draft PRs when early review feedback is needed during implementation.
- **Conflict resolution**: Always resolve conflicts manually; do not rely on auto-merge.
