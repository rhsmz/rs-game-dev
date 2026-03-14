# 12: Save System — セーブ/ロード

## 概要
MessagePack (`rmp-serde`) ベースのセーブ/ロードシステム。スロット管理、オートセーブ、バージョンマイグレーション、サムネイルキャプチャを実装する。

## 依存先
- `01_ecs_foundation`

## モジュール構成
```
crates/engine_core/src/save/
├── mod.rs          # 公開 API
├── data.rs         # SaveFile 構造体
├── serializer.rs   # MessagePack シリアライズ/デシリアライズ
├── manager.rs      # SaveManager (スロット管理)
├── migration.rs    # バージョンマイグレーション
├── thumbnail.rs    # サムネイルキャプチャ
└── error.rs        # SaveError
```

## タスク

### 1. SaveFile 構造体
```rust
#[derive(Serialize, Deserialize)]
pub struct SaveFile {
    pub version: u32,
    pub timestamp: u64,
    pub script_pos: (String, usize),
    pub global_vars: HashMap<String, i32>,
    pub live2d_state: Vec<Param>,
    pub mini_game_blob: Vec<u8>,
    pub thumbnail: Option<Vec<u8>>,
    pub rng_seed: [u8; 32],
}
```

### 2. シリアライズ/デシリアライズ
```rust
pub fn save_to_bytes(save: &SaveFile) -> Result<Vec<u8>, SaveError> {
    rmp_serde::to_vec_named(save).map_err(SaveError::Serialize)
}
pub fn load_from_bytes(data: &[u8]) -> Result<SaveFile, SaveError> {
    rmp_serde::from_slice(data).map_err(SaveError::Deserialize)
}
```

### 3. SaveManager
- スロット管理 (`slot_000.sav` 〜)
- オートセーブ / クイックセーブ
- スロット一覧取得（サムネイル + 日時）

### 4. バージョンマイグレーション
- `version` フィールドで再帰的マイグレーション
- 後方互換性チェーン

### 5. サムネイルキャプチャ
- Filament フレームバッファ読み取り
- リサイズ (128×72) → JPEG エンコード (品質 70%)

### 6. ECS 連携
```rust
pub enum SaveCommand { Save(usize), Load(usize), AutoSave, QuickSave, QuickLoad }
pub fn save_system(world: &World, manager: &SaveManager);
```

## テスト計画
- SaveFile の round-trip (serialize → deserialize) テスト
- バージョンマイグレーションチェーンテスト
- スロット管理テスト

## 完了条件
- セーブ/ロードの round-trip が成功
- 複数スロット管理
- バージョン1→最新のマイグレーション
