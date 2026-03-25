---
name: git_commit
description: Semantic Commit Message（日本語）・フェーズ別フィーチャーブランチ・PR作成ワークフロースキル
---

# Git コミットスキル
## 概要
本プロジェクトでは **Semantic Commit Message** を基本とし、コミットメッセージはすべて **日本語** で記述する。
また、実装は機能・フェーズ単位でフィーチャーブランチを切り、PR（プルリクエスト）を通じて `main` へマージする。

---
## 1. コミットメッセージ規約
### 基本フォーマット
```
<type>(<scope>): <要約>

[本文（任意）]

[フッター（任意）]
```

### type 一覧
| type       | 用途                                             |
|------------|------------------------------------------------|
| `feat`     | 新機能の追加                                     |
| `fix`      | バグ修正                                         |
| `refactor` | 動作を変えないリファクタリング                    |
| `perf`     | パフォーマンス改善                               |
| `test`     | テストの追加・修正                               |
| `docs`     | ドキュメント・コメントのみの変更                 |
| `style`    | フォーマット・空白・セミコロン等（ロジック無変更）|
| `build`    | ビルドシステム・依存関係の変更（Cargo.toml 等）  |
| `ci`       | CI/CD 設定の変更                                |
| `chore`    | その他のメンテナンス作業                         |
| `revert`   | 以前のコミットの取り消し                         |

### scope 一覧（本プロジェクト）
| scope           | 対象クレート / モジュール                        |
|-----------------|------------------------------------------------|
| `engine_core`   | `crates/engine_core` 全体                      |
| `script_editor` | `crates/script_editor` 全体                    |
| `game_player`   | `crates/game_player` 全体                      |
| `ecs`           | ECS 基盤モジュール                              |
| `scene`         | シーン管理モジュール                            |
| `render`        | 描画エンジン（Filament 連携）                   |
| `audio`         | サウンドシステム                               |
| `asset`         | アセット管理                                   |
| `script`        | スクリプトパーサ・ステートマシン               |
| `ui`            | UI レイアウト・レンダリング                    |
| `live2d`        | Live2D Cubism SDK 連携                        |
| `vrm`           | VRM/FBX モデル連携                            |
| `save`          | セーブ/ロードシステム                          |
| `minigame`      | ミニゲームプラグイン                           |
| `workspace`     | Cargo ワークスペース・ルート設定               |
| `agent`         | `.agent/` 以下のルール・スキル・ワークフロー   |

### コミットメッセージ例
```
feat(ecs): World・Entity・Component の基本型を定義
fix(scene): on_exit 後にアセットが解放されない問題を修正
refactor(render): CommandBuffer 生成処理を関数に抽出
docs(agent): git_commit スキルを追加
build(workspace): engine_core クレートを Cargo.toml に追加
test(save): セーブ/ロードのラウンドトリップテストを追加
```

### 本文・フッターの書き方

- 本文は要約だけでは伝わらない **なぜ** その変更をしたかを記述する
- 破壊的変更は本文に `BREAKING CHANGE:` で始まる行を含める
- 関連 Issue は `Closes #<番号>` でフッターに記述する

```
feat(script): タグベーススクリプトのジャンプ命令を実装

分岐演出に必要な goto/gosub 命令を追加した。
ステートマシンのスタックを利用して gosub の復帰先を管理する。

Closes #42
```

---
## 2. フィーチャーブランチ戦略
### ブランチ命名規則

```
feature/<phase>/<task-description>
```

| 要素                  | 説明                                                              |
|----------------------|------------------------------------------------------------------|
| `<phase>`            | `p1a`, `p1b` … フェーズ番号 + サブフェーズ英字（例: `p2a`, `p2b`）|
| `<task-description>` | 機能を端的に表す英語の kebab-case 文字列（1 PR で完結する粒度）    |

**例:**

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

### ブランチ作成手順

```bash
# 最新の main から派生させる
git switch main
git pull origin main
git switch -c feature/<phase>/<task-description>
```

### コミット粒度の指針

- 1 コミット ＝ **論理的に一つのまとまった変更**（型定義のみ、実装のみ、テストのみ、など）
- ビルドが通る状態でコミットする（`cargo check` を通過していること）
- 1 PR あたり **3〜10 コミット** を目安とする（機能単位で完結させる）
- 作業途中の WIP コミットは `git commit --fixup` または `git stash` で管理し、PR 前に `rebase -i` で整理する

