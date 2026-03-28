# third_party — 外部依存の置き場

| パス | 内容 |
|------|------|
| [`filament/`](./filament) | **プライベートミラー** [rhsmz/filament-rs-game-dev](https://github.com/rhsmz/filament-rs-game-dev)（ブランチ `private`）。上流は [google/filament](https://github.com/google/filament)。 |
| [`filament_prebuilt/`](./filament_prebuilt) | 公式 **GitHub Releases** から取得したライブラリの展開先（中身は Git 管理外）。 |
| [`FILAMENT_SUBMODULE_TASKS.md`](./FILAMENT_SUBMODULE_TASKS.md) | 上記 Filament まわりの **メンテナンス用タスク**（rs-game-dev 側で管理）。 |

初回クローン後（[Task](https://taskfile.dev) 利用時）:

```bash
task submodules:init
```

手動の場合:

```bash
git submodule update --init --recursive
```

よく使うタスク: `task submodules:status` / `task submodules:sync` / `task submodules:foreach-fetch`  
Filament プリビルト: Windows は `task submodules:filament:prebuilt:win`、Linux/macOS は `task submodules:filament:prebuilt:unix`

プリビルトの詳細は `filament_prebuilt/README.md` と `scripts/fetch_filament_prebuilt.*` を参照してください。

### プライベートミラー初回プッシュ

`.gitmodules` を private URL にしたあと、**ミラーにオブジェクトが無いと `submodule update` が失敗**します。リポジトリ管理者は、親リポジトリを共有する前に次を実行してください。

- Windows: `task submodules:filament:push-mirror` または `.\scripts\push_filament_private_mirror.ps1`
- Unix: `task submodules:filament:push-mirror:unix` または `bash scripts/push_filament_private_mirror.sh`

詳細は [`FILAMENT_SUBMODULE_TASKS.md`](./FILAMENT_SUBMODULE_TASKS.md) を参照。
