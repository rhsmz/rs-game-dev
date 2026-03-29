# Phase 1 総評レビュー（修正版）

作成日: 2026-03-29  
対象: `engine_core` Phase 1（Renderer / Audio / Input / RNG）

---

## エグゼクティブサマリー

- **総合判定**: **Phase 1 は「条件付きで完了」**（評価: **7.6 / 10**）。
- **判断理由（要約）**:
  - Renderer はフレーム制御・2ビュー描画順・リサイズ同期まで実装済み。
  - Audio は ECS コマンド実行系（queue drain → manager 適用）が成立。
  - Input / RNG は実装・テスト密度とも高水準を維持。
  - ただし Renderer の「実メッシュ描画の本番品質検証」と、CI の強制力は未完。
- **提言**: Phase 2 は進めてよいが、**P0（描画の実幾何パス検証 / 品質ゲート強化）を並走必須**とする。

---

## 1. 観点①: ゲームエンジンのコアロジックとして完成されているか

### 判定: **概ね YES（7.4 / 10）**

### 1-1. 成立している点

1. **Renderer の最小実行ループが成立**
   - `begin_frame()` / `end_frame()` / `resize()` が実装され、1フレーム制御が可能。
   - `GameView` と `UiView` の実体（Scene / View / CameraBinding）生成・破棄が成立。

2. **描画順序ポリシーが実装されている**
   - `render_system` で **GameView → UiView** の順序を固定。
   - カメラ不在時の挙動（メッシュ投入スキップ）も仕様として明記。

3. **Audio の ECS 実行系が実用レベルで接続済み**
   - `AudioCommandQueue` から FIFO で drain し、`GameAudioManager` へ適用。
   - ファイル欠損・デコード失敗時は warn + skip で継続（運用上の妥当性あり）。

4. **Input / RNG は再現性・取り回しとも良好**
   - フレーム遷移を前提にした入力状態管理とイベント連携が整っている。
   - RNG は deterministic / non-deterministic を分離し、seed relay を維持。

### 1-2. 未達・注意点

- **Renderer の本番DoD（実幾何描画品質）**は未達。
  - 現時点は「縦スライスとしての描画経路確認」が中心。
  - マテリアル・ジオメトリ・可視結果の回帰保証は今後の追加が必要。

### 1-3. 結論

- 「コアロジックの骨格」は成立。  
- ただし「製品品質の描画完成」とはまだ区別すべき段階。

---

## 2. 観点②: ソースコードがエンタープライズ品質か

### 判定: **改善済みだが未完成（7.9 / 10）**

### 2-1. 良い点

1. **安全性ルールの明文化がある**
   - `SAFETY.md` に `unsafe impl Send/Sync` の前提と禁止事項を記載。

2. **未実装公開APIの危険度が下がっている**
   - 主要経路で `not implemented yet` 依存は解消方向。

3. **DoD運用の素地がある**
   - `TASKS.MD` に検証コマンド・失敗時挙動・証跡観点が記録されている。

4. **統合観点のテストが存在する**
   - Renderer: 1フレーム smoke + 描画順検証。
   - Audio: queue drain の順序検証（デバイス非依存）。

### 2-2. 企業品質観点での不足

1. **CIでの強制力不足**
   - 「実行すべきチェック」が運用上必須でも、常時自動担保の設計が不足。

2. **visual regression の未有効化**
   - Game/UI 重畳の可視回帰テストが placeholder 段階。

3. **デバイス依存経路の定期検証設計不足**
   - Audio 実デバイス経路の nightly / 環境分離戦略を明文化すべき。

### 2-3. 結論

- コードの方向性は enterprise 寄り。
- ただし **「ルールがある」から「CIで強制される」へ進めること**が必要。

---

## 3. 観点③: 修正タスク（子タスク・孫タスク）

> 目的: Phase 2 の結合負債を抑えつつ、Phase 1 を「動く」から「壊れにくい」に引き上げる。  
> 優先度: **P0 > P1 > P2**

### P0（必須）

