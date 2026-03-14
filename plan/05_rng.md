# 05: Dual RNG — 乱数エンジン

## 概要
演出用 (Xoshiro256++) とロジック用 (ChaCha8) の 2 つの乱数生成器を管理する。ロジック用 RNG はリプレイ再現性を保証する。

## 依存先
- `01_ecs_foundation`

## モジュール構成
```
crates/engine_core/src/rng/
├── mod.rs          # 公開 API
├── visual.rs       # VisualRng (Xoshiro256++)
└── logic.rs        # LogicRng (ChaCha8)
```

## 外部依存
```toml
[dependencies]
rand = "0.9"
rand_xoshiro = "0.7"
rand_chacha = "0.9"
```

## タスク

### 1. VisualRng — 演出用乱数
```rust
use rand_xoshiro::Xoshiro256PlusPlus;

pub struct VisualRng {
    rng: Xoshiro256PlusPlus,
}

impl VisualRng {
    pub fn new() -> Self; // 非決定的シード
    pub fn gen_range(&mut self, range: std::ops::Range<f32>) -> f32;
    pub fn gen_bool(&mut self, probability: f64) -> bool;
}
```

- パーティクル、画面揺れ、エフェクトタイミング等の演出に使用
- 再現性不要、高速性重視

### 2. LogicRng — ロジック用乱数
```rust
use rand_chacha::ChaCha8Rng;

pub struct LogicRng {
    rng: ChaCha8Rng,
    seed: [u8; 32],
}

impl LogicRng {
    pub fn from_seed(seed: [u8; 32]) -> Self;
    pub fn gen_range(&mut self, range: std::ops::Range<i32>) -> i32;
    pub fn get_seed(&self) -> [u8; 32];
    /// シーン遷移時のシード引き継ぎ
    pub fn derive_child_seed(&mut self) -> [u8; 32];
}
```

- ミニゲーム判定、ダメージ計算、ドロップ抽選等に使用
- 同一シードで完全再現可能
- セーブデータにシード状態を保存

### 3. シード・バケツリレー
- シーン遷移時に `derive_child_seed()` で子シードを生成
- 新シーンの LogicRng に引き継ぎ
- リプレイ再現性の保証

### 4. ECS リソースとして登録
```rust
// World のリソースとして登録
world.insert_resource(VisualRng::new());
world.insert_resource(LogicRng::from_seed(initial_seed));
```

## テスト計画
- 同一シード → 同一出力列の再現性テスト
- `derive_child_seed` のチェーンテスト
- VisualRng の統計的一様分布テスト

## 完了条件
- VisualRng / LogicRng の生成・使用が可能
- LogicRng のシード保存・復元でリプレイ再現可能
