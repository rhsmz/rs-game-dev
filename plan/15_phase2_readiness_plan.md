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

#### Visual golden / pixel tests（導入条件メモ）
- **ゴールデン画像**比較を CI に載せる前に次を満たすこと: 固定解像度のオフスクリーン RT、プラットフォーム間の許容誤差（SSAA / トーンマップ差）、参照画像の保管場所と更新手順。
- 現状の `filament_smoke` はパス順・メッシュ縦スライス受理・リサイズ同期の**自動検証**まで。Game/UI 重畳のピクセル一致は [`crates/engine_core/tests/filament_smoke.rs`](../crates/engine_core/tests/filament_smoke.rs) の `#[ignore]` プレースホルダがハーネス接続後に有効化する。

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

---

## テスト階層（B2-1 / `TASKS.MD` と一致）

| 階層 | 内容 | 代表コマンド |
|------|------|----------------|
| unit | `src/**` 内 `#[cfg(test)]` | `cargo test -p engine_core audio::command_dispatch` |
| integration | `crates/*/tests/*.rs`（別バイナリ） | `cargo test -p engine_core --test audio_command_drain` |
| smoke | winit + Filament 縦スライス | `cargo test -p engine_core --features filament --test filament_smoke`（Linux: `xvfb-run -a`、macOS: `RUST_TEST_THREADS=1`） |
| nightly / manual | `#[ignore]`・実デバイス | `cargo test -p engine_core --features audio-kira -- --ignored` |

CI のステップ構成は [`.github/workflows/ci.yml`](../.github/workflows/ci.yml) を参照。

## ログキー・代表メッセージ（B3-1 初版）

`RUST_LOG` 例: `engine_core::renderer=debug,engine_core::audio=trace`。

### `target: engine_core::renderer`

| レベル | メッセージの要約 |
|--------|------------------|
| debug | `render_system: no Camera3D entities; skipping mesh submission (views still rendered)` |
| debug | `render_system: {n} Camera3D entities; using first in storage order only` |
| info | `render_system: vertical_slice frame — mesh_entities=... camera_entities=...` |
| warn | `mesh submit skipped: renderable_id=0 (unloaded); entity=...` |
| warn | `mesh submit failed: renderable_id=... entity=...` |

### `target: engine_core::audio`

| レベル | メッセージの要約 |
|--------|------------------|
| warn | `audio_command_system: dropping N audio command(s): GameAudioManager resource missing` |
| warn | `audio_command_system: bulk_drop threshold exceeded (drop_count=... threshold=...)` |
| warn | `audio command dispatch error: ...` |
| （経由） | 欠損・デコード系は `format_audio_warn_message`（単体テストで形式固定） |

### その他（target 未指定の注意）

`engine_core::renderer::engine` 内の一部 `warn!` は `target` を付けていないため、既定では `engine_core::renderer::engine` がログ名になる。

---

## 検証コマンド（エビデンス用）

| 領域 | コマンド |
|------|----------|
| ワークスペース | `cargo test --workspace` |
| `engine_core` スタブ音声 | `cargo test -p engine_core --no-default-features` |
| Audio キュー（統合・デバイス非依存） | `cargo test -p engine_core --test audio_command_drain` |
| Audio dispatch（FIFO・bulk drop 等） | `cargo test -p engine_core audio::command_dispatch::tests` |
| Renderer smoke | `cargo test -p engine_core --features filament --test filament_smoke` |
| Audio 実デバイス（ignored） | `cargo test -p engine_core --features audio-kira -- --ignored` |

`#[ignore]` のみをもってタスク完了にしないこと（`TASKS.MD` の運用ルール参照）。

---

## 実施状況ログ（2026-03-29 更新）

### P0-1 Renderer 縦切り
- [x] フレーム契約: `begin_frame` → `render_system`（**GameView → UiView** 順をコードコメントで固定）→ `end_frame`
- [x] `filament_smoke`: ウィンドウ `inner_size` を `RenderEngine::resize` に渡す
- [x] ECS で `Camera3D` + `MeshRenderer` 投入、カメラ欠如時はメッシュ系スキップ（既存挙動）
- [x] 診断ログ: `target = "engine_core::renderer"` で初期化・縦切りフレームを `info`/`debug`
- [x] **ブリッジ実装:** [`crates/engine_core/stub/filament_bridge.cpp`](../crates/engine_core/stub/filament_bridge.cpp) で `Scene_submit_mesh_vertical_slice` が `RenderableManager` + `Engine::getDefaultMaterial` + 三角形を `Scene` に追加（`FILAMENT_LIB_DIR` リンク時）。`Scene_destroy` で C++ 側レジストリを解放
- [ ] **残（Phase 2）:** アセット由来の頂点／インデックス／マテリアル、`MeshRenderer` と Filament 資源の永続レジストリ、ロード失敗時の warn+skip 統合
- [x] **対応表（設計メモ）:** `renderable_id` から `RenderableManager` 実体へ至る **レジストリ方針・ライフサイクル**を [`crates/engine_core/src/renderer/mod.rs`](../crates/engine_core/src/renderer/mod.rs) のモジュール doc に記載

### P0-2 公開 API の足場
- [x] `SkyBox::apply_to` / `SpriteRenderer::render` / `RenderTarget::apply_for_live2d` / `PostProcessPipeline::apply_to` を **`pub(crate)`** に変更（外部から足場を誤用しない）
- [x] Phase 2 まで未配線であることをドキュメント化、`dead_code` 許可は足場専用とコメント

### P0-3 Audio ECS
- [x] `audio_command_system` + キュー FIFO、**同一フレーム優先ルールは厳密 FIFO**（`command_dispatch` モジュール doc）
- [x] 競合境界テスト: `test_fifo_stop_play_stop_play_policy` / `test_fifo_play_se_set_volume_play_voice` / bulk drop しきい値
- [x] warn ログキー: `seq` / `source` / `op` / `err_kind`（`target = engine_core::audio`）
- [x] **デバイス非依存**: `audio_command_drain` + `command_dispatch` ユニット
- [x] **E2E（kira）** `test_audio_ecs_set_volume_end_to_end_kira`（`#[ignore]` — nightly / 手動のみ完了根拠にしない）

### P1-1 CI
- [x] `.github/workflows/ci.yml`: `fmt` / `clippy` / `test --workspace`、行列で各 OS（`ubuntu` / `windows` / `macos`）について **`args: ""` と `args: "--no-default-features"`** の少なくとも 2 行（plan P1-1.1 の default / no-default を全 OS で担保）
- [x] `cargo test -p engine_core --features filament --test filament_smoke` を毎ジョブで実行（スタブリンク想定）

### P1-2 Unsafe ガバナンス
- [x] `crates/engine_core/SAFETY.md` 新設
- [x] `view.rs` モジュール Doc から参照
- [x] `.github/pull_request_template.md` に unsafe チェックリスト

### P1-3 / P2-1
- [ ] トラッキング運用の刷新、追加統合テスト — 未着手（Stretch）

### P2-1 可観測性（メトリクス名のたたき台・実装は未着手）
- 候補カウンタ名（ログまたは将来 OTel 用）: `engine_core.renderer.frame_begin_false_total`、`engine_core.renderer.mesh_attach_fail_total`、`engine_core.renderer.mesh_skipped_unloaded_total`、`engine_core.audio.command_drop_total`、`engine_core.audio.command_drop_bulk_total`
- nightly でのトレンド保存・閾値通知は CI / 外部監視との接続が必要（TASKS.MD Phase 1-alpha P2-1 参照）
