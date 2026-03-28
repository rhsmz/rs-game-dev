# タスクリスト — rs-game-dev

> **最終更新**: 2026-03-28
> **ビルド状態**: `cargo test --workspace` 全パス（engine_core は 68 passed / 3 ignored）/ `cargo fmt` OK / `cargo clippy --workspace --all-targets` 警告 0 件
> **現在ブランチ**: （作業ブランチに依存）

## Phase 0: プロジェクト基盤 ✅

### ワークスペース初期化 (plan/00) ✅

- [x] ルート `Cargo.toml` のワークスペース定義（`[workspace]` / `members` / `resolver = "2"`）
- [x] `[workspace.package]` の設定（version, edition, license, rust-version）
- [x] `[workspace.dependencies]` の共通依存定義（serde, rmp-serde, anyhow, thiserror, log, env_logger）
- [x] `[workspace.lints.clippy]` の Clippy 共通リント設定
- [x] `rustfmt.toml` / `clippy.toml` の作成
- [x] `.gitignore` の設定
- [x] `engine_core` クレート作成 + `lib.rs` モジュール宣言
- [x] `engine_core` サブモジュール骨組み（ecs/, renderer/, audio/, asset/, script/, scene/, input/, save/, rng/, ffi/, util/）
- [x] `script_editor` クレート作成 + `lib.rs` / `main.rs` + サブモジュール骨組み
- [x] `game_player` クレート作成 + `main.rs`
- [x] `cargo build --workspace` / `cargo test --workspace` 成功確認

### ECS 基盤 (plan/01) ✅

- [x] `Entity` 構造体 + `EntityAllocator`（世代付き ID、free list 再利用、`is_alive` チェック）
- [x] `Component` トレイト + `ComponentStorage<T>`（SparseSet 方式、O(1) 挿入/削除/参照）
- [x] `AnyComponentStorage` トレイト（型消去によるストレージ管理）
- [x] `Resources`（`TypeId` ベースのグローバルリソース管理）
- [x] `System` トレイト + `FnSystem` + `into_system`
- [x] `Schedule`（System の登録と順次実行）
- [x] `World`（Entity 生成/破棄、Component 追加/削除、Resource 管理の統合コンテナ）
- [x] `EventQueue<T>`（型付きイベントキュー、send / drain / iter）
- [x] ECS ユニットテスト（Entity / Component / System / World / Event）

### ECS クエリ・フィルタ拡張 (PR #18, #19) ✅

- [x] `QueryParam` / `ReadOnlyQueryParam` トレイト（Sealed パターンで外部実装を禁止）
- [x] `With<T>` / `Without<T>` クエリフィルタ
- [x] `Query<Q>` / `QueryMut<Q>` クエリ実行オブジェクト（`iter` / `iter_mut` / `get` / `get_mut`）
- [x] `WorldQueryExt` トレイト（`world.query::<Q>()` / `world.query_mut::<Q>()`）
- [x] 2 要素タプルクエリ（`(&T1, &T2)`, `(&T1, With<T2>)`, `(&T1, Without<T2>)`, `(&mut T1, With/Without<T2>)` 等）
- [x] `ComponentStorage` 変更検知（changed フラグ、`iter_changed`、`clear_changed_flags`）
- [x] 基本 Component 定義（`Transform`, `Name`, `Active`）
- [x] `SystemStage`（`PreUpdate` / `Update` / `PostUpdate` 実行フェーズ）
- [x] `EntityAllocator::iter` の O(1) 最適化（alive bitmap）
- [x] `EventQueues`（`HashMap<TypeId, Box<dyn AnyEventQueue>>` による型消去コンテナ）
- [x] クエリ・フィルタ ユニットテスト（40 件全パス）

### ECS API 残タスク

- [x] `StorageIter` / `StorageIterMut` 名前付きイテレータの導入（現在は `impl Iterator` で返却）
- [x] `EventWriter<T>` / `EventReader<T>` ハンドル型の導入（現在は `EventQueue<T>` を直接操作）
- [x] スタブモジュールの Clippy 警告修正（asset, audio, input, renderer, rng, save, scene, script, ffi, util — 計 85 件: `Default` 未実装、`#[must_use]` 不足、`const fn` 化 等）

