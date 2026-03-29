# GEMINI Agent Instructions

本リポジトリの AI エージェント向けコンテキストファイル。

## Project Context
- **Name:** rs-game-dev
- **Description:** Rust 製アドベンチャーゲームエンジン。ビジュアルノベルコアにミニゲーム（2D/3Dアクション、RPG、パズル、RTS）を統合するプラグイン型アーキテクチャ。
- **Language:** Rust
- **Build Tool:** Cargo (workspace)
- **Reference:** 恋姫†無双 (Koihime Musō)、TWINKLE CRUSADERS

## Technology Stack
- **Renderer:** Google Filament (PBR, IBL, ポストプロセス)
- **2D Character:** Live2D Cubism Native SDK (C++ → Rust FFI)
- **3D Character:** VRM / FBX (SpringBone, BlendShape, GLB変換)
- **UI Layout:** Taffy (Flexbox) + Signal駆動リアクティブ (Floem思想)
- **UI Rendering:** SDF テキスト、Glassmorphism マテリアル
- **Audio:** `kira` or `rodio` (BGM/SE/Voice ミキシング)
- **ECS:** Entity Component System ベースのゲームロジック
- **Serialization:** `serde`, `rmp-serde` (MessagePack), CSV メタデータ
- **Script:** タグベース独自スクリプト構文 (パーサ + ステートマシン)
- **RNG:** Xoshiro256++ (演出用) / ChaCha8 (ロジック・リプレイ用)

## Crate Structure (`crates/`)
- **`engine_core`** — ECS基盤、描画エンジン、サウンド、アセット管理、スクリプト解析
- **`script_editor`** — オーサリングツール (ライブプレビュー、ビジュアル編集、シグナル駆動UI)
- **`game_player`** — 配布用軽量ランタイム (アセットアーカイブ解読 + メインループ)

## Architecture Patterns
- **ECS × MVVM**: ゲームロジック (ECS) と UI (MVVM) をメッセージパッシングで疎結合に連携
- **プラグインパターン**: ミニゲームを独立モジュール (`Scene` トレイト実装) として管理
- **シーンベースアーキテクチャ**: 全パートを `Scene` トレイトで抽象化、非同期ロード遷移
- **マルチビューレンダリング**: Game View (Layer 0: 3D+Live2D+2D) + UI View (Layer 1: Ortho)
- **Dual RNG**: 演出用 (Xoshiro256++) とロジック用 (ChaCha8) の分離

## 規約・ワークフローの所在（Cursor を正とする）

Cursor / Copilot 等は **`.cursor/rules/*.mdc`** をワークスペースルールとして読み込む。本ファイルは **汎用 AI 向けの短い索引**であり、詳細は各ルールファイルを参照すること。

| 種別 | パス（編集の優先順位） |
|------|-------------------------|
| ルール | [`.cursor/rules/`](.cursor/rules/)（`rust-coding.mdc`, `architecture.mdc`, `project-context.mdc`, `crate-structure.mdc`, `asset-convention.mdc`） |
| スキル | [`.cursor/skills/`](.cursor/skills/)（`build_workflow`, `test_workflow`, `git_commit` 等の `SKILL.md`） |
| 補助 | [`.agent/`](.agent/)（旧来の `rules` / `workflows` / `skills`。**`.cursor` と重複する場合は `.cursor` を正**とし、新規は `.cursor` に集約する） |

## タスク・計画（作業の入口）

- **日々のタスク・DoD**: [`TASKS.MD`](TASKS.MD)（`## Phase 1-alpha review修正` はレビュー由来の継続 WBS）
- **Phase 2 readiness・検証コマンド**: [`plan/15_phase2_readiness_plan.md`](plan/15_phase2_readiness_plan.md)
- **レビュー反映の経緯メモ**: [`docs/phase1_review_fix_2026-03-29.md`](docs/phase1_review_fix_2026-03-29.md)

## General Guidelines
- Rust のメモリ安全性・借用チェッカーを尊重し `unsafe` は FFI 境界に限定（[`SAFETY.md`](SAFETY.md)）
- `cargo fmt` / `cargo clippy -- -D warnings` を PR 前に通す
- ECS と UI を直接参照せずメッセージパッシングで連携
- ミニゲームは `Scene` トレイトを実装した独立プラグインとして追加
- セーブデータは MessagePack 形式 (`rmp-serde`)