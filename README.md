# 🚀 次世代ハイブリッド・ゲームエンジン：完全詳細設計仕様書

## 1. 開発ビジョンとコア・コンセプト

本プロジェクトは、Rustのメモリ安全性・並行処理性能を基盤に、Google製の物理ベースレンダラー **Filament** を採用。「**Live2Dによる最高峰の2D表現**」と「**VRMによる次世代3D表現**」を単一のスクリプト環境で制御し、3Dライティングの影響を受ける2D表現や3Dミニゲームとのシームレスな融合を、プログラミング知識のないクリエイター（学生）でも実現できる環境を提供する。

---

## 1.1 開発・CI環境の前提（システム依存ライブラリ）

本リポジトリでは、Rustクレート依存に加えて OS 側のライブラリが必要です。  
特に `engine_core` は音声バックエンドとして `kira`（`alsa-sys` 経由）を利用するため、Linux では ALSA の開発パッケージが必要です。

- Linux（Debian/Ubuntu系）: `libasound2-dev`（`alsa.pc` を提供）
- Linux（Fedora/RHEL系）: `alsa-lib-devel`
- Linux（Arch系）: `alsa-lib`

`cargo test` / `cargo clippy` 実行時に `alsa.pc not found` が出る場合は、上記パッケージを先に導入してください。

---

## 2. システムアーキテクチャ (Crate Structure)

Rustのワークスペース機能を活用し、以下の3つの主要コンポーネントで構成する。全体のデータ・ロジック管理には **ECS (Entity Component System)** を採用し、高いパフォーマンスと拡張性を確保する。

### 2.1 `engine_core` (心臓部・Model)

* **ECS基盤**: すべてのゲーム内オブジェクト（3Dモデル、Live2D、UIノード）をEntityとして扱い、ComponentとSystemでロジックを分離・並行処理する。
* **描画エンジン**: Filament PBRレンダラーのラップ、Live2D Cubism SDK Nativeの統合、VRM/FBXローダー。
* **サウンド**: `kira` または `rodio` をベースとした、BGM/SE/Voiceのミキシングとクロスフェード制御。
* **アセット管理**: 非同期ロード、メモリキャッシュ、言語別ルーティング。
* **スクリプト解析**: タグベース構文のパーサおよび実行ステートマシン。

### 2.2 `script_editor` (オーサリングツール)

* **UI基盤**: Floem思想（シグナル駆動リアクティブ）を継承したFilamentネイティブUI。
* **ライブプレビュー**: スクリプトの変更を検知し、即座にゲーム画面に反映（ホットリロード）。
* **ビジュアル編集**: Live2Dパラメータのスライダー調整、3Dモデルの配置、セーブデータのJSON編集機能。

### 2.3 `game_player` (配布用ランタイム)

* **最適化**: デバッグ機能を削ぎ落とした軽量コンテナ。アセットアーカイブの解読とメインループの実行に特化。

---

## 3. レンダリング ＆ UI システム (View ＆ ViewModel)

### 3.1 マルチビュー・レンダリング・スタック

* **Game View (Layer 0)**:
* **3D描画 (VRM/FBX)**: PBR、IBL（環境光）、影、ポストプロセス。
* **Live2D描画**: FilamentのテクスチャとしてLive2Dのレンダリング結果を書き込む「Render-to-Texture」方式、またはコマンドバッファへの直接介入。
* **2Dスプライト**: 3D空間内のQuadとして配置。2DでありながらIBLによるライティングやブルームの恩恵を受ける。


* **UI View (Layer 1)**: 最前面Orthoカメラ。深度テスト無効。

### 3.2 UIアーキテクチャ (MVVM & Signal駆動)

Rustの借用チェッカー（Borrow Checker）との競合を避け、保守性を高めるため **MVVM (Model-View-ViewModel) アーキテクチャ** を採用する。

* **レイアウトエンジン (View)**: **Taffy** を採用。Flexboxアルゴリズムに基づき、Rust側でUIノードの座標・サイズを計算。状態を持たず、ViewModelからのシグナルを監視して描画を更新する。
* **状態管理 (ViewModel)**: Floem等で採用される `Signal<T>` を用いたファイングレイン（きめ細かい）リアクティブシステム。UIに必要なデータ（スライダーの現在値やテキスト）を保持し、変更があったUIコンポーネントのみをピンポイントで再計算・再描画する。
* **描画詳細**: **SDF (Signed Distance Field)** 方式による劣化のないテキスト描画。角丸、グラデーション、背景ぼかし（Glassmorphism）をサポートする専用マテリアル。

---

## 4. データフローとアーキテクチャ連携 (ECS × MVVM)

UI（MVVM）とゲームロジック（ECS）は直接参照し合わず、**メッセージパッシング（イベントキュー）**を用いた疎結合な連携を行うことで、Rustにおける所有権の安全性を担保する。

### 4.1 UIからECSへの伝達 (コマンドキューによる入力)

ユーザーのUI操作は、直接ECSのデータを書き換えるのではなく、メッセージとして送信される。

1. **View**: ユーザーがUI（例：キャラクタースケールのスライダー）を操作。
2. **ViewModel**: 入力を受け取り、更新コマンド（例: `Command::UpdateScale { id, value }`）を生成し、マルチスレッド安全なチャネル（`std::sync::mpsc` 等）またはECSのイベントキューに送信。
3. **Model (ECS)**: 次のフレームのループ処理時、入力ハンドリング用のSystemがキューからコマンドを受信し、該当するEntityのComponentを安全に更新する。

### 4.2 ECSからUIへの伝達 (変更検知とSignal更新)