---
## 3. PR（プルリクエスト）作成手順
### PR タイトル

コミットメッセージと同じフォーマットで記述する。

```
feat(ecs): ECS 基盤の World・Entity・Component を実装
```

### PR 本文テンプレート

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

### GitHub CLI を使った PR 作成

```bash
# フィーチャーブランチをプッシュ
git push -u origin feature/<phase>/<task-description>

# PR を作成（gh コマンド）
gh pr create \
  --title "feat(ecs): ECS 基盤の World・Entity・Component を実装" \
  --body-file .cursor/skills/git_commit/pr_template.md \
  --base main \
  --head feature/<phase>/<task-description>
```

---
## 4. フェーズ別ブランチ計画

各フェーズを機能単位のサブブランチ（a/b/c…）に細分化する。
1 つのサブブランチ = 1 PR = 1 つの機能・責務で完結させる。

### P1: ワークスペース初期化

| ブランチ                              | 主な内容                                          |
|--------------------------------------|---------------------------------------------------|
| `feature/p1a/cargo-workspace-init`   | ルート `Cargo.toml`、ワークスペースメンバー定義   |
| `feature/p1b/gitignore-rustfmt`      | `.gitignore`、`rustfmt.toml`、`clippy.toml`       |
| `feature/p1c/crate-stub-modules`     | 各クレートの `lib.rs` / `main.rs` スタブ作成      |

### P2: ECS 基盤

| ブランチ                              | 主な内容                                          |
|--------------------------------------|---------------------------------------------------|
| `feature/p2a/ecs-world-entity`       | `World`、`Entity`（ID 生成・再利用）の型定義      |
| `feature/p2b/ecs-component-storage`  | `ComponentStorage`（SparseSet / Dense Array）     |
| `feature/p2c/ecs-system-scheduler`   | `System` トレイト、スケジューラ、実行順制御       |
| `feature/p2d/ecs-query-api`          | `Query<T>` / `QueryMut<T>` クエリ API             |
| `feature/p2e/ecs-event-bus`          | イベントキュー、`EventReader` / `EventWriter`     |

### P3: シーン管理

| ブランチ                              | 主な内容                                          |
|--------------------------------------|---------------------------------------------------|
| `feature/p3a/scene-trait-definition` | `Scene` トレイト、`SceneTransition` 列挙型        |
| `feature/p3b/scene-manager-stack`    | `SceneManager`（スタック管理、push/pop/replace）  |
| `feature/p3c/scene-context`          | `SceneContext`（メッセージパッシング基盤）         |
| `feature/p3d/scene-async-transition` | 非同期トランジション、ローディング画面            |

### P4: アセット管理

| ブランチ                              | 主な内容                                          |
|--------------------------------------|---------------------------------------------------|
| `feature/p4a/asset-descriptor`       | `AssetDescriptor`、アセット識別子・型定義         |
| `feature/p4b/asset-loader-core`      | `AssetLoader` トレイト、同期ロード基盤            |
| `feature/p4c/asset-async-loading`    | 非同期ロード、進捗通知                            |
| `feature/p4d/asset-cache`            | アセットキャッシュ、参照カウント・解放             |

### P5: 描画エンジン（Filament）

| ブランチ                              | 主な内容                                          |
|--------------------------------------|---------------------------------------------------|
| `feature/p5a/filament-ffi-bindings`  | Filament C++ → Rust FFI バインディング            |
| `feature/p5b/render-command-buffer`  | `CommandBuffer`、描画コマンドキュー               |
| `feature/p5c/render-multi-view`      | ゲームビュー（Layer 0）+ UI ビュー（Layer 1）     |
| `feature/p5d/render-pbr-materials`   | PBR マテリアル、IBL 設定                          |
| `feature/p5e/render-post-process`    | ポストプロセス（ブルーム、トーンマッピング等）    |

### P6: サウンドシステム

| ブランチ                              | 主な内容                                          |
|--------------------------------------|---------------------------------------------------|
| `feature/p6a/audio-backend-kira`     | `kira` 統合、AudioManager 初期化                  |
| `feature/p6b/audio-bgm-player`       | BGM 再生・クロスフェード                          |
| `feature/p6c/audio-se-voice`         | SE / Voice チャンネル、ミキサー                   |

### P7: スクリプトエンジン

