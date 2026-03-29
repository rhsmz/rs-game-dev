# engine_core — `unsafe` とスレッド境界

このクレートでは **FFI（Filament / 将来の Live2D 等）の境界にだけ `unsafe` を置く**方針です（プロジェクトルール）。

## `unsafe impl Send` / `Sync`（`GameView` / `UiView`）

- **前提:** Filament の `Engine` / `View` / `Scene` ポインタは **メインスレッド（ウィンドウ＆描画を所有するスレッド）だけ**で操作する。
- **理由:** ECS の `World` にリソースとして載せるには `Send + Sync` が必要な場合がある。ポインタ自体は共有されず、フレームごとに単一スレッドからアクセスする設計とする。
- **禁止:** 別スレッドから `RenderEngine` / `GameView` / `UiView` の Filament ハンドルを読み書きしない。並列レンダリングに移行する場合は、この `unsafe impl` を削除し、専用スレッド＋メッセージパッシングに置き換える。

## FFI ブロック

- すべての `unsafe` ブロックに **`// SAFETY:`** で不変条件と null チェック・寿命を明示する（`filament_sys` 呼び出し側）。

## レビュー時チェックリスト

1. 新規 `unsafe` は `ffi/` またはブリッジ層に閉じ込めているか。
2. `Send`/`Sync` を手書きした型は、**どのスレッドから触るか**がドキュメントと一致しているか。
3. ポインタの寿命が Rust の所有者（例: `RenderEngine`）と対応しているか。
