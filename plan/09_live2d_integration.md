# 09: Live2D Integration — Live2D 統合

## 概要
Live2D Cubism Native SDK (C++) を Rust FFI 経由で統合。Render-to-Texture で Filament に合成し、音声 RMS リップシンク、モーション/表情制御を実装する。

## 依存先
- `02_renderer`

## モジュール構成
```
crates/engine_core/src/live2d/
├── mod.rs              # 公開 API
├── model.rs            # Live2DModel (安全ラッパー)
├── renderer.rs         # Live2DRenderer (Render-to-Texture)
├── motion.rs           # モーション再生管理
├── expression.rs       # 表情ブレンド
├── physics.rs          # 物理演算 (揺れ物)
└── error.rs            # Live2DError

crates/engine_core/src/ffi/
└── live2d_sys.rs       # Cubism SDK C バインディング
```

## タスク

### 1. FFI バインディング (`live2d_sys`)
- Cubism Native SDK の C API バインディング
- `build.rs` でライブラリリンク設定
- 全 `unsafe` に `// SAFETY:` コメント

### 2. Live2DModel (安全ラッパー)
```rust
pub struct Live2DModel {
    moc: NonNull<ffi::CsmMoc>,
    model: NonNull<ffi::CsmModel>,
    parameters: Vec<ParameterInfo>,
    parts: Vec<PartInfo>,
}

impl Live2DModel {
    pub fn load(data: &[u8]) -> Result<Self, Live2DError>;
    pub fn set_parameter(&mut self, id: &str, value: f32);
    pub fn get_parameter(&self, id: &str) -> Option<f32>;
    pub fn update(&mut self, dt: f32);
}
```

### 3. Render-to-Texture 統合
- Live2D の描画結果を Filament テクスチャに書き込み
- 3D 空間内の Quad にマッピング → IBL/ブルーム適用
- Z-Order: Characters (-1.0〜-5.0)

### 4. モーション再生
- `.motion3.json` パーサ
- フェードイン/アウト付きモーション遷移
- モーションキュー管理

### 5. 表情ブレンド
- 複数表情の重み付きブレンド
- 表情 + モーションの並行適用

### 6. 物理演算 (揺れ物)
- Cubism SDK の物理演算パラメータ設定
- 髪・衣装の揺れシミュレーション

### 7. ECS 連携
```rust
pub struct Live2DComponent {
    pub model: Live2DModel,
    pub current_motion: Option<MotionId>,
    pub lip_sync_enabled: bool,
    pub lip_sync_value: f32,
}
```

### 8. スクリプト連携
```text
[chara id="reimu" motion="happy" expression="smile"]
[chara id="reimu" lip_sync="on"]
```

## テスト計画
- MOC ファイルのロード / パラメータ設定テスト
- モーション再生 / ブレンドテスト
- RMS → リップシンク値変換テスト

## 注意事項
- Live2D Cubism SDK は商用ライセンスが必要
- SDK のヘッダーファイルは `.gitignore` で除外

## 完了条件
- Live2D モデルの Filament 画面上での描画
- モーション / 表情の切替
- リップシンクのリアルタイム動作
