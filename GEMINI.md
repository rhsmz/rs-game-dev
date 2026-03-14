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

## Agent System Structure
- `.agent/rules/` — Rust コーディング規約、アーキテクチャ規約、アセット規約、クレート構造規約
- `.agent/workflows/` — ビルド、テスト、リリース、新クレート追加
- `.agent/skills/` — シーン作成、ミニゲームプラグイン、Live2D/VRM統合、UI開発、セーブシステム

## Key Rules
- **コーディング:** `.agent/rules/rust_coding.md` に従う
- **アーキテクチャ:** `.agent/rules/architecture.md` に従う
- **アセット管理:** `.agent/rules/asset_convention.md` に従う
- **クレート構成:** `.agent/rules/crate_structure.md` に従う

## General Guidelines
- Rust のメモリ安全性・借用チェッカーを尊重し `unsafe` を最小限に
- `clippy` / `rustfmt` を常に適用
- ECS と UI を直接参照せずメッセージパッシングで連携
- ミニゲームは `Scene` トレイトを実装した独立プラグインとして追加
- セーブデータは MessagePack 形式 (`rmp-serde`)