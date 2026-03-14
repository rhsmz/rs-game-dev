---
name: save_system
description: MessagePack ベースのセーブ/ロードシステム実装スキル
---

# セーブシステム実装スキル

## 概要

ゲームの進行状態を MessagePack 形式 (`rmp-serde`) でシリアライズ/デシリアライズする。セーブデータにはスクリプト位置、グローバル変数、Live2D/3D 状態、ミニゲームデータ、サムネイルを含む。

## セーブデータ構造

```rust
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct SaveFile {
    /// セーブデータフォーマットバージョン（マイグレーション用）
    pub version: u32,
    /// セーブ日時（UNIX タイムスタンプ）
    pub timestamp: u64,
    /// スクリプト位置（ファイル名, 行番号）
    pub script_pos: (String, usize),
    /// グローバル変数（好感度、フラグ等）
    pub global_vars: HashMap<String, i32>,
    /// Live2D パラメータ状態（ポーズ復元用）
    pub live2d_state: Vec<Param>,
    /// ミニゲーム固有のシリアライズデータ
    pub mini_game_blob: Vec<u8>,
    /// サムネイル画像（JPEG バイナリ）
    pub thumbnail: Option<Vec<u8>>,
    /// LogicRng シード状態（リプレイ再現用）
    pub rng_seed: [u8; 32],
}

#[derive(Serialize, Deserialize)]
pub struct Param {
    pub id: String,
    pub value: f32,
}
```

## シリアライズ / デシリアライズ

```rust
/// MessagePack シリアライズ（フィールド名付きで互換性確保）
pub fn save_to_bytes(save: &SaveFile) -> Result<Vec<u8>, SaveError> {
    // to_vec_named: フィールド名を保持（バージョン間互換性に有利）
    // to_vec: コンパクト配列形式（サイズ優先の場合）
    rmp_serde::to_vec_named(save).map_err(SaveError::Serialize)
}

pub fn load_from_bytes(data: &[u8]) -> Result<SaveFile, SaveError> {
    rmp_serde::from_slice(data).map_err(SaveError::Deserialize)
}
```

## ファイル I/O

```rust
use std::path::Path;

pub fn save_to_file(save: &SaveFile, path: &Path) -> Result<(), SaveError> {
    let bytes = save_to_bytes(save)?;
    std::fs::write(path, bytes).map_err(SaveError::Io)?;
    Ok(())
}

pub fn load_from_file(path: &Path) -> Result<SaveFile, SaveError> {
    let bytes = std::fs::read(path).map_err(SaveError::Io)?;
    load_from_bytes(&bytes)
}
```

## セーブスロット管理

```rust
pub struct SaveManager {
    save_dir: PathBuf,
    max_slots: usize,
}

impl SaveManager {
    pub fn new(save_dir: PathBuf, max_slots: usize) -> Self {
        Self { save_dir, max_slots }
    }

    /// スロット一覧取得（サムネイル・日時付き）
    pub fn list_slots(&self) -> Result<Vec<SaveSlotInfo>, SaveError> {
        // save_dir 内の .sav ファイルをスキャン
        // ヘッダーのみ読み込んでサムネイル・日時を返す
        todo!()
    }

    /// 指定スロットにセーブ
    pub fn save(&self, slot: usize, save: &SaveFile) -> Result<(), SaveError> {
        let path = self.save_dir.join(format!("slot_{:03}.sav", slot));
        save_to_file(save, &path)
    }

    /// 指定スロットからロード
    pub fn load(&self, slot: usize) -> Result<SaveFile, SaveError> {
        let path = self.save_dir.join(format!("slot_{:03}.sav", slot));
        load_from_file(&path)
    }

    /// オートセーブ
    pub fn auto_save(&self, save: &SaveFile) -> Result<(), SaveError> {
        let path = self.save_dir.join("auto_save.sav");
        save_to_file(save, &path)
    }
}
```

## バージョンマイグレーション

セーブデータの `version` フィールドでフォーマット変更に対応。

```rust
pub fn migrate(mut save: SaveFile) -> Result<SaveFile, SaveError> {
    match save.version {
        1 => {
            // v1 → v2: rng_seed フィールド追加
            // デフォルト値をセット
            save.version = 2;
            migrate(save)  // 再帰的にチェーン
        }
        2 => Ok(save),  // 最新バージョン
        _ => Err(SaveError::UnsupportedVersion(save.version)),
    }
}
```

## エラー型

```rust
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SaveError {
    #[error("serialization error: {0}")]
    Serialize(#[from] rmp_serde::encode::Error),
    #[error("deserialization error: {0}")]
    Deserialize(#[from] rmp_serde::decode::Error),
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("unsupported save version: {0}")]
    UnsupportedVersion(u32),
}
```

## サムネイル生成

セーブ時に現在のゲーム画面をキャプチャし、JPEG でエンコードしてセーブデータに含める。

```rust
pub fn capture_thumbnail(renderer: &Renderer, width: u32, height: u32) -> Vec<u8> {
    // Filament のフレームバッファを読み取り
    // リサイズ（128x72 等の低解像度）
    // JPEG エンコード
    todo!()
}
```

## ECS 連携

```rust
pub enum SaveCommand {
    Save(usize),       // スロット番号
    Load(usize),
    AutoSave,
    QuickSave,
    QuickLoad,
}

// セーブ System
pub fn save_system(/* ECS クエリ */) {
    // SaveCommand をチェック
    // 現在の ECS 状態から SaveFile を構築
    // SaveManager 経由でファイルに書き出し
}
```

## 注意事項

- セーブデータには機密情報を含めない（チート対策は別途検討）
- ロード時は必ず `migrate()` を通してバージョン互換性を確保
- サムネイル画像は圧縮品質 70% 程度でファイルサイズを抑制