#### P0-1. Renderer 実幾何描画の閉塞

- **子タスク**
  1. `MeshRenderer` 実体を Filament の renderable と結線
  2. Camera 行列（view/proj）反映の検証
  3. 自動テストで「1メッシュ描画成功」を保証

- **孫タスク**
  - 1.1 `renderable_id` と内部ハンドルの対応テーブルを導入。
  - 1.2 未ロードメッシュ時の挙動を warn + skip に統一。
  - 2.1 `resize` 後も投影更新が破綻しないことをテスト化。
  - 2.2 カメラ複数体時の採用ポリシー（最初/priority）を仕様化。
  - 3.1 `filament_smoke` を拡張し、submit 実行の成否を判定。
  - 3.2 将来の visual golden test 導入条件を明文化。

- **DoD**
  - `--features filament` で「clear + 1mesh + UI overlay」が自動検証で通る。

#### P0-2. Audio 実行系の運用品質向上

- **子タスク**
  1. 競合コマンド（Stop/Play）の優先ルール固定
  2. 異常系の観測性強化（ログキー標準化）
  3. fake backend テストの異常系追加

- **孫タスク**
  - 1.1 `StopBgm > PlayBgm` 等を明文化。
  - 1.2 FIFO と優先ルールの境界ケースをテスト追加。
  - 2.1 `seq/source/op/err_kind` を必須ログキー化。
  - 2.2 ファイル欠損・decode失敗のログをテストで検証。
  - 3.1 `audio-kira` 有無の差分を CI 出力で可視化。
  - 3.2 queue drop 件数のしきい値監視を追加。

- **DoD**
  - 正常系/異常系とも、デバイス非依存で再現可能。

#### P0-3. 完了定義（DoD）の判定ブレ防止

- **子タスク**
  1. 各タスクに「縦スライス完了 / 本番要件完了」を併記
  2. チェック更新時の証跡リンク（テストID/コマンド）必須化

- **孫タスク**
  - 1.1 Renderer / Audio の各項目へ検証コマンドを紐付け。
  - 1.2 失敗時挙動（warn/skip/fail）を必須項目化。
  - 2.1 PR テンプレに検証証跡欄を追加。
  - 2.2 `#[ignore]` のみで完了扱いしない運用を固定。

- **DoD**
  - 再レビュー時、同じ手順で同じ判定に到達できる。

### P1（推奨）

#### P1-1. CI マトリクスの強制運用

- **子タスク**
  1. `default` / `--no-default-features` / `--features filament` をCI化
  2. OS軸（Linux + macOS）で最低1ジョブ運用

- **孫タスク**
  - 1.1 `audio_command_drain` を常時実行。
  - 1.2 `filament_smoke` は条件付き実行で安定運用。
  - 2.1 失敗ログへ feature/OS/backend を添付。

- **DoD**
  - ローカル依存だった品質ゲートが CI 上で再現される。

#### P1-2. unsafe レビュー運用の制度化

- **子タスク**
  1. `SAFETY.md` 準拠チェックを PR テンプレ化
  2. 新規 `unsafe` の証跡（理由・前提・境界）必須化

- **孫タスク**
  - 1.1 `Send/Sync` 手書き時にスレッド制約記載を必須化。
  - 2.1 `// SAFETY:` 欠落をレビューNG条件へ。

- **DoD**
  - unsafe 変更の監査可能性が保たれる。

### P2（余力）

#### P2-1. 可観測性の継続改善

- **子タスク**
  1. Renderer / Audio の定点メトリクス設計
  2. nightly で回帰トレンド監視

- **孫タスク**
  - 1.1 frame skip 率・audio drop 率を標準メトリクス化。
  - 1.2 閾値超過時の通知導線を整備。

- **DoD**
  - 障害の「発生後対応」ではなく「予兆検知」が可能。

---

## 最終結論

- Phase 1 は「最低限動く状態」から「次段階に進める状態」へ改善済み。
- ただし enterprise 品質としては、**P0（実幾何描画検証 + CI強制）**が依然クリティカル。
- よって方針は **「Phase 2 着手可、ただしP0並走を必須条件化」** が最も安全。

