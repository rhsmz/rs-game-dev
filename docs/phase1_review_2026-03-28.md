# Phase 1 総評レビュー（客観・厳しめ）

作成日: 2026-03-28  
対象: `engine_core` の Phase 1（Renderer / Audio / Input / RNG）

## エグゼクティブサマリー

- **総合判定**: **Phase 1 は「完了」ではなく「部分完了」**（厳密評価: **5.5 / 10**）。
- **強み**: Input / RNG は実装密度・テスト密度が高く、ECS 連携の最低限が成立。
- **弱み**: Renderer は公開 API が雛形中心で、`not implemented` を返す経路が残存。Audio も ECS 実行系（コマンド適用システム）が未接続。
- **結論**: **Phase 2 全面移行の前に、Renderer 実装とシステム結線を優先して閉じるべき**。

---

## 1) 「ゲームエンジンのコアロジックとして完成されているか」

### 判定: **未完成（5.5 / 10）**

#### 根拠（ファクト）

1. **Renderer の中核処理が未実装**
   - `RenderEngine::begin_frame()` が常に `false` を返す。
   - `render_system` は no-op（描画コマンド未生成）。
   - `SkyBox.apply_to` / `SpriteRenderer.render` / `RenderTarget.apply_for_live2d` / `PostProcessPipeline.apply_to` が `not implemented` エラーを返す。

2. **マルチビュー要件とのギャップ**
   - `GameView` / `UiView` は Filament 実体（`View` / `Scene`）未生成のまま `None`。
   - `plan/02_renderer.md` の完了条件（基本メッシュ描画、2ビュー同時レンダリング、IBL適用）と現状実装に差分がある。

3. **Audio の ECS 結線不足**
   - `GameAudioManager` は API を持つが、ECS 側は `AudioSource` / `AudioCommand` の型定義中心。
   - `AudioCommand` を消費して `GameAudioManager` に適用するシステムが確認できない。

4. **Input / RNG は比較的高完成度**
   - Input は押下フェーズ管理やイベント送受信テストを備える。
   - RNG は決定論 / 非決定論の分離と seed relay を実装している。

#### 厳しめコメント

- 「Phase 1 完了」を名乗るには、描画経路の未実装が致命的。
- コアロジックのうち最重要パス（Renderer）が未閉塞のため、実運用準備は未了。

---

## 2) 「ソースコードがエンタープライズ品質か」

### 判定: **中位（6.5 / 10）**

#### 良い点

- ワークスペースで lint 方針を統一し、`TASKS.MD` に品質タスクが管理されている。
- ECS / Input / RNG は責務分割と API の一貫性があり、ユニットテストも整備されている。
- コメントと doc は比較的丁寧で、保守者が意図を追いやすい。

#### 懸念点

1. **「実装済み」と「雛形」の混在が大きい**
   - 同一フェーズ内で成熟度の差が大きく、完了判定がブレやすい。

2. **未実装公開 API が runtime error を返す**
   - `anyhow!("... not implemented yet")` が公開 API に残っており、上位統合時の障害要因。

3. **unsafe 境界の運用ルールが弱い**
   - `unsafe impl Send/Sync` が存在するが、メインスレッド制約を実行モデルとして強制する仕組みが弱い。

4. **テスト再現性の課題**
   - Audio の一部テストが `#[ignore]` で、CI で実経路を常時検証しづらい。

#### 厳しめコメント

- 「方針」は良いが「完了条件の自動検証」が弱く、企業開発で求める運用品質には未達。

---

## 3) Phase 2 着手前に改善すべき内容（優先度付き・詳細WBS）

> 目的: P0 を最短で閉じ、Phase 2 での結合負債発生を抑える。  
> 目安: **P0 = 1〜2 スプリント、P1 = 並行 1〜2 スプリント、P2 = 余力対応**

### P0（必須）

#### P0-1. Renderer DoD を満たす最小縦スライスを完成
- **子タスク**
  1. `RenderEngine` フレームライフサイクル実装
  2. `GameView` / `UiView` 実体生成
  3. `render_system` で描画コマンド生成
  4. 最低1オブジェクト描画の smoke test 化
- **孫タスク**
  - 1.1 `begin_frame()` で Filament begin を呼び、戻り値を実状態で返す。
  - 1.2 `end_frame()` で submit/present 相当を接続。
  - 1.3 resize 時の swap chain / viewport 更新を定義。
  - 2.1 `GameView::new` で `View_create`/`Scene_create` を接続。
  - 2.2 `UiView::new` で Ortho 設定 + 深度無効設定を接続。
  - 2.3 View/Scene の破棄責務を `Drop` か専用ラッパで明文化。
  - 3.1 `MeshRenderer` と `Camera3D` をクエリし、Renderable を submit。
  - 3.2 カメラ未存在時の挙動（skip / error）を仕様化。
  - 3.3 render order（Game→UI）の固定とテスト。
  - 4.1 `--features filament` で 1フレーム描画成功を CI smoke に追加。
  - 4.2 失敗時ログ（初期化失敗・描画失敗）を識別可能に整備。
