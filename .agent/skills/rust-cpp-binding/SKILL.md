---
name: rust-cpp-binding
description: C++ライブラリのRustバインディングを作成・保守するためのスキル。C APIラッパーの作成、build.rsによる自動ビルド、安全なRust層の構築という堅牢なパターンを踏襲して実装を行う際に使用する。
---

# Rust C++ Binding Skill

## 🎯 目的 (Purpose)
このスキルは、C++で書かれたライブラリをRustから安全に呼び出すためのFFI（Foreign Function Interface）バインディングを構築・保守する手順を定義する。
直接C++の機能をバインドするのではなく、中間にC言語のインターフェース（Cラッパー）を挟む設計思想に従うことで、`bindgen`との親和性とABIの安定性を確保する。

## 🏗️ アーキテクチャ構成 (Architecture Pattern)
エージェントはバインディングを以下の3層構造で実装すること。

1. **C API Wrapper (`bindings.cpp` / `bindings.h`)**
   - C++のクラスやメソッドを `extern "C"` のフラットな関数群としてラップする。
   - C++固有の機能（テンプレート、オーバーロード等）はここで吸収し、Rustへは不透明ポインタ（Opaque Pointer: `void*` 等）として構造体を渡す。
2. **Build Script (`build.rs`) & Dependencies**
   - `Cargo.toml` の `[build-dependencies]` に `cc` と `bindgen` を明記する。
   - `cc` クレートを使用して `bindings.cpp` をコンパイルする。コンパイル時は必ず `cpp(true)` と `flag("-std=c++17")` などのC++バージョンの指定を行うこと。
   - `bindgen::Builder` を使用して `bindings.h` からRustの生バインディング (`bindings.rs`) を自動生成する。
   - 事前ビルド済みバイナリが存在する場合は、ダウンロードスクリプトと連携し、適切なパスに対して `cargo:rustc-link-lib` と `cargo:rustc-link-search` を出力する。
3. **Safe Rust Wrapper (`src/`)**
   - `bindgen` が生成した `unsafe` な生関数群を隠蔽し、Rustの所有権モデルに従った安全な構造体を提供する。

## 🚀 実行手順 (Execution Steps)
新しいバインディングを作成、または既存のものを修正する際は以下の手順で進める。

1. **C++ヘッダの解析**: バインド対象となるC++ライブラリのパブリックAPIを読み解く。
2. **Cラッパーの実装**:
   - `bindings.cpp` にインスタンス生成関数（例: `_create`）と破棄関数（例: `_destroy`）を実装する。
   - メソッド呼び出しは、第一引数に対象インスタンスのポインタを受け取るC関数として定義する。
3. **ビルド設定**: `build.rs` を更新し、対象のC++ファイルが正しくコンパイル・リンクされるようにする（プラットフォームごとのリンクフラグ分岐に注意する）。
4. **Rustセーフティ層の構築**:
   - 不透明ポインタをラップするRust構造体を作成する。
   - 対象となるC++インスタンスがスレッドセーフ（スレッド間での共有・移動が可能）であることが保証されている場合に限り、Rustのラッパー構造体に `unsafe impl Send` および `Sync` を実装する。
   - 構造体に `Drop` トレイトを実装し、C++側の破棄関数を呼んでメモリリークを防ぐ。
   - エラーハンドリングは `Result` 型に変換し、Rustらしいメソッドチェーンを提供する。

## ⚠️ コーディング規約 (Conventions & Best Practices)
- **生ポインタの隠蔽**: パブリックなRust API（公開クレート）に生ポインタや `unsafe` ブロックをそのまま露出させてはならない。
- **例外の処理**: C++の例外（Exception）はFFIの境界を越えられないため、必ず `bindings.cpp` 側で `try-catch` してエラーコード（列挙型など）に変換してからRustへ返すこと。
- **命名規則**: Cラッパーの関数名が他のライブラリと衝突しないよう、一貫したプレフィックスを付与すること。