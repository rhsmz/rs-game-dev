use std::env;
use std::path::{Path, PathBuf};

/// Windows 公式プリビルト（README の `mt` 例）と同じ静的ライブラリ名（拡張子なし）。
const FILAMENT_STATIC_LIBS: &[&str] = &[
    "filament",
    "backend",
    "bluegl",
    "bluevk",
    "filabridge",
    "filaflat",
    "utils",
    "geometry",
    "smol-v",
    "ibl",
    "abseil",
    // 公式ミニマム README より後続リリースで追加（Material 圧縮など）
    "zstd",
];

fn resolve_filament_include(lib_dir: &str) -> PathBuf {
    if let Ok(p) = env::var("FILAMENT_INCLUDE_DIR") {
        return PathBuf::from(p);
    }
    let base = Path::new(lib_dir);
    // Linux/mac 公式展開: `.../filament/lib/x86_64` → `../../include` で `.../filament/include`
    // Windows 公式展開: `.../windows-x86_64/lib/x86_64/md` → `../../../include` でパッケージ直下の `include`
    let candidates = [base.join("../../include"), base.join("../../../include")];
    for candidate in candidates {
        let engine_h = candidate.join("filament").join("Engine.h");
        if engine_h.is_file() {
            // Windows で `canonicalize()` が `\\?\` 付きパスを返すと、環境によっては cl.exe が
            // `-I` を解釈できずコンパイルが失敗するため、存在確認できたパスをそのまま使う。
            return candidate;
        }
    }
    panic!(
        "FILAMENT_INCLUDE_DIR が未設定で、FILAMENT_LIB_DIR から include を解決できません。\n\
         試した相対パス: ../../include, ../../../include（基準: {}）\n\
         `FILAMENT_INCLUDE_DIR` に Filament の `include` ディレクトリを明示するか、\n\
         LIB_DIR を公式 README 通り（例: Windows は lib/x86_64/md または mdd）に指定してください。",
        base.display()
    );
}

/// Windows プリビルトの `mdd` / `mtd` は公式 README 通り `matdbg` もリンクする。
fn windows_filament_debug_libs(lib_dir: &str) -> bool {
    let lower = lib_dir.to_ascii_lowercase();
    lower.contains("mdd") || lower.contains("mtd")
}

fn link_filament_bundle(lib_dir: &str) {
    println!("cargo:rustc-link-search=native={lib_dir}");
    for lib in FILAMENT_STATIC_LIBS {
        println!("cargo:rustc-link-lib=static={lib}");
    }

    match env::var("CARGO_CFG_TARGET_OS").as_deref() {
        Ok("windows") => {
            if windows_filament_debug_libs(lib_dir) {
                println!("cargo:rustc-link-lib=static=matdbg");
            }
            println!("cargo:rustc-link-lib=gdi32");
            println!("cargo:rustc-link-lib=user32");
            println!("cargo:rustc-link-lib=opengl32");
        }
        Ok("macos") => {
            println!("cargo:rustc-link-lib=c++");
            println!("cargo:rustc-link-lib=framework=Cocoa");
            println!("cargo:rustc-link-lib=framework=Metal");
            println!("cargo:rustc-link-lib=framework=CoreVideo");
        }
        Ok("linux") => {
            println!("cargo:rustc-link-lib=stdc++");
            println!("cargo:rustc-link-lib=pthread");
            println!("cargo:rustc-link-lib=dl");
        }
        _ => {}
    }
}

fn main() {
    if env::var_os("CARGO_FEATURE_FILAMENT").is_none() {
        return;
    }

    println!("cargo:rerun-if-env-changed=FILAMENT_LIB_DIR");
    println!("cargo:rerun-if-env-changed=FILAMENT_INCLUDE_DIR");
    println!("cargo:rerun-if-changed=stub/filament_stub.c");
    println!("cargo:rerun-if-changed=stub/filament_bridge.cpp");

    if let Ok(lib_dir) = env::var("FILAMENT_LIB_DIR") {
        let include_dir = resolve_filament_include(&lib_dir);

        let mut bridge = cc::Build::new();
        bridge.cpp(true);
        bridge.std("c++20");
        bridge.file("stub/filament_bridge.cpp");
        bridge.include(&include_dir);
        bridge.warnings(false);
        bridge.flag_if_supported("-Wno-unused-parameter");
        // cc-rs は MSVC でも常に `-MD` を付けるため、`mdd`/`mtd` プリビルトと CRT が食い違う。
        // `/MDd` を後から付与し、最終的にデバッグ DLL ランタイムで揃える。
        if env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("windows")
            && windows_filament_debug_libs(&lib_dir)
        {
            bridge.flag("/MDd");
        }
        bridge.compile("filament_rs_bridge");

        link_filament_bundle(&lib_dir);
        return;
    }

    // `FILAMENT_LIB_DIR` 未設定時は最小スタブを静的リンクする。
    // プリビルト取得: `task deps:filament` 等。展開先の `lib/...` を `FILAMENT_LIB_DIR` に指定。
    let mut build = cc::Build::new();
    build.file("stub/filament_stub.c");
    build.std("c11");
    build.compile("filament_rs_dev_stub");
}