---

## Phase 1: コアサブシステム（並行開発可能）

### Renderer — Filament レンダラー統合 (plan/02)

- [x] Filament C API の FFI バインディング生成（`ffi/filament_sys.rs`）
- [x] `build.rs` でのライブラリリンク設定
- [x] `RenderEngine` ラッパー（Engine / Renderer / SwapChain の初期化・終了）
- [x] `GameView`（Perspective カメラ + PBR + IBL、Layer 0）
- [x] `UiView`（Ortho カメラ、深度テスト無効、Layer 1）
- [x] IBL / ライティング（HDR 環境マップ、DirectionalLight、SkyBox）
- [x] 2D スプライト描画（3D Quad + テクスチャ、Z-Order 制御）
- [x] Render-to-Texture（Live2D 合成用オフスクリーンターゲット）
- [x] ポストプロセス（Bloom、Tone Mapping、ガウシアンブラー）
- [x] ECS 連携（`MeshRenderer` / `Camera3D` Component、`render_system`）
- [x] ウィンドウ作成 + Filament 初期化テスト
- [x] マルチビュー描画テスト

### Audio — サウンドシステム (plan/03)

- [x] `kira` クレート依存追加
- [x] `GameAudioManager`（kira ラッパー、BGM / SE / Voice の 3 トラック構成）
- [x] BGM 制御（ロード、ループ再生、クロスフェード遷移、一時停止/再開）
- [x] SE 制御（重複再生対応、ワンショット再生、ボリューム/パン制御）
- [x] Voice 再生 + RMS 解析リップシンク（`calculate_lip_sync_value`）
- [x] CSV メタデータ連携（`sound.csv` から `SoundMeta` デシリアライズ）
- [x] ECS 連携（`AudioSource` Component、`AudioCommand` enum）
- [x] AudioManager 初期化 / BGM 再生 / RMS 数値テスト

### Input — 入力抽象化 (plan/04) ✅

- [x] `InputState`（キーボード / マウス / ゲームパッドの状態管理）
- [x] `is_pressed` / `is_just_pressed` / `is_just_released` フレーム判定
- [x] `ActionMap`（物理入力 → アクション名マッピング）
- [x] `InputEvent` enum（KeyPressed / MouseMoved / ActionTriggered）
- [x] ECS イベントキューへの入力送信
- [x] `winit` ウィンドウイベントからの変換
- [x] アクションマッピング解決テスト / フレーム間リセットテスト

### Dual RNG — 乱数エンジン (plan/05) ✅

- [x] `rand` / `rand_xoshiro` / `rand_chacha` 依存追加（`[workspace.dependencies]` 集約）
- [x] `VisualRng`（Xoshiro256++、演出用、スレッド RNG 由来の非決定論的起動シード）
- [x] `LogicRng`（ChaCha8、ロジック用、決定論的シード）
- [x] `derive_child_seed` によるシーン遷移時のシード・バケツリレー（親シード + `salt` で子 `LogicSeed` を導出）
- [x] ECS リソースとして登録（`world.insert_resource` で `LogicRng` / `VisualRng`）
- [x] 同一シード再現性テスト / 粗い一様分布（ビン頻度）テスト / `World` 登録テスト

---

## Phase 2: ゲームロジック基盤

### Asset Management — アセット管理 (plan/06)

- [ ] `AssetHandle<T>`（参照カウント方式、ロード状態追跡）
- [ ] `Asset` トレイト（`from_bytes` によるデコード抽象化）
- [ ] `AssetLoader`（非同期ファイル I/O、`tokio` / `async-std` ベース）
- [ ] `LocaleRouter`（`common_` 接頭辞ルーティング、言語別フォールバック `{lang}/ → ja/`）
- [ ] `AssetCache`（参照カウント、LRU、シーン遷移時パージ）
- [ ] CSV メタデータ読み込み（`chara.csv` / `sound.csv` / `bg.csv` / `images.csv` / `voices.csv`）
- [ ] `AssetDescriptor`（Scene の `required_assets()` 用、事前宣言リスト）
- [ ] `AssetError` エラー型定義（`error.rs`）
- [ ] ルーティング解決テスト / CSV デシリアライズテスト / キャッシュ管理テスト

