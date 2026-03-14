# 06: Asset Management — アセット管理

## 概要
非同期アセットローダー、メモリキャッシュ、ローカライズルーティング（言語別フォールバック）、CSV メタデータ読み込みを実装する。

## 依存先
- `01_ecs_foundation`

## モジュール構成
```
crates/engine_core/src/asset/
├── mod.rs          # 公開 API
├── loader.rs       # 非同期アセットローダー
├── cache.rs        # メモリキャッシュ管理
├── router.rs       # ローカライズルーティング
├── metadata.rs     # CSV メタデータ (serde)
├── handle.rs       # AssetHandle (参照カウント)
└── error.rs        # AssetError
```

## タスク

### 1. アセットハンドル
```rust
pub struct AssetHandle<T> {
    id: AssetId,
    _marker: PhantomData<T>,
}

pub enum LoadState {
    Loading,
    Loaded,
    Failed(AssetError),
}
```

- 参照カウント方式のアセット管理
- ロード状態の非同期追跡

### 2. 非同期ローダー
```rust
pub struct AssetLoader {
    base_path: PathBuf,
    cache: AssetCache,
    router: LocaleRouter,
}

impl AssetLoader {
    pub async fn load<T: Asset>(&self, path: &str) -> Result<AssetHandle<T>, AssetError>;
    pub fn load_sync<T: Asset>(&self, path: &str) -> Result<AssetHandle<T>, AssetError>;
}

pub trait Asset: Send + Sync + 'static {
    fn from_bytes(data: &[u8]) -> Result<Self, AssetError> where Self: Sized;
}
```

- `tokio` / `async-std` による非同期ファイル I/O
- `Asset` トレイトによるデコード抽象化

### 3. ローカライズルーティング
```rust
pub struct LocaleRouter {
    current_lang: String,
    default_lang: String, // "ja"
}

impl LocaleRouter {
    pub fn resolve(&self, path: &str) -> PathBuf {
        // 1. common_ 接頭辞 → assets/common/
        // 2. assets/{current_lang}/ を優先
        // 3. フォールバック: assets/{default_lang}/
    }
}
```

- `common_` 接頭辞: `assets/common/` から検索
- 言語別: `assets/{lang}/` → `assets/ja/` フォールバック

### 4. メモリキャッシュ
```rust
pub struct AssetCache {
    entries: HashMap<AssetId, CacheEntry>,
}

struct CacheEntry {
    data: Box<dyn Any + Send + Sync>,
    ref_count: usize,
    last_access: Instant,
}
```

- 参照カウントでライフタイム管理
- シーン遷移時にパージ可能
- LRU 方式のキャッシュ整理（オプション）

### 5. CSV メタデータ読み込み
```rust
#[derive(Deserialize)]
pub struct CharaMeta {
    pub chara_id: String,
    #[serde(rename = "type")]
    pub chara_type: CharaType,
    pub file_path: String,
    pub anchor_x: f32,
    pub anchor_y: f32,
    pub base_scale: f32,
    pub physics: PhysicsToggle,
    pub metadata: String,
}

pub fn load_metadata<T: DeserializeOwned>(path: &Path) -> Result<Vec<T>, AssetError> {
    let mut rdr = csv::Reader::from_path(path)?;
    rdr.deserialize().collect::<Result<Vec<T>, _>>()
        .map_err(AssetError::CsvParse)
}
```

- `chara.csv`, `sound.csv`, `bg.csv`, `images.csv`, `voices.csv` 対応
- `serde` + `csv` クレートによる型安全デシリアライズ

### 6. アセットディスクリプタ (事前宣言)
```rust
pub struct AssetDescriptor {
    pub path: String,
    pub asset_type: AssetType,
    pub priority: LoadPriority,
}

pub enum AssetType {
    Texture, Model, Sound, Script, Metadata,
}
```

- Scene の `required_assets()` で返す事前宣言リスト
- ロード画面での進捗表示対応

## テスト計画
- ルーティング解決テスト（`common_` 接頭辞、フォールバック）
- CSV メタデータデシリアライズテスト
- キャッシュ挿入/取得/パージテスト

## 完了条件
- ファイルパスのローカライズルーティングが正確
- CSV メタデータ 5 種の読み込み成功
- アセットキャッシュのライフサイクル管理
