---
name: scene_creation
description: Scene トレイト実装およびシーン遷移の作成スキル
---

# Scene 作成スキル
## 概要
本エンジンの全ゲームパート（ADVパート、ミニゲーム、タイトル画面等）は `Scene` トレイトを実装する。このスキルでは新しいシーンの作成手順とベストプラクティスを定義する。

## Scene トレイト定義
```rust
use crate::asset::AssetDescriptor;
use crate::scene::{SceneContext, RenderContext, SceneTransition};

pub trait Scene: Send + Sync {
    /// シーンの初期化（アセットロード完了後に呼ばれる）
    fn on_enter(&mut self, ctx: &mut SceneContext);

    /// 毎フレーム更新。次のシーン遷移を返す。
    fn update(&mut self, ctx: &mut SceneContext, dt: f32) -> SceneTransition;

    /// 描画コマンドの発行
    fn render(&self, ctx: &RenderContext);

    /// シーン終了時のクリーンアップ
    fn on_exit(&mut self, ctx: &mut SceneContext);

    /// このシーンで必要なアセット一覧（事前ロード用）
    fn required_assets(&self) -> Vec<AssetDescriptor>;
}
```

## SceneTransition 列挙型
```rust
pub enum SceneTransition {
    /// 現シーンを継続
    None,
    /// 指定シーンに遷移
    Push(Box<dyn Scene>),
    /// 現シーンを終了して前シーンに戻る
    Pop,
    /// 現シーンを破棄して指定シーンに置換
    Replace(Box<dyn Scene>),
    /// ゲーム終了
    Quit,
}
```

## 新シーン作成手順

### 1. ファイル作成

`crates/engine_core/src/scene/` にシーンモジュールを作成。

```rust
// crates/engine_core/src/scene/title_scene.rs

pub struct TitleScene {
    // シーン固有のステート
}

impl TitleScene {
    pub fn new() -> Self {
        Self { }
    }
}

impl Scene for TitleScene {
    fn on_enter(&mut self, ctx: &mut SceneContext) {
        // BGM再生、UIセットアップ等
    }

    fn update(&mut self, ctx: &mut SceneContext, dt: f32) -> SceneTransition {
        // 入力チェック → シーン遷移判定
        SceneTransition::None
    }

    fn render(&self, ctx: &RenderContext) {
        // 描画コマンド発行
    }

    fn on_exit(&mut self, ctx: &mut SceneContext) {
        // リソース解放
    }

    fn required_assets(&self) -> Vec<AssetDescriptor> {
        vec![
            // 必要アセットを列挙
        ]
    }
}
```

### 2. モジュール登録
`crates/engine_core/src/scene/mod.rs` に追加。

### 3. テスト
- `on_enter` → `update` → `on_exit` のライフサイクルが正しく動作することを確認
- `SceneTransition` の各バリアントが正しく処理されることを確認

## シーン遷移フロー
```
現シーン.on_exit()
    ↓
アセット完全パージ
    ↓
新シーン.required_assets() → 非同期ロード
    ↓
ロード完了後、新シーン.on_enter()
    ↓
メインループで新シーン.update() / render()
```

## ベストプラクティス
- シーン固有の ECS Component/System はシーンの `on_enter` で登録、`on_exit` で削除
- 重いアセットは `required_assets()` で事前宣言し、ロード画面で非同期ロード
- シーン間のデータ受け渡しは `SceneContext` 経由のメッセージパッシングで行う

