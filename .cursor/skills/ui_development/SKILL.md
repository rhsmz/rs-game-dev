---
name: ui_development
description: MVVM + Signal 駆動 UI 開発スキル（Taffy レイアウト、SDF テキスト、Glassmorphism）
---

# UI 開発スキル
## 概要
本エンジンの UI は MVVM アーキテクチャ + Signal 駆動リアクティブシステムで構築する。レイアウトエンジンに Taffy (Flexbox)、テキスト描画に SDF、マテリアルに Glassmorphism を採用する。

## MVVM レイヤー
| レイヤー | 責務 | 実装概要 |
|----------|------|----------|
| View | レイアウト計算・描画 | Taffy + Filament Ortho |
| ViewModel | UI 状態管理 | `Signal<T>` リアクティブ |
| Model | ECS ゲームデータ | Component / Resource |

## Signal 駆動リアクティブシステム
Floem の思想を継承。値の変更があった UI コンポーネントのみを再計算・再描画する。
```rust
// Signal 定義
pub struct Signal<T> {
    value: T,
    subscribers: Vec<Callback>,
}

impl<T: PartialEq> Signal<T> {
    pub fn get(&self) -> &T {
        &self.value
    }

    pub fn set(&mut self, new_value: T) {
        if self.value != new_value {
            self.value = new_value;
            // 購読者に変更通知
            for cb in &self.subscribers {
                cb.notify();
            }
        }
    }

    pub fn subscribe(&mut self, callback: Callback) {
        self.subscribers.push(callback);
    }
}
```

## Taffy レイアウト
Flexbox アルゴリズムでUIノードの座標・サイズを計算。
```rust
use taffy::prelude::*;

pub fn build_message_box(taffy: &mut TaffyTree) -> NodeId {
    let text_node = taffy.new_leaf(Style {
        size: Size { width: percent(1.0), height: auto() },
        padding: Rect::all(length(16.0)),
        ..Default::default()
    }).unwrap();

    let container = taffy.new_with_children(
        Style {
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            size: Size { width: percent(0.8), height: auto() },
            position: Position::Absolute,
            inset: Rect {
                bottom: length(32.0),
                left: percent(0.1),
                ..Rect::auto()
            },
            ..Default::default()
        },
        &[text_node],
    ).unwrap();

    container
}
```

## SDF テキスト描画
Signed Distance Field 方式で劣化のないテキスト描画を実現。
### フロー
1. フォントからグリフの SDF テクスチャアトラスを事前生成
2. Filament のカスタムマテリアルで SDF サンプリング
3. 任意のスケールでシャープなテキスト描画
```rust
pub struct SdfTextRenderer {
    atlas: SdfAtlas,
    material: FilamentMaterial,
}

impl SdfTextRenderer {
    pub fn render_text(&self, text: &str, position: Vec2, font_size: f32, color: Color) {
        // グリフごとに Quad を生成、SDF マテリアルで描画
    }
}
```

## Glassmorphism マテリアル
背景ぼかし + 半透明 + 薄いボーダーの Glassmorphism エフェクト。
```rust
pub struct GlassMaterial {
    pub blur_radius: f32,      // 背景ぼかし強度
    pub opacity: f32,          // 不透明度 (0.0〜1.0)
    pub border_radius: f32,    // 角丸半径
    pub border_width: f32,     // ボーダー幅
    pub tint_color: Color,     // 着色
}
```

### 実装方式
1. Game View のレンダリング結果をテクスチャとしてキャプチャ
2. ガウシアンブラーを適用した背景テクスチャを生成
3. UI パネルの背景として使用

## 基本 UI コンポーネント
### メッセージウィンドウ
```rust
pub struct MessageWindow {
    pub text: Signal<String>,
    pub speaker: Signal<Option<String>>,
    pub is_visible: Signal<bool>,
    style: GlassMaterial,
    layout: NodeId,
}
```

### 選択肢パネル
```rust
pub struct ChoicePanel {
    pub choices: Signal<Vec<ChoiceItem>>,
    pub selected_index: Signal<Option<usize>>,
    style: GlassMaterial,
}
```

### ステータス表示
```rust
pub struct StatusBar {
    pub hp: Signal<f32>,
    pub mp: Signal<f32>,
    pub name: Signal<String>,
}
```

## ECS ↔ UI 通信
### UI → ECS (コマンドキュー)
```rust
pub enum UiCommand {
    ChoiceSelected(usize),
    MenuAction(MenuAction),
    SliderChanged { id: String, value: f32 },
}
```

### ECS → UI (Signal 更新)
```rust
// UI 同期 System
pub fn ui_sync_system(/* ECS クエリ */) {
    // Component 変更検知 → Signal 更新
    // 例: HP 変更を検知 → status_bar.hp.set(new_hp)
}
```

## 描画レイヤー
UI は `UI View (Layer 1)` に描画。Ortho カメラ、深度テスト無効、Z = -10.0。

