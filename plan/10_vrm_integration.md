# 10: VRM/FBX Integration — 3D モデル統合

## 概要
VRM (.vrm) / FBX (.fbx) 形式の 3D モデルを Filament レンダラーに統合。SpringBone 揺れ物シミュレーション、BlendShape 表情制御、エディタでの GLB 事前変換をサポートする。

## 依存先
- `02_renderer`

## モジュール構成
```
crates/engine_core/src/vrm/
├── mod.rs              # 公開 API
├── loader.rs           # VRM/GLB ローダー
├── spring_bone.rs      # SpringBone 物理演算
├── blend_shape.rs      # BlendShape → MorphTarget
├── skeleton.rs         # スケルトン / ボーン階層
├── animation.rs        # アニメーション再生
└── error.rs            # VrmError
```

## 外部依存
```toml
[dependencies]
gltf = "1"  # glTF 2.0 パーサ (VRM は glTF 拡張)
```

## タスク

### 1. VRM/GLB ローダー
```rust
pub struct VrmModel {
    pub meshes: Vec<FilamentMesh>,
    pub skeleton: Skeleton,
    pub spring_bones: Vec<SpringBoneChain>,
    pub blend_shapes: BlendShapeProxy,
    pub animations: Vec<AnimationClip>,
}

impl VrmModel {
    pub fn load(data: &[u8]) -> Result<Self, VrmError>;
}
```

- glTF 2.0 パース → VRM 拡張データ抽出
- メッシュ → Filament VertexBuffer / IndexBuffer 変換
- マテリアル → Filament Material 変換

### 2. SpringBone
```rust
pub struct SpringBoneChain {
    pub stiffness: f32,
    pub gravity_power: f32,
    pub drag_force: f32,
    pub hit_radius: f32,
    nodes: Vec<SpringBoneNode>,
    colliders: Vec<Collider>,
}

impl SpringBoneChain {
    pub fn update(&mut self, dt: f32, center_transform: &Transform);
}
```

- バーレ法による物理シミュレーション
- コライダー衝突判定
- LOD による更新頻度調整（負荷対策）

### 3. BlendShape (MorphTarget)
```rust
pub struct BlendShapeProxy {
    groups: HashMap<BlendShapePreset, Vec<MorphBinding>>,
}

pub enum BlendShapePreset {
    Joy, Angry, Sorrow, Fun,
    Blink, BlinkL, BlinkR,
    A, I, U, E, O,  // リップシンク用
}

impl BlendShapeProxy {
    pub fn set_weight(&mut self, preset: BlendShapePreset, weight: f32);
    pub fn apply(&self, mesh: &mut FilamentMesh);
}
```

### 4. アニメーション再生
```rust
pub struct AnimationPlayer {
    clips: Vec<AnimationClip>,
    current: Option<usize>,
    time: f32,
    blend_factor: f32,
}

impl AnimationPlayer {
    pub fn play(&mut self, clip_index: usize, blend_duration: f32);
    pub fn update(&mut self, dt: f32, skeleton: &mut Skeleton);
}
```

### 5. FBX → GLB 変換 (Editor 機能)
- `script_editor` クレートでインポート → GLB エクスポート
- ランタイム (`engine_core`) では GLB のみロード

### 6. ECS 連携
```rust
pub struct Model3DComponent {
    pub model: VrmModel,
    pub animation_player: AnimationPlayer,
}

pub fn spring_bone_update_system(world: &mut World, dt: f32);
pub fn animation_update_system(world: &mut World, dt: f32);
pub fn blend_shape_update_system(world: &mut World);
```

## テスト計画
- GLB ファイルのロード / メッシュ抽出テスト
- SpringBone シミュレーション精度テスト
- BlendShape ウェイト適用テスト
- アニメーション再生 / ブレンドテスト

## 完了条件
- VRM モデルの Filament 画面上での描画
- SpringBone による髪・衣装の揺れ
- BlendShape による表情変更
- アニメーション再生とブレンド