- **完了条件（DoD）**
  - カラークリア + 1メッシュ描画 + UIオーバーレイが1テストで通る。

#### P0-2. 未実装公開 API の棚卸しと扱い統一
- **子タスク**
  1. `not implemented yet` を返す API 一覧化
  2. API ごとの方針決定（実装/非公開/stub退避）
  3. 互換性への影響評価（SemVer 観点）
- **孫タスク**
  - 1.1 `SkyBox.apply_to` / `SpriteRenderer.render` / `RenderTarget.apply_for_live2d` / `PostProcessPipeline.apply_to` を台帳化。
  - 1.2 依存先モジュールと呼び出し元を追跡し、利用実態を可視化。
  - 2.1 直近必須 API は最小 no-op 成功実装に置換（失敗返却を減らす）。
  - 2.2 すぐ実装しない API は `pub(crate)` 化 or feature gated に変更。
  - 2.3 `TASKS.MD` に「雛形完了」と「実装完了」を分離記録。
  - 3.1 破壊的変更の有無を changelog 下書きに反映。
  - 3.2 呼び出し側の代替経路（ガード）を追加。
- **完了条件（DoD）**
  - Phase 1 範囲の公開 API から `not implemented yet` を解消、または非公開化。

#### P0-3. Audio の ECS 実行系を追加
- **子タスク**
  1. `AudioCommand` 消費システム実装
  2. `GameAudioManager` への適用層（adapter）実装
  3. 実行順序（stage）定義
  4. E2E テスト追加
- **孫タスク**
  - 1.1 EventQueue から `AudioCommand` を drain し、コマンドディスパッチ。
  - 1.2 `PlayBgm/StopBgm/PlaySe/PlayVoice/SetVolume` を最小対応。
  - 2.1 ファイルロード失敗時のリトライ/スキップ方針を決定。
  - 2.2 音量単位（dB/linear）を API 境界で統一。
  - 3.1 Update or PostUpdate のどちらで適用するか固定。
  - 3.2 同フレーム競合コマンドの優先順位（Stop > Play など）を定義。
  - 4.1 Input → ECS Event → AudioSystem の統合テストを追加。
  - 4.2 デバイス非依存テスト用の fake backend を導入。
- **完了条件（DoD）**
  - ECS イベント駆動で BGM/SE/Voice の最低操作が再現可能。

### P1（推奨）

#### P1-1. 品質ゲートを「宣言」から「強制」へ
- **子タスク**
  1. CI feature マトリクス追加
  2. Renderer smoke 基準引き上げ
  3. 失敗時の診断性向上
- **孫タスク**
  - 1.1 `default`, `--no-default-features`, `--features filament` の3軸実行。
  - 1.2 Linux/macOS で最低1ジョブずつ実施（可能なら）。
  - 2.1 「初期化OK」から「1フレーム描画完了」へ成功条件を変更。
  - 2.2 Audio は `#[ignore]` 以外で通るロジックテストを増やす。
  - 3.1 失敗ログに feature・OS・バックエンド情報を含める。

#### P1-2. unsafe 設計を文書化
- **子タスク**
  1. `unsafe impl Send/Sync` の根拠整理
  2. 実行モデル制約の明文化
  3. レビュー観点の定義
- **孫タスク**
  - 1.1 `GameView` / `UiView` の `unsafe impl` 前提（メインスレッド限定）を文書化。
  - 1.2 将来破綻するケース（別スレッドアクセス）を禁止事項化。
  - 2.1 `SAFETY.md` 作成、関連箇所にリンク。
  - 3.1 PR テンプレに「unsafe チェック項目」を追加。

#### P1-3. 進捗管理の評価軸を実行可能性へ寄せる
- **子タスク**
  1. 完了定義（DoD）テンプレ作成
  2. `TASKS.MD` 反映ルール更新
- **孫タスク**
  - 1.1 各項目に「実行条件・テスト証跡・失敗時挙動」を必須化。
  - 2.1 チェックボックス更新時に対応テストIDを記録。

### P2（余力）

#### P2-1. 統合テスト拡充
- **子タスク**
  1. Input→ECS→System の E2E
  2. RNG seed relay の擬似シーン遷移テスト
  3. Audio/Renderer 横断 smoke
- **孫タスク**
  - 1.1 `ActionTriggered` が期待システムで消費されることを検証。
  - 2.1 同一 seed で同一結果、異なる salt で差分を検証。
  - 3.1 低頻度でも nightly 実行し、回帰を早期検知。

---

## 最終結論

- **現状の Phase 1 は、Input/RNG は良好、Audio は中間、Renderer は未完。**
- よって、**Phase 2 に全面移行する前に Renderer と ECS 結線を閉じるべき**。
- この順序を守らない場合、Phase 2（Asset/Script/Scene）で結合負債が増幅する。
