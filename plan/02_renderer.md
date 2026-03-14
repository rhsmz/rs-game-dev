# 02: Renderer — Filament レンダラー統合

## 概要
Google Filament PBR レンダラーを Rust FFI 経由で統合し、マルチビュー・レンダリング・スタック（Game View + UI View）を構築する。

## 依存先
- `01_ecs_foundation`

## モジュール構成
```
crates/engine_core/src/renderer/
├── mod.rs              # 公開 API
├── engine.rs           # Filament Engine ラッパー
├── view.rs             # View / Camera 管理
├── material.rs         # マテリアル管理
├── mesh.rs             # メッシュ / VertexBuffer
├── texture.rs          # テクスチャ管理
├── light.rs            # ライティング (IBL, DirectionalLight)
├── sprite.rs           # 2D スプライト (3D Quad)
├── post_process.rs     # ポストプロセス (Bloom, Tone Mapping)
└── render_target.rs    # Render-to-Texture (Live2D 合成用)
```

FFI バインディング:
```
crates/engine_core/src/ffi/
└── filament_sys.rs     # Filament C API バインディング
```

## タスク

### 1. Filament FFI バインディング (`filament_sys`)
- `bindgen` または手動で Filament C API のバインディング生成
- 主要関数: `Engine_create`, `Engine_destroy`, `Scene_create`, `View_create`, `Renderer_create`
- `build.rs` でライブラリリンク設定

### 2. Engine ラッパー
```rust
pub struct RenderEngine {
    engine: NonNull<ffi::Engine>,
    renderer: NonNull<ffi::Renderer>,
    swap_chain: NonNull<ffi::SwapChain>,
}

impl RenderEngine {
    pub fn new(window_handle: RawWindowHandle) -> Result<Self, RendererError>;
    pub fn begin_frame(&mut self) -> bool;
    pub fn end_frame(&mut self);
    pub fn resize(&mut self, width: u32, height: u32);
}

impl Drop for RenderEngine {
    fn drop(&mut self) { /* Filament リソース解放 */ }
}
```

### 3. マルチビュー構成
```rust
/// Game View (Layer 0): 3D + Live2D + 2D Sprite
pub struct GameView {
    view: NonNull<ffi::View>,
    camera: Camera,
    scene: NonNull<ffi::Scene>,
}

/// UI View (Layer 1): Ortho カメラ、深度テスト無効
pub struct UiView {
    view: NonNull<ffi::View>,
    camera: Camera, // Ortho projection
    scene: NonNull<ffi::Scene>,
}
```

- Game View: Perspective カメラ、PBR + IBL
- UI View: Ortho カメラ、深度テスト無効、Z = -10.0

### 4. IBL / ライティング
- HDR 環境マップのロードと IBL 設定
- DirectionalLight (太陽光) 設定
- SkyBox 管理

### 5. 2D スプライト描画
- 3D 空間内の Quad にテクスチャを貼り付け
- IBL ライティングの恩恵を受ける 2D 表現
- Z-Order: Background (10.0) / Characters (-1.0〜-5.0)

### 6. Render-to-Texture
- Live2D 合成用のオフスクリーンレンダーターゲット
- テクスチャ → 3D Quad にマッピング

### 7. ポストプロセス
- Bloom エフェクト
- Tone Mapping
- ガウシアンブラー（UI Glassmorphism 用）

### 8. ECS 連携
```rust
// Renderer 用 Component
pub struct MeshRenderer {
    pub renderable: NonNull<ffi::RenderableManager>,
    pub material: MaterialHandle,
}

pub struct Camera3D {
    pub fov: f32,
    pub near: f32,
    pub far: f32,
}

// Renderer System
pub fn render_system(world: &World, engine: &mut RenderEngine);
```

## テスト計画
- ウィンドウ作成 + Filament 初期化テスト
- マルチビュー描画テスト
- テクスチャロード / 表示テスト

## 完了条件
- ウィンドウにカラークリア + 基本メッシュ描画ができる
- Game View + UI View の 2 ビュー同時レンダリング
- IBL ライティング適用
