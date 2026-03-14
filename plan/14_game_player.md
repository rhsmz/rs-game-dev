# 14: Game Player — 配布用ランタイム

## 概要
デバッグ機能を削ぎ落とした配布用軽量ランタイム。アセットアーカイブの解読とメインループの実行に特化する。

## 依存先
- `08_scene_management`, `12_save_system`

## モジュール構成
```
crates/game_player/src/
├── main.rs             # エントリーポイント
├── runtime/
│   ├── mod.rs          # ランタイムコア
│   ├── game_loop.rs    # メインループ
│   ├── config.rs       # ランタイム設定 (TOML)
│   └── window.rs       # ウィンドウ管理
└── archive/
    ├── mod.rs          # アーカイブ解読
    ├── reader.rs       # .pak ファイルリーダー
    └── index.rs        # アセットインデックス
```

## タスク

### 1. メインループ
```rust
fn main() -> anyhow::Result<()> {
    let config = Config::load("config.toml")?;
    let mut engine = EngineCore::new(&config)?;
    let archive = AssetArchive::open("assets.pak")?;

    engine.set_asset_source(archive);
    engine.push_scene(TitleScene::new());

    // 固定タイムステップ + 可変レンダリング
    let mut last_time = Instant::now();
    loop {
        let now = Instant::now();
        let dt = (now - last_time).as_secs_f32();
        last_time = now;

        engine.process_input();
        engine.update(dt);
        engine.render();

        if engine.should_quit() { break; }
    }
    Ok(())
}
```

### 2. ランタイム設定
```toml
# config.toml
[window]
title = "Game Title"
width = 1920
height = 1080
fullscreen = false
vsync = true

[audio]
master_volume = 1.0
bgm_volume = 0.8
se_volume = 1.0
voice_volume = 1.0

[locale]
language = "ja"
```

### 3. アセットアーカイブ (.pak)
```rust
pub struct AssetArchive {
    file: BufReader<File>,
    index: HashMap<String, ArchiveEntry>,
}

struct ArchiveEntry {
    offset: u64,
    size: u64,
    compressed: bool,
}

impl AssetArchive {
    pub fn open(path: &str) -> Result<Self, ArchiveError>;
    pub fn read_asset(&mut self, path: &str) -> Result<Vec<u8>, ArchiveError>;
}
```

- アセットファイルを単一 .pak にパック
- ファイルインデックス（パス → オフセット + サイズ）
- オプション: LZ4 圧縮

### 4. ウィンドウ管理
- `winit` によるウィンドウ作成
- フルスクリーン / ウィンドウ切替
- リサイズ対応

### 5. Feature フラグによるデバッグ除外
```toml
# crates/engine_core/Cargo.toml
[features]
default = []
editor = []  # script_editor 向け機能
debug_ui = []  # デバッグオーバーレイ
```

- `game_player` はデフォルト features のみ使用
- `script_editor` は `editor` + `debug_ui` features 有効

## テスト計画
- .pak アーカイブ作成 / 読み込みテスト
- 設定ファイルのパーステスト
- メインループの起動 / 終了テスト

## 完了条件
- .pak アーカイブからアセットを読み込んで動作
- config.toml 設定の反映
- タイトル画面 → ゲーム本編の遷移
