---
name: minigame_plugin
description: ミニゲーム（2D/3Dアクション、RPG、パズル、RTS）のプラグイン実装スキル
---

# ミニゲーム・プラグイン実装スキル
## 概要
本エンジンは恋姫†無双、TWINKLE CRUSADERS 等を参考に、ADVパート中にミニゲームを挿入する設計。ミニゲームは `Scene` トレイトを実装した独立プラグインとして作成する。

## 対応ミニゲームタイプ
| タイプ | 説明 | 例 |
|--------|------|-----|
| 2D アクション | 横スクロール、シューティング等 | 戦闘パート |
| 3D アクション | 3D空間での戦闘・探索 | ボス戦 |
| RPG | ターン制/リアルタイム戦闘 | 合戦パート |
| パズル | ロジック、マッチング等 | クイズ・謎解き |
| RTS | リアルタイムストラテジー | 軍団指揮 |

## アーキテクチャ
```text
ADV Scene
  ├── スクリプト実行 → ミニゲーム開始コマンド検出
  ├── MiniGame Scene を Push
  │     ├── 独自 ECS Component/System
  │     ├── 独自アセット
  │     └── 終了時に MiniGameResult を返す
  └── ADV Scene に Pop → 結果を受け取りスクリプト続行
```

## 実装手順
### 1. ミニゲームモジュール作成
```rust
// crates/engine_core/src/scene/minigame/battle_scene.rs

use crate::scene::{Scene, SceneContext, RenderContext, SceneTransition};

pub struct BattleScene {
    config: BattleConfig,
    state: BattleState,
}

/// ミニゲームの結果
pub enum MiniGameResult {
    Victory,
    Defeat,
    Draw,
    Score(u32),
}
```

### 2. Scene トレイト実装
```rust
impl Scene for BattleScene {
    fn on_enter(&mut self, ctx: &mut SceneContext) {
        // ミニゲーム固有の ECS Component/System を登録
        // 専用 BGM 再生
        // LogicRng (ChaCha8) の状態をセットアップ
    }

    fn update(&mut self, ctx: &mut SceneContext, dt: f32) -> SceneTransition {
        // ゲームロジック更新
        // 終了条件チェック
        if self.state.is_finished() {
            // 結果を SceneContext に書き込み
            ctx.set_minigame_result(self.state.result());
            return SceneTransition::Pop;
        }
        SceneTransition::None
    }

    fn on_exit(&mut self, ctx: &mut SceneContext) {
        // ミニゲーム固有の ECS リソースをクリーンアップ
    }

    fn required_assets(&self) -> Vec<AssetDescriptor> {
        // ミニゲーム固有アセット
        vec![]
    }

    fn render(&self, ctx: &RenderContext) {
        // ミニゲーム固有の描画
    }
}
```

### 3. ADV パートとの連携
スクリプトタグでミニゲームを起動:
```text
[minigame type="battle" config="battle_01"]
[if result=="victory"]
  [text]勝利！[/text]
[else]
  [text]敗北...[/text]
[/if]
```

### 4. RNG 管理
- ミニゲームのロジック判定には `LogicRng` (ChaCha8) を使用
- シーン突入時に LogicRng のシードをセーブデータに含める（リプレイ再現性）
- 演出（パーティクル等）には `VisualRng` (Xoshiro256++) を使用

## ディレクトリ構造
```text
crates/engine_core/src/scene/minigame/
├── mod.rs              # ミニゲーム共通インターフェース
├── battle_scene.rs     # 2D/3D アクション戦闘
├── rpg_battle.rs       # RPG 戦闘
├── puzzle_scene.rs     # パズル
└── rts_scene.rs        # RTS
```

## テスト方針
- 各ミニゲームの勝利/敗北/引き分け条件を網羅テスト
- LogicRng のシード固定でリプレイ再現性をテスト
- ADV パートへの結果受け渡しを統合テストで検証

