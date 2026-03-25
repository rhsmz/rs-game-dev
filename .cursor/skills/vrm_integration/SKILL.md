---
name: vrm_integration
description: VRM/FBX 3Dモデル統合スキル（SpringBone、BlendShape、GLB変換）
---

# VRM/FBX 統合スキル
## 概要
VRM（バーチャルキャスト標準）と FBX フォーマットの3Dモデルを Filament レンダラーに統合する。SpringBone による揺れ物シミュレーション、BlendShape による表情制御、エディタでの GLB 事前変換をサポートする。

## 対応フォーマット
| フォーマット | 用途 | 処理 |
|-------------|------|------|
| VRM (.vrm) | バーチャルキャラクター | 直接ロード、SpringBone 対応 |
| FBX (.fbx) | 汎用3Dモデル | エディタで GLB に事前変換 |
| GLB (.glb) | ランタイムフォーマット | 高速ロード |

## VRM ローダー
```rust
// crates/engine_core/src/vrm/loader.rs

pub struct VrmModel {
    pub mesh: FilamentMesh,
    pub skeleton: Skeleton,
    pub spring_bones: Vec<SpringBone>,
    pub blend_shapes: BlendShapeProxy,
    pub animations: Vec<Animation>,
}

impl VrmModel {
    pub fn load(data: &[u8]) -> Result<Self, VrmError> {
        // glTF 2.0 パース → VRM 拡張データ抽出
        // メッシュ → Filament VertexBuffer / IndexBuffer 変換
        // スケルトン構築
        // SpringBone パラメータ抽出
        // BlendShape 定義読み込み
        Ok(Self { /* ... */ })
    }
}
```

## SpringBone シミュレーション
VRM の揺れ物（髪、衣装、アクセサリ）をフレームごとにシミュレーション。
```rust
// crates/engine_core/src/vrm/spring_bone.rs

pub struct SpringBone {
    pub stiffness: f32,     // 剛性
    pub gravity_power: f32, // 重力
    pub drag_force: f32,    // 抵抗力
    pub hit_radius: f32,    // 当たり判定半径
    bones: Vec<SpringBoneNode>,
}

impl SpringBone {
    pub fn update(&mut self, dt: f32, center: &Transform) {
        for bone in &mut self.bones {
            // バーレ法による物理演算
            // コライダー衝突判定
            // ボーン位置更新
        }
    }
}
```

## BlendShape (MorphTarget)
VRM の `BlendShapeProxy` を Filament の MorphTarget にマッピング。
```rust
pub struct BlendShapeProxy {
    presets: HashMap<BlendShapePreset, Vec<BlendShapeBinding>>,
}

#[derive(Hash, Eq, PartialEq)]
pub enum BlendShapePreset {
    Joy,
    Angry,
    Sorrow,
    Fun,
    Blink,
    BlinkL,
    BlinkR,
    // VRM 標準プリセット
}

impl BlendShapeProxy {
    pub fn set_value(&mut self, preset: BlendShapePreset, weight: f32) {
        // MorphTarget ウェイト更新
    }
}
```

## FBX → GLB 変換
エディタ (`script_editor`) で FBX を GLB に事前変換し、ランタイムでは GLB のみをロード。
```text
[script_editor]
  FBX インポート → アニメーション保持 → GLB エクスポート
      ↓
[game_player / engine_core]
  GLB 高速ロード
```

## ECS 連携
```rust
// 3D モデル用 Component
pub struct Model3DComponent {
    pub model: VrmModel,
    pub current_animation: Option<AnimationId>,
    pub blend_shape_state: HashMap<BlendShapePreset, f32>,
}

// SpringBone 更新 System
pub fn spring_bone_update_system(/* ECS クエリ */) {
    // 全 VRM の SpringBone を物理更新
}

// BlendShape 更新 System
pub fn blend_shape_update_system(/* ECS クエリ */) {
    // 表情パラメータを MorphTarget に反映
}
```

## スクリプト連携
```text
[chara3d id="warrior" animation="idle"]
[chara3d id="warrior" expression="joy" weight="0.8"]
[chara3d id="warrior" animation="attack" blend="0.3"]
```

## 注意事項
- VRM 1.0 仕様に準拠
- SpringBone の負荷が高い場合、LOD で更新頻度を調整
- FBX→GLB 変換は `script_editor` の機能として提供（ランタイムでは FBX を扱わない）

