# アーキテクチャ規約

## 全体方針

本エンジンは **ECS × MVVM** のハイブリッドアーキテクチャを採用する。ゲームロジック（ECS）と UI（MVVM）を直接参照せず、メッセージパッシング（イベントキュー）で疎結合に連携する。

## ECS (Entity Component System)

### 基本ルール
- すべてのゲームオブジェクト（3Dモデル、Live2Dキャラ、スプライト、サウンドソース）は **Entity** として管理
- **Component** はデータのみ保持し、ロジックを持たない
- **System** はロジックのみ記述し、状態を保持しない
- System 間の依存は明示的に宣言し、実行順序を制御

### Component 設計
```rust
// ✅ データのみのComponent
pub struct Transform {
    pub position: Vec3,
    pub rotation: Quat,
    pub scale: Vec3,
}

// ❌ メソッドにロジックを持たせない
// → System に記述する
```

### System 設計
- System は単一責務を持つ（1 System = 1 関心事）
- 副作用は Command パターンで遅延実行
- クエリは必要最小限のコンポーネントに絞る

## MVVM (UI アーキテクチャ)

### レイヤー構成
| レイヤー | 責務 | 技術 |
|----------|------|------|
| **View** | UIノードの描画・レイアウト | Taffy (Flexbox), Filament 描画 |
| **ViewModel** | UI状態保持、`Signal<T>` 管理 | Signal 駆動リアクティブ |
| **Model** | ECS のゲームデータ | Component / Resource |

### 通信フロー

#### UI → ECS (コマンドキュー)
```
View → ViewModel → Command → mpsc::channel → ECS System
```
- ユーザー操作を ViewModel が `Command` enum に変換
- チャネル経由で ECS の入力ハンドリング System に送信
- 次フレームで ECS が安全に処理

#### ECS → UI (Signal 更新)
```
ECS Component (変更検知) → ViewModel (Signal<T> 更新) → View (再描画)
```
- UI 同期 System が Component の変更を検知
- 対応する ViewModel の `Signal<T>` を更新
- 購読している View ノードだけが再描画

### 禁止事項
- ❌ View から ECS Component を直接読み書き
- ❌ ECS System から View ノードを直接操作
- ❌ ViewModel にゲームロジックを記述

## Scene トレイト

すべてのゲームパート（ADVパート、ミニゲーム、タイトル画面等）は `Scene` トレイトを実装する。

```rust
pub trait Scene: Send + Sync {
    /// シーンの初期化（アセットロード完了後に呼ばれる）
    fn on_enter(&mut self, ctx: &mut SceneContext);

    /// 毎フレーム更新
    fn update(&mut self, ctx: &mut SceneContext, dt: f32) -> SceneTransition;

    /// 描画コマンドの発行
    fn render(&self, ctx: &RenderContext);

    /// シーン終了時のクリーンアップ
    fn on_exit(&mut self, ctx: &mut SceneContext);

    /// このシーンで必要なアセット一覧
    fn required_assets(&self) -> Vec<AssetDescriptor>;
}
```

### シーン遷移フロー
1. 現シーン `on_exit()` → リソース解放
2. アセット完全パージ（リーク防止）
3. 新シーン `required_assets()` → 非同期ロード
4. ロード完了後 `on_enter()` → 初期化

## プラグインパターン（ミニゲーム）

- ミニゲームは独立した `Scene` トレイト実装として作成
- ADV パートとの通信はメッセージパッシング（`MiniGameResult` enum）
- ミニゲーム固有の ECS Component / System は専用モジュールにスコープ
- ミニゲーム終了時に ECS をクリーンアップし、ADV パートに制御を返す

## Z-Order ルール

描画レイヤーの深度値を統一する。

| レイヤー | Z 値 | 説明 |
|----------|-------|------|
| Background | `10.0` | 背景画像、HDR 環境マップ |
| 3D/Live2D | `0.0` 〜 `5.0` | キャラクター・オブジェクト（描画順で制御） |
| Characters | `-1.0` 〜 `-5.0` | 前景キャラクター |
| UI/Text | `-10.0` | 最前面 UI（Orthoカメラ、深度テスト無効） |

## マルチビュー・レンダリング

- **Game View (Layer 0)**: 3D空間。PBR + IBL + Live2D (Render-to-Texture) + 2D Sprite (Quad)
- **UI View (Layer 1)**: Ortho カメラ。深度テスト無効。UI/テキスト描画専用

## Dual RNG

| 用途 | アルゴリズム | 性質 |
|------|-------------|------|
| 演出 (VisualRng) | Xoshiro256++ | 非決定論的、高速 |
| ロジック (LogicRng) | ChaCha8 | 決定論的、リプレイ再現可能 |

- シーン遷移時に LogicRng のシードを「バケツリレー」方式で引き継ぐ
- セーブデータに LogicRng のシード状態を含める
