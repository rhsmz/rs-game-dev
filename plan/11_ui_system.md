# 11: UI System — MVVM + Signal 駆動 UI

## 概要
MVVM アーキテクチャ + Signal 駆動リアクティブ UI を構築。Taffy (Flexbox) レイアウト、SDF テキスト、Glassmorphism マテリアルを Filament の UI View (Layer 1) に描画する。

## 依存先
- `02_renderer`

## モジュール構成
```
crates/engine_core/src/ui/
├── mod.rs              # 公開 API
├── signal.rs           # Signal<T> リアクティブシステム
├── view_model.rs       # ViewModel 基底
├── layout.rs           # Taffy レイアウトエンジン連携
├── node.rs             # UINode (View 層)
├── text.rs             # SDF テキストレンダラー
├── material.rs         # Glassmorphism マテリアル
├── widgets/
│   ├── mod.rs
│   ├── message_window.rs   # メッセージウィンドウ
│   ├── choice_panel.rs     # 選択肢パネル
│   ├── status_bar.rs       # ステータスバー
│   ├── button.rs           # ボタン
│   └── slider.rs           # スライダー
└── command.rs          # UI → ECS コマンド
```

## 外部依存
```toml
[dependencies]
taffy = "0.7"
```

## タスク

### 1. Signal<T> リアクティブシステム
```rust
pub struct Signal<T> {
    value: T,
    version: u64,
    subscribers: Vec<SubscriberId>,
}

impl<T: PartialEq + Clone> Signal<T> {
    pub fn new(value: T) -> Self;
    pub fn get(&self) -> &T;
    pub fn set(&mut self, value: T);  // 変更時のみ通知
    pub fn subscribe(&mut self, id: SubscriberId);
}
```

- ファイングレイン変更検知
- 変更があったノードのみ再計算

### 2. Taffy レイアウト連携
```rust
pub struct UiLayoutEngine {
    tree: TaffyTree<()>,
    root: NodeId,
}

impl UiLayoutEngine {
    pub fn new() -> Self;
    pub fn add_node(&mut self, style: Style, parent: NodeId) -> NodeId;
    pub fn compute(&mut self, viewport: Size<f32>);
    pub fn get_layout(&self, node: NodeId) -> &Layout;
}
```

### 3. SDF テキストレンダラー
- フォント → SDF テクスチャアトラス生成
- Filament カスタムマテリアルで SDF サンプリング
- 任意スケールでシャープ描画
- 文字送りアニメーション（ADV テキスト用）

### 4. Glassmorphism マテリアル
```rust
pub struct GlassStyle {
    pub blur_radius: f32,
    pub opacity: f32,
    pub border_radius: f32,
    pub border_width: f32,
    pub tint_color: [f32; 4],
}
```

- Game View のフレームバッファキャプチャ → ガウシアンブラー
- 半透明 + ぼかし背景 + 薄ボーダー

### 5. ADV 用 Widget
- **MessageWindow**: テキスト表示、話者名、文字送り、Glassmorphism 背景
- **ChoicePanel**: 選択肢リスト、ホバー/選択状態
- **StatusBar**: HP/MP ゲージ、キャラ名

### 6. ECS ↔ UI 通信
```rust
// UI → ECS
pub enum UiCommand {
    ChoiceSelected(usize),
    MenuAction(MenuAction),
    SliderChanged { id: String, value: f32 },
}

// ECS → UI (Signal 更新)
pub fn ui_sync_system(world: &World, view_model: &mut UiViewModel);
```

## テスト計画
- Signal の変更検知 / 通知テスト
- Taffy レイアウト計算の正確性テスト
- Widget 状態遷移テスト

## 完了条件
- メッセージウィンドウに文字送り表示
- 選択肢パネルの選択操作
- Glassmorphism 背景描画
- ECS 状態変更 → UI 自動更新