### Script System — スクリプトエンジン (plan/07)

- [ ] タグベーススクリプト構文の設計確定
- [ ] レキサー（`Token` enum、タグ / 属性 / テキスト / コメントのトークナイズ）
- [ ] パーサ（`AstNode`、`[tag]...[/tag]` ネスト構造、属性 key="value" 解析）
- [ ] `ScriptCommand` enum（SetBackground / PlayBgm / ShowCharacter / ShowText / Choice / Jump / SetVariable / Conditional / StartMiniGame 等）
- [ ] AST → `ScriptCommand` への変換
- [ ] `ScriptVm`（ステートマシン、プログラムカウンタ、変数管理、コールスタック）
- [ ] VM 状態管理（Running / WaitingForClick / WaitingForChoice / WaitingForMiniGame / Finished）
- [ ] `ScriptError` エラー型定義（`error.rs`）
- [ ] ECS イベントキューとの連携（スクリプトコマンド → ECS イベント変換）
- [ ] レキサー / パーサ / VM / コマンド変換のユニットテスト

### Scene Management — シーン管理 (plan/08)

- [ ] `Scene` トレイト（`on_enter` / `update` / `render` / `on_exit` / `required_assets`）
- [ ] `SceneTransition` enum（None / Push / Pop / Replace / Quit）
- [ ] `SceneManager`（スタックベース管理、遷移処理）
- [ ] `SceneContext`（World / AssetLoader / Audio / Input / MiniGameResult へのアクセス）
- [ ] 遷移フロー実装（`on_exit` → アセットパージ → 非同期ロード → `on_enter`）
- [ ] `LoadingScreen`（進捗表示、ロード完了待ち）
- [ ] `TransitionEffect`（Fade / Slide エフェクト）
- [ ] シーンスタック Push/Pop/Replace テスト / ライフサイクル順序テスト

---

## Phase 3: 3D / Live2D 統合

### Live2D Integration (plan/09)

- [ ] Cubism Native SDK の C API FFI バインディング（`ffi/live2d_sys.rs`）
- [ ] `build.rs` での Cubism SDK リンク設定
- [ ] `Live2DModel` 安全ラッパー（MOC ロード、パラメータ get/set、update）
- [ ] Render-to-Texture 統合（Live2D 描画 → Filament テクスチャ → 3D Quad マッピング）
- [ ] モーション再生（`.motion3.json` パーサ、フェードイン/アウト、キュー管理）
- [ ] 表情ブレンド（複数表情の重み付きブレンド、モーション並行適用）
- [ ] 物理演算（揺れ物シミュレーション）
- [ ] ECS 連携（`Live2DComponent`、リップシンク値連携）
- [ ] スクリプト連携（`[chara motion="..." expression="..." lip_sync="on"]`）
- [ ] MOC ロード / モーション再生 / RMS リップシンクテスト

### VRM/FBX Integration (plan/10)

- [ ] `gltf` クレート依存追加
- [ ] VRM/GLB ローダー（glTF 2.0 パース → VRM 拡張データ抽出）
- [ ] メッシュ → Filament VertexBuffer / IndexBuffer 変換
- [ ] マテリアル → Filament Material 変換
- [ ] `Skeleton`（ボーン階層管理）
- [ ] `SpringBoneChain`（バーレ法物理シミュレーション、コライダー衝突判定）
- [ ] `BlendShapeProxy`（MorphTarget、表情プリセット、リップシンク用母音）
- [ ] `AnimationPlayer`（アニメーション再生、ブレンド遷移）
- [ ] ECS 連携（`Model3DComponent`、`spring_bone_update_system` / `animation_update_system`）
- [ ] GLB ロード / SpringBone 精度 / BlendShape 適用テスト

---

## Phase 4: UI / セーブ（並行開発可能）

### UI System — MVVM + Signal 駆動 (plan/11)

