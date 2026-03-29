# Filament サブモジュール向けタスク — rs-game-dev

> **対象パス:** `third_party/filament`（プライベートミラー [rhsmz/filament-rs-game-dev](https://github.com/rhsmz/filament-rs-game-dev) のブランチ `private`）  
> **上流:** [google/filament](https://github.com/google/filament)（タグ追従・差分取り込み用）  
> **プリビルト:** `third_party/filament_prebuilt`（[Releases](https://github.com/google/filament/releases) から取得）  
> **最終更新:** 2026-03-28

本ファイルは **rs-game-dev リポジトリ側**で管理します（上流 Filament にコミットしない）。サブモジュール内で作業するときのチェックリストとして使います。

---

## プライベートミラー（必読）

- [ ] 初回またはミラーが空のとき、`scripts/push_filament_private_mirror.ps1`（または `.sh` / `task submodules:filament:push-mirror`）を実行し、`private` ブランチに **superproject が指すコミット**が存在することを確認した（`git ls-remote git@github.com:rhsmz/filament-rs-game-dev.git`）  
- [ ] 親リポジトリのブランチを他者へ push する前に上記が完了している（未完了だと clone 先で `submodule update` が失敗する）  
- [ ] 上流を取り込む場合は `third_party/filament` で `git remote add upstream https://github.com/google/filament.git`（未登録なら）し、`fetch` / cherry-pick / merge の方針を決めた

## バージョン整合

- [ ] サブモジュールが `third_party/filament_prebuilt/VERSION` と同じタグ（例: `v1.70.1`）を指している  
- [ ] `git submodule update --init --recursive` で他開発者と同じコミットになることを確認した  
- [ ] タグを上げる場合は `VERSION`・`fetch_filament_prebuilt.*` の説明・本タスクをまとめて更新し、必要ならミラーへタグを `git push` する

---

## プリビルト（公式バイナリ）

- [ ] 各 OS で `task submodules:filament:prebuilt:win` または `task submodules:filament:prebuilt:unix`（または `scripts/fetch_filament_prebuilt.*`）を実行し、展開先が README の想定パスになっている  
- [ ] `FILAMENT_LIB_DIR` を未設定のままスタブで開発するか、プリビルトを使うかチームで方針を揃えた  
- [ ] Windows デバッグで `/MDd` が必要な場合、`lib/x86_64/mdd` への切り替えを検討した

---

## FFI / エンジン連携（engine_core）

- [ ] `crates/engine_core/src/ffi/filament_sys.rs` のシンボルが、リンクしている Filament（プリビルトまたは自前ビルド）の **実際の C/C++ エクスポート**と一致している  
- [ ] `crates/engine_core/stub/filament_stub.c` を、公式 API に合わせて更新するか、プリビルト専用に分岐する方針が決まっている  
- [ ] `cargo test -p engine_core --features filament`（`filament_smoke`）がローカルで通る

---

## 上流追従・メンテナンス

- [ ] Filament マイナー/パッチ更新時に **Release Notes**（破壊的変更・バックエンド削除等）を確認した  
- [ ] 必要なら `plan/02_renderer.md` やルート `TASKS.MD` の Renderer 項目を更新した  
- [ ] サブモジュールを shallow のまま運用するか、`depth` を増やすか方針を README に残した

---

## 参考

- サブモジュール初期化: `task submodules:init` または `git submodule update --init --recursive`
- 共通タスク定義: [`Taskfile.yml`](./Taskfile.yml)（ルート `Taskfile.yml` から `submodules:` 名前空間で include）  
- プリビルト手順: `third_party/filament_prebuilt/README.md`  
- ビルド手順（ソースから）: サブモジュール内 `BUILDING.md`（[Filament リポジトリ](https://github.com/google/filament)）