| ブランチ                              | 主な内容                                          |
|--------------------------------------|---------------------------------------------------|
| `feature/p7a/script-tokenizer`       | タグベーストークナイザ、字句解析                  |
| `feature/p7b/script-parser-ast`      | パーサ、AST 定義                                  |
| `feature/p7c/script-state-machine`   | ステートマシン、命令実行ループ                    |
| `feature/p7d/script-commands-basic`  | 基本命令（text, wait, jump, label）               |
| `feature/p7e/script-commands-adv`    | 高度命令（branch, call, gosub, variable）         |

### P8: UI システム

| ブランチ                              | 主な内容                                          |
|--------------------------------------|---------------------------------------------------|
| `feature/p8a/ui-taffy-layout`        | Taffy Flexbox レイアウトエンジン統合              |
| `feature/p8b/ui-signal-reactive`     | Signal 駆動リアクティブシステム                   |
| `feature/p8c/ui-sdf-text`            | SDF テキストレンダリング                          |
| `feature/p8d/ui-glassmorphism`       | Glassmorphism マテリアル、ブラー効果              |
| `feature/p8e/ui-animation`           | UI アニメーション、トランジション                 |

### P9: Live2D 統合

| ブランチ                              | 主な内容                                          |
|--------------------------------------|---------------------------------------------------|
| `feature/p9a/live2d-ffi-core`        | Cubism Core C++ FFI、モデルロード                 |
| `feature/p9b/live2d-render-texture`  | Render-to-Texture パイプライン                    |
| `feature/p9c/live2d-motion`          | モーション再生・ブレンド                          |
| `feature/p9d/live2d-lipsync`         | リップシンク（音声解析 → パラメータ反映）         |

### P10: VRM/FBX 統合

| ブランチ                              | 主な内容                                          |
|--------------------------------------|---------------------------------------------------|
| `feature/p10a/vrm-loader`            | VRM/GLB ロード、`gltf` クレート統合               |
| `feature/p10b/vrm-spring-bone`       | SpringBone 物理シミュレーション                   |
| `feature/p10c/vrm-blend-shape`       | BlendShape / MorphTarget 制御                     |
| `feature/p10d/fbx-import`            | FBX → GLB 変換パイプライン                        |

### P11: セーブシステム

| ブランチ                              | 主な内容                                          |
|--------------------------------------|---------------------------------------------------|
| `feature/p11a/save-data-schema`      | セーブデータ構造体、`serde` 実装                  |
| `feature/p11b/save-msgpack-io`       | MessagePack（`rmp-serde`）シリアライズ/IO         |
| `feature/p11c/save-migration`        | バージョンマイグレーション機構                    |
| `feature/p11d/save-slot-manager`     | スロット管理、サムネイル・メタデータ              |

### P12: ミニゲームプラグイン

| ブランチ                              | 主な内容                                          |
|--------------------------------------|---------------------------------------------------|
| `feature/p12a/minigame-plugin-trait` | `MinigamePlugin` トレイト、登録/呼び出しAPI       |
| `feature/p12b/minigame-2d-action`    | 2D アクションミニゲームプロトタイプ               |
| `feature/p12c/minigame-rpg-battle`   | RPG バトルミニゲームプロトタイプ                  |
| `feature/p12d/minigame-puzzle`       | パズルミニゲームプロトタイプ                      |
| `feature/p12e/minigame-rts`          | RTS ミニゲームプロトタイプ                        |

---
## 5. コミット前チェックリスト

エージェントがコミットを実行する前に以下を確認する：

- [ ] `cargo fmt --all` でフォーマット済み
- [ ] `cargo clippy --all-targets -- -D warnings` でエラー・警告なし
- [ ] `cargo check --all-targets` でビルドエラーなし
- [ ] 関連するテストが存在する場合 `cargo test` がパスしている
- [ ] コミットメッセージが本スキルのフォーマットに準拠している
- [ ] スコープが適切に設定されている

---
## 6. ベストプラクティス

- **atomic commits**: 1 コミットに複数の無関係な変更を含めない
- **rebase before PR**: main が進んでいる場合は `git rebase main` で最新化してから PR を出す
- **squash は慎重に**: 意味のあるコミット履歴は保持する。PR マージ時は `Squash and merge` より `Rebase and merge` を優先する
- **draft PR**: 実装途中でもレビューが必要な場合は Draft PR を活用する
- **コンフリクト解消**: コンフリクトは必ず手動で確認し、自動マージに依存しない