---

## 反映メモ（リポジトリ対応・2026-03-29）

以下は本レビューの **P0-2 / P0-3 / P1-1 の一部** をコード・CI・ドキュメントへ反映した記録である（実メッシュ Filament 結線＝P0-1 は未着手のまま）。

- **Audio P0-2**: `command_dispatch` に FIFO 優先ルール・必須ログキー（`seq` / `source` / `op` / `err_kind`）の明文化、`StopBgm` / `SetVolume` / `PlayVoice` trace の `op=` 統一、マネージャ不在時 **16 件以上** ドロップで bulk warn、境界テスト（`test_fifo_*` / `test_audio_command_system_drops_bulk_without_manager`）。
- **DoD P0-3**: PR テンプレに「検証証跡」表、`TASKS.MD` に `#[ignore]` 運用ルールと検証コマンド早見表、`plan/15` に検証コマンド表と P0-3 ログ追記。
- **CI 可視化**: `cargo tree -p engine_core -e features --prefix depth` を各行列で実行。

### 追記（2026-03-29 以降・P0-1 / P1 フォロー）

- **Renderer P0-1（縦スライス）**: `Scene_submit_mesh_vertical_slice`（スタブ＋`filament_bridge.cpp` のプレースホルダ）を追加し、`render_system` が Game `Scene` に対して `MeshRenderer` を走査投入。`renderable_id==0` は warn + skip。`ENGINE_CORE_MESH_SUBMIT_TRACE` と `take_mesh_submit_trace` で smoke が受理回数を検証。`RenderableManager` 実結線はブリッジ TODO。
- **Audio P0-2 残**: `format_audio_warn_message` の単体テストで必須ログキーを検証（`#[cfg(any(test, feature = "audio-kira"))]` で no-default-features の dead_code を回避）。
- **CI P1-1**: `macos-latest` 行列を追加、ジョブ先頭で OS / args / `rustc -vV` を出力。Filament smoke は macOS で `RUST_TEST_THREADS=1`。
- **PR P1-2**: `pull_request_template.md` に `SAFETY.md` / `// SAFETY:` / 手書き `Send`/`Sync` の明示項目を追加。
- **計画 doc**: `plan/15_phase2_readiness_plan.md` に visual golden 導入条件を追記。

### 追記（`plan/15`・`TASKS.MD` Phase 1-alpha 同期・同一日続報）

- **CI P1-1 完走**: `windows-latest` / `macos-latest` に **`--no-default-features`** 行を追加し、ubuntu と同様にスタブ音声＋既定 feature の両方を全 OS で検証。
- **P0-1 設計寄り**: `RenderableManager` 実装は未着手のまま、**対応表・ライフサイクル方針**を `crates/engine_core/src/renderer/mod.rs` のモジュール doc と `plan/15` 実施ログに明文化。
- **B1-1**: `render_system`（モジュール doc）と `audio_command_system` の rustdoc に前提・縮退動作を追記。
- **P2-1 たたき台**: 定点メトリクス候補名を `plan/15` 実施ログに列挙（実装・nightly は未着手）。
- **rustdoc**: `kira.rs` の `purge_faded_tracks` 参照リンクを `Self::purge_faded_tracks` に修正。

### 追記（P0-1 `RenderableManager` 実結線）

- **`filament_bridge.cpp`**: `Scene_submit_mesh_vertical_slice` が `VertexBuffer` / `IndexBuffer` / `RenderableManager` で三角形を構築し、`Engine::getDefaultMaterial()->getDefaultInstance()` をバインド。`renderable_id` ごとに 1 エンティティ（同一 ID の再呼び出しはスキップ）。`Scene_destroy` で C++ グローバルレジストリからエンティティとバッファを `Engine::destroy`。
- **スタブ**（`FILAMENT_LIB_DIR` 未設定）の挙動は変更なし。CI は引き続きスタブリンク。
