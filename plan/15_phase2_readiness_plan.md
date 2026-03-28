# 15: Phase 2 Readiness Plan (Pre-Phase 2 Remediation WBS)

## Objective
Turn the current pre-Phase-2 findings into an executable plan that closes critical integration risks before full Phase 2 rollout.

## Scope
- In scope: `engine_core` Phase 1 gaps (Renderer, Audio ECS wiring, quality gates, safety/process hardening).
- Out of scope: Full Phase 2 feature implementation (`asset`, `script`, `scene`) except for readiness dependencies.

## Priority Model
- **P0 (Mandatory)**: Must be completed before broad Phase 2 development.
- **P1 (Recommended)**: Can run in parallel with early Phase 2 work.
- **P2 (Stretch)**: Nice-to-have, improves long-term reliability.

---

## P0 (Mandatory)

### P0-1. Complete a minimum vertical slice for Renderer DoD

#### Tasks
1. Implement `RenderEngine` frame lifecycle.
2. Instantiate actual `GameView` / `UiView` runtime objects.
3. Generate render commands in `render_system`.
4. Add a one-object/one-frame renderer smoke test.

#### Subtasks
- 1.1 Wire `begin_frame()` to real Filament frame begin and return actual state.
- 1.2 Wire `end_frame()` to submit/present path.
- 1.3 Define resize behavior for swap chain and viewport updates.
- 2.1 In `GameView::new`, create Filament `View`/`Scene`.
- 2.2 In `UiView::new`, configure ortho camera + disable depth where required.
- 2.3 Define cleanup ownership (Drop wrapper or dedicated RAII handle).
- 3.1 Query `MeshRenderer` + `Camera3D` and submit renderables.
- 3.2 Define behavior when camera is missing (skip vs fail-fast).
- 3.3 Fix render order (Game first, then UI) and test it.
- 4.1 Add CI smoke with `--features filament` validating one successful frame.
- 4.2 Improve diagnostic logs for init/render failures.

#### Definition of Done
- One test verifies clear + basic mesh draw + UI overlay in a single frame.

---

### P0-2. De-stub public APIs that currently return `not implemented`

#### Tasks
1. Inventory all public APIs returning `not implemented yet`.
2. Decide per-API strategy: implement / gate / hide / move-to-stub-layer.
3. Evaluate compatibility impact.

#### Subtasks
- 1.1 Track APIs including:
  - `SkyBox.apply_to`
  - `SpriteRenderer.render`
  - `RenderTarget.apply_for_live2d`
  - `PostProcessPipeline.apply_to`
- 1.2 Map each API to callers and dependency edges.
- 2.1 For near-term required APIs, replace hard-fail stubs with minimal safe behavior.
- 2.2 For deferred APIs, apply feature gating or reduce visibility (`pub(crate)`).
- 2.3 Split reporting labels into "scaffolded" vs "fully implemented" in task tracking.
- 3.1 Record SemVer implications in release notes/changelog draft.
- 3.2 Add guards/fallbacks at callsites.

#### Definition of Done
- No Phase 1 public API returns `not implemented yet` without explicit gating or de-publication.

---

### P0-3. Add Audio ECS execution path

#### Tasks
1. Implement a system that consumes `AudioCommand`.
2. Add adapter layer from ECS commands to `GameAudioManager` operations.
3. Define execution stage/order.
4. Add end-to-end tests.

#### Subtasks
- 1.1 Drain `AudioCommand` events from queue and dispatch by variant.
- 1.2 Support minimum command set: `PlayBgm`, `StopBgm`, `PlaySe`, `PlayVoice`, `SetVolume`.
- 2.1 Define load failure policy (retry/skip/report).
- 2.2 Normalize volume semantics (dB vs linear) at API boundaries.
- 3.1 Choose fixed stage (`Update` or `PostUpdate`) and document rationale.
- 3.2 Define same-frame conflict policy (e.g., Stop > Play).
- 4.1 Add E2E test: Input/Event -> ECS -> Audio system.
- 4.2 Add fake backend for device-independent CI tests.

#### Definition of Done
- BGM/SE/Voice minimum controls are reproducible via ECS-driven command flow.

---

## P1 (Recommended)

### P1-1. Enforce quality gates (from policy to execution)

#### Tasks
1. Add CI feature matrix.
2. Raise renderer smoke acceptance bar.
3. Improve failure diagnosability.

#### Subtasks
- 1.1 Run `default`, `--no-default-features`, and `--features filament` in CI.
- 1.2 Run at least one Linux/macOS lane where feasible.
- 2.1 Upgrade smoke success criteria from "init succeeds" to "one frame rendered".
- 2.2 Expand non-ignored audio logic tests.
- 3.1 Include feature/OS/backend context in failure logs.

#### Exit Criteria
- CI catches regressions across feature combinations before merge.

---

### P1-2. Document and govern unsafe boundaries

#### Tasks
1. Capture rationale for `unsafe impl Send/Sync`.
2. Document runtime/threading constraints.
3. Add review controls.

#### Subtasks
- 1.1 Document assumptions for `GameView`/`UiView` unsafe impls.
- 1.2 Explicitly list forbidden access patterns.
- 2.1 Create `SAFETY.md` and link from module docs.
- 3.1 Add unsafe checklist section to PR review template.

#### Exit Criteria
- Unsafe usage has auditable invariants and repeatable review checks.

---

### P1-3. Shift progress tracking from existence-based to behavior-based

#### Tasks
1. Define reusable DoD template.
2. Update project tracking conventions.

#### Subtasks
- 1.1 Require: runtime behavior, test evidence, and failure-mode statement for completion.
- 2.1 Require traceable test IDs when marking task checkboxes complete.

#### Exit Criteria
- Task completion reflects verified behavior, not just code presence.

---

## P2 (Stretch)

### P2-1. Expand integration tests

#### Tasks
1. Add Input -> ECS -> System E2E.
2. Add RNG seed relay pseudo-scene transition tests.
3. Add cross-subsystem smoke tests.

#### Subtasks
- 1.1 Verify `ActionTriggered` is consumed by intended systems.
- 2.1 Verify same seed/salt determinism and changed-salt divergence.
- 3.1 Schedule nightly low-frequency smoke run for early regression detection.

#### Exit Criteria
- Integration regressions are detected before they impact Phase 2 feature work.

---

## Suggested Sequencing
1. **Sprint A (P0 focus)**: P0-1 and P0-2 in parallel, then P0-3 integration.
2. **Sprint B (P0 closure + P1 start)**: lock DoD evidence, begin P1-1 and P1-2.
3. **Sprint C (Phase 2 controlled start)**: begin limited Phase 2 while finishing P1-3 and optionally P2-1.

## Readiness Gate (Go / No-Go)
- **Go** only when all P0 DoD items are met and evidenced in CI artifacts.
- **Conditional Go** allowed if one P1 item is open but risk is explicitly accepted.
- **No-Go** if renderer one-frame smoke or audio ECS command path is still missing.
