---
name: live2d_integration
description: Live2D Cubism SDK 統合スキル（FFI、Render-to-Texture、リップシンク、モーション）
---

# Live2D 統合スキル

## 概要

Live2D Cubism Native SDK (C++) を Rust に FFI 経由で統合する。Filament の Render-to-Texture 方式でゲームシーンに合成し、音声 RMS 解析による自動リップシンク、`.motion3.json` による表情・モーション制御を実現する。

## アーキテクチャ

```text
Live2D Cubism SDK (C++)
    ↓ FFI (unsafe ラッパー)
engine_core::ffi::live2d_sys  ← 低レベルバインディング
    ↓ 安全なラッパー
engine_core::live2d           ← 公開 API
    ↓
Filament Render-to-Texture    ← ゲームシーンに合成
```

## FFI バインディング

### `live2d_sys` モジュール

```rust
// crates/engine_core/src/ffi/live2d_sys.rs

#[link(name = "Live2DCubismCore")]
extern "C" {
    // SAFETY: SDK の初期化関数。アプリケーション開始時に1度だけ呼び出す。
    pub fn csmReviveMocInPlace(address: *mut u8, size: u32) -> *mut CsmMoc;
    pub fn csmGetSizeofModel(moc: *const CsmMoc) -> u32;
    pub fn csmInitializeModelInPlace(moc: *const CsmMoc, address: *mut u8, size: u32) -> *mut CsmModel;
    // ... その他の SDK 関数
}
```

### 安全なラッパー

```rust
// crates/engine_core/src/live2d/model.rs

pub struct Live2DModel {
    moc: NonNull<CsmMoc>,
    model: NonNull<CsmModel>,
    // ...
}

impl Live2DModel {
    pub fn load(data: &[u8]) -> Result<Self, Live2DError> {
        // SAFETY: データサイズを検証済み。メモリアラインメント確保済み。
        unsafe {
            // MOC の復元とモデル初期化
        }
    }

    pub fn set_parameter(&mut self, id: &str, value: f32) {
        // パラメータ更新（口、目、体等）
    }

    pub fn update(&mut self, dt: f32) {
        // 物理演算・モーション更新
    }
}

impl Drop for Live2DModel {
    fn drop(&mut self) {
        // SAFETY: モデルのライフタイムはこの構造体が管理。
        unsafe { /* メモリ解放 */ }
    }
}
```

## Render-to-Texture 統合

1. Live2D のレンダリング結果を専用テクスチャに書き込み
2. Filament の 3D 空間内の Quad にテクスチャを貼り付け
3. IBL / ブルーム等のポストプロセスの恩恵を受ける

```rust
pub struct Live2DRenderer {
    render_target: filament::RenderTarget,
    quad_entity: Entity,  // Filament 空間内の Quad
}
```

## 自動リップシンク

音声データの RMS（実効値）をリアルタイム解析し、Live2D の口パクパラメータに反映。

```rust
pub fn calculate_lip_sync(audio_samples: &[f32], frame_size: usize) -> f32 {
    let rms = (audio_samples.iter()
        .take(frame_size)
        .map(|s| s * s)
        .sum::<f32>() / frame_size as f32)
        .sqrt();
    // RMS を 0.0〜1.0 にマッピング
    (rms * 10.0).clamp(0.0, 1.0)
}
```

## モーション・表情制御

- `.motion3.json` の再生: フェードイン/アウト付きモーション遷移
- 表情ブレンド: 複数の表情を重み付きでブレンド
- スクリプトタグから制御:

```text
[chara id="reimu" motion="happy" expression="smile"]
[chara id="reimu" lip_sync="on"]
```

## ECS 連携

```rust
// Live2D 用 Component
pub struct Live2DComponent {
    pub model: Live2DModel,
    pub current_motion: Option<MotionId>,
    pub lip_sync_enabled: bool,
}

// Live2D 更新 System
pub fn live2d_update_system(/* ECS クエリ */) {
    // 物理演算、モーション更新、リップシンク反映
}
```

## 注意事項

- Live2D Cubism SDK は商用ライセンスが必要
- `unsafe` コードは `ffi/live2d_sys.rs` に隔離
- すべての `unsafe` ブロックに `// SAFETY:` コメント必須
