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

## 📝 実装例 (Example)

### 1. C API Wrapper (`bindings.h` / `bindings.cpp`)

```cpp
// bindings.h
#pragma once

#ifdef __cplusplus
extern "C" {
#endif

typedef void* MyClassHandle;

MyClassHandle my_class_create(int value);
void my_class_destroy(MyClassHandle handle);
int my_class_do_something(MyClassHandle handle);

#ifdef __cplusplus
}
#endif
```

```cpp
// bindings.cpp
#include "bindings.h"
#include "my_class.hpp" // C++の元ヘッダ

extern "C" {

MyClassHandle my_class_create(int value) {
    return new MyClass(value);
}

void my_class_destroy(MyClassHandle handle) {
    if (handle) {
        delete static_cast<MyClass*>(handle);
    }
}

int my_class_do_something(MyClassHandle handle) {
    if (!handle) return -1;
    try {
        return static_cast<MyClass*>(handle)->doSomething();
    } catch (...) {
        // 例外をキャッチし、適切なエラーコードを返す
        return -1; 
    }
}

}
```

### 2. Safe Rust Wrapper (`src/lib.rs`)

```rust
// bindgen で生成された生関数群
mod ffi {
    #![allow(non_upper_case_globals, non_camel_case_types, non_snake_case)]
    // 実際には `include!(concat!(env!("OUT_DIR"), "/bindings.rs"));` などを利用
    extern "C" {
        pub type MyClassHandle = *mut std::ffi::c_void;
        pub fn my_class_create(value: std::os::raw::c_int) -> MyClassHandle;
        pub fn my_class_destroy(handle: MyClassHandle);
        pub fn my_class_do_something(handle: MyClassHandle) -> std::os::raw::c_int;
    }
}

pub struct MyClass {
    handle: ffi::MyClassHandle,
}

// 該当C++インスタンスがスレッドセーフ（共有/移動可能）である保証がある場合のみ実装
unsafe impl Send for MyClass {}
unsafe impl Sync for MyClass {}

impl MyClass {
    pub fn new(value: i32) -> Result<Self, &'static str> {
        let handle = unsafe { ffi::my_class_create(value) };
        if handle.is_null() {
            return Err("Failed to create MyClass instance");
        }
        Ok(Self { handle })
    }

    pub fn do_something(&self) -> Result<i32, &'static str> {
        let result = unsafe { ffi::my_class_do_something(self.handle) };
        // エラーコード(-1)をResult型へ変換
        if result == -1 {
            Err("An exception occurred in C++ code")
        } else {
            Ok(result)
        }
    }
}

impl Drop for MyClass {
    fn drop(&mut self) {
        if !self.handle.is_null() {
            unsafe { ffi::my_class_destroy(self.handle) };
        }
    }
}
```