# Filament プリビルト（公式リリース）

メンテナンス用チェックリストは親リポジトリの [`../FILAMENT_SUBMODULE_TASKS.md`](../FILAMENT_SUBMODULE_TASKS.md) を参照してください。

[google/filament](https://github.com/google/filament) の **GitHub Releases** から取得したバイナリ／静的ライブラリを、OS 別ディレクトリに展開して使います。  
中身は Git に含めません（容量が大きいため）。`VERSION` のタグに対応するアーカイブをスクリプトがダウンロードします。

## バージョン

`VERSION` ファイルにタグ名（例: `v1.70.1`）を記載しています。サブモジュール `../filament` も同じタグに揃えることを推奨します。

## 取得方法

リポジトリルートで実行:

- **Windows (PowerShell):** `.\scripts\fetch_filament_prebuilt.ps1`
- **Linux / macOS:** 初回のみ `chmod +x scripts/fetch_filament_prebuilt.sh` のうえ `bash scripts/fetch_filament_prebuilt.sh`

展開先:

| 環境 | ディレクトリ | `FILAMENT_LIB_DIR`（例・自動検出の基準） |
|------|----------------|-------------------------------------------|
| Windows x64 | `windows-x86_64/` | `.../windows-x86_64/lib/x86_64/md` |
| Linux x64 | `linux-x86_64/filament/` | `.../linux-x86_64/filament/lib/x86_64` |
| macOS arm64 | `macos-arm64/filament/` | `.../macos-arm64/filament/lib/arm64` |

### Windows デバッグビルド

Rust のデバッグが `/MDd` の場合、必要に応じて `FILAMENT_LIB_DIR` を `lib/x86_64/mdd` に切り替えてください。

### macOS Intel

公式 `filament-v*-mac.tgz` は arm64 向け静的ライブラリを含むリリースが多いです。Intel Mac ではソースビルド（サブモジュール）か、別途バイナリを用意してください。

## engine_core との連携

環境変数 `FILAMENT_LIB_DIR` に、上表のディレクトリ（`filament.lib` または `libfilament.a` があるフォルダ）を設定すると、`--features filament` ビルドで公式ライブラリをリンクします。  
未設定のときは開発用 C スタブがリンクされます（FFI 名が公式と一致していない場合、プリビルトを指定するとリンクエラーになることがあります）。

## ソース（サブモジュール）

`../filament` に公式リポジトリをサブモジュールで置いています。プリビルトと異なるビルドオプションが必要な場合に利用してください。