- [ ] `Signal<T>` リアクティブシステム（ファイングレイン変更検知、通知）
- [ ] `taffy` クレート依存追加
- [ ] `UiLayoutEngine`（Taffy Flexbox 連携、ノード追加、レイアウト計算）
- [ ] SDF テキストレンダラー（フォント → SDF アトラス、Filament カスタムマテリアル、文字送りアニメーション）
- [ ] `GlassStyle` Glassmorphism マテリアル（ガウシアンブラー + 半透明 + ボーダー）
- [ ] `MessageWindow` ウィジェット（テキスト表示、話者名、文字送り、Glassmorphism 背景）
- [ ] `ChoicePanel` ウィジェット（選択肢リスト、ホバー/選択状態）
- [ ] `StatusBar` / `Button` / `Slider` ウィジェット
- [ ] ECS ↔ UI 通信（`UiCommand` enum → mpsc、`ui_sync_system` → Signal 更新）
- [ ] Signal 変更検知テスト / Taffy レイアウト正確性テスト / Widget 状態遷移テスト

### Save System — セーブ/ロード (plan/12)

- [ ] `SaveFile` 構造体（version, timestamp, script_pos, global_vars, rng_seed, thumbnail 等）
- [ ] MessagePack シリアライズ / デシリアライズ（`rmp_serde` round-trip）
- [ ] `SaveManager`（スロット管理 `slot_000.sav`〜、オートセーブ / クイックセーブ）
- [ ] バージョンマイグレーション（`version` フィールドによる再帰的マイグレーション）
- [ ] サムネイルキャプチャ（Filament フレームバッファ読み取り → 128×72 JPEG）
- [ ] ECS 連携（`SaveCommand` enum、`save_system`）
- [ ] round-trip テスト / マイグレーションチェーンテスト / スロット管理テスト

---

## Phase 5: アプリケーション統合

### Script Editor — オーサリングツール (plan/13)

- [ ] エディタ UI シェル（MVVM + Signal、マルチパネルレイアウト、メニューバー）
- [ ] スクリプト編集パネル（シンタックスハイライト、タグ入力補完、エラー表示）
- [ ] ライブプレビューエンジン（`notify` ファイル変更検知、ホットリロード、現在行から再生）
- [ ] インスペクタ — Live2D パラメータスライダー
- [ ] インスペクタ — 3D モデル配置 / 回転 / スケール Gizmo
- [ ] インスペクタ — セーブデータ JSON 編集
- [ ] アセットブラウザ（ツリー表示、D&D 参照挿入、サムネイルプレビュー）
- [ ] FBX → GLB 変換（インポート / エクスポート）
- [ ] ホットリロード検知テスト / パラメータ変更反映テスト

### Game Player — 配布用ランタイム (plan/14)

- [ ] メインループ（固定タイムステップ + 可変レンダリング）
- [ ] `Config` ランタイム設定（`config.toml`、ウィンドウ / オーディオ / ロケール）
- [ ] `AssetArchive`（.pak ファイルリーダー、インデックス、LZ4 圧縮オプション）
- [ ] ウィンドウ管理（`winit`、フルスクリーン/ウィンドウ切替、リサイズ対応）
- [ ] Feature フラグによるデバッグ機能除外（`editor` / `debug_ui` feature）
- [ ] .pak アーカイブ作成 / 読み込みテスト
- [ ] config.toml パーステスト / メインループ起動/終了テスト

---

## Phase 6: CI/CD・ドキュメント・品質

### CI/CD

- [ ] GitHub Actions ワークフロー構築（`cargo fmt` / `cargo clippy` / `cargo test`）
- [ ] クロスプラットフォームビルド (Windows, macOS, Linux)
- [ ] リリースバイナリの自動生成
- [ ] コードカバレッジの計測と可視化

### ドキュメント

- [ ] README.md の充実（使い方、ビルド手順、アーキテクチャ図）
- [ ] `cargo doc` による API ドキュメント生成
- [ ] アーキテクチャ図の更新（Mermaid）
- [ ] CONTRIBUTING.md の作成
- [ ] CHANGELOG.md の運用開始

### 品質

- [ ] 全モジュールの Clippy 警告ゼロ化
- [ ] `cargo fmt` による統一フォーマット
- [ ] ベンチマークテスト（ECS クエリ性能、レンダリングフレームレート）
- [ ] 統合テスト（`tests/` ディレクトリ）の充実