ゲーム内の状態変化をUIに反映させる際は、ECS側の変更検知を利用する。

1. **Model (ECS)**: スクリプト進行やゲームロジックにより、EntityのComponent（例: HPや進行ステータス）が更新される。
2. **ViewModel (中継)**: UI同期用のSystemがComponentの変更（Change Detection）を検知し、対応するViewModel上の `Signal<T>` を新しい値で更新する。
3. **View**: 該当する `Signal` を購読しているUIノードだけが変更を検知し、TaffyレイアウトとFilamentの描画を再構築する。

---

## 5. アセット管理 ＆ ローカライズ (Convention over Configuration)

### 5.1 ディレクトリ・ルーティング規則

1. **共通リソース (`common_` 接頭辞)**: `assets/common/` を参照。全言語で共有されるサウンド、UI。
2. **ローカライズリソース (接頭辞なし)**: `assets/{current_lang}/` を優先探索。
3. **自動フォールバック**: 現在の言語に素材がない場合、自動的に `assets/ja/` を参照。

### 5.2 ディレクトリ構造

```text
assets/
├── common/             # 全言語共通（サウンド、共通パーツ）
└── ja/                 # 日本語（デフォルト・マスター）
    ├── chara/          # Live2D(.model3.json), VRM, FBX, PNG
    ├── bg/             # 背景画像、HDR
    ├── voice/          # 音声ファイル
    ├── metadata/       # 各種CSVファイル
    └── image/          # 汎用画像、CG

```

---

## 6. メタデータ詳細定義 (CSV形式)

すべてのメタデータは `serde` を介して読み込まれる。

### 6.1 キャラクター・モデル管理 (`chara.csv`)

| 項目名 | 型 | 説明 |
| --- | --- | --- |
| `chara_id` | String | スクリプトから参照する一意のID |
| `type` | Enum | `LIVE2D` / `VRM` / `FBX` / `2D_SPRITE` |
| `file_path` | String | 基本パス（接頭辞ルール適用） |
| `anchor_x/y` | f32 | 表示中心点 (0.0 〜 1.0) |
| `base_scale` | f32 | Filament空間内での基本スケール |
| `physics` | Enum | `ENABLED` / `DISABLED` (VRM/Live2Dの揺れ物) |
| `metadata` | String | Live2Dの `.model3.json` や表情定義へのパス |

### 6.2 音響管理 (`sound.csv`)

| 項目名 | 型 | 説明 |
| --- | --- | --- |
| `sound_id` | String | スクリプト参照ID |
| `type` | Enum | `BGM` / `SE` |
| `file_path` | String | ファイルパス |
| `volume` | f32 | 基本音量 (0.0 〜 1.0) |
| `loop_start/end` | f32 | ループ区間（秒）。0なら末尾まで。 |

### 6.3 背景・画像・台詞管理 (`bg.csv`, `images.csv`, `voices.csv`)

* **`bg.csv`**: 背景タイプ (`STATIC`, `PANORAMA`, `3D_SCENE`)、IBL強度、ぼかし係数。
* **`images.csv`**: イベントCG、アイテムアイコン、ギャラリー解禁フラグ。
* **`voices.csv`**: `voice_id` と字幕テキスト、音声パス、個別音量。

---

## 7. 技術スタック ＆ 内部実装

### 7.1 Live2D Cubism 統合

* **SDK**: Cubism Native SDK (C++) をRustにリンク。
* **自動リップシンク**: `voices.csv` 指定の音声データのRMS（実効値）を解析し、Live2Dの口パクパラメータにリアルタイム反映。
* **モーション**: `.motion3.json` の再生と表情のブレンド。

### 7.2 VRM / FBX 統合

* **VRM**: `SpringBone` による揺れ物シミュレーション。`BlendShapeProxy` をFilamentのMorphTargetにマッピング。
* **FBX**: 複雑な階層・アニメーションを保持。エディタ側でGLBへ事前変換し、高速なロードを実現。

### 7.3 乱数エンジン (Dual RNG)

* **VisualRng**: Xoshiro256++。演出用の非決定論的乱数。
* **LogicRng**: ChaCha8。ミニゲーム等のための決定論的乱数。シーン遷移時に「シード・バケツリレー」を行い、リプレイ性と再現性を確保。

---

## 8. 永続化 ＆ シーン管理

### 8.1 シーンベース・アーキテクチャ

全てのパートを `Scene` トレイトで抽象化。

* **移行フロー**: 現シーン破棄 ➔ アセット完全パージ ➔ 新シーン非同期ロード ➔ データ展開。

### 8.2 セーブデータ (MessagePack)

* **形式**: **MessagePack** (`rmp-serde`)。
* **構造**:

```rust
pub struct SaveFile {
    pub version: u32,               // 互換性マイグレーション用
    pub timestamp: u64,             // UNIXタイム
    pub script_pos: (String, usize), // ファイル名, 行番号
    pub global_vars: HashMap<String, i32>, // 好感度、フラグ等
    pub live2d_state: Vec<Param>,   // Live2Dのポーズ復元用
    pub mini_game_blob: Vec<u8>,    // ミニゲーム専用シリアライズデータ
    pub thumbnail: Option<Vec<u8>>, // JPEGバイナリ
}

```

---

## 9. 内部描画ルール (Z-Order)

1. **Background**: Z = 10.0
2. **3D/Live2D Layer**: Z = 0.0 〜 5.0 （重なりは描画順で制御）
3. **Characters**: Z = -1.0 〜 -5.0
4. **UI/Text**: Z = -10.0 (最前面・Orthoカメラ)
