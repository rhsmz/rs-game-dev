use std::env;

fn main() {
    if env::var_os("CARGO_FEATURE_FILAMENT").is_none() {
        return;
    }

    println!("cargo:rerun-if-env-changed=FILAMENT_LIB_DIR");
    println!("cargo:rerun-if-env-changed=FILAMENT_LIB_NAME");
    println!("cargo:rerun-if-changed=stub/filament_stub.c");

    if let Ok(lib_dir) = env::var("FILAMENT_LIB_DIR") {
        let lib_name =
            env::var("FILAMENT_LIB_NAME").unwrap_or_else(|_| "filament".to_string());
        println!("cargo:rustc-link-search=native={lib_dir}");
        println!("cargo:rustc-link-lib={lib_name}");
        return;
    }

    // `FILAMENT_LIB_DIR` 未設定時は最小スタブを静的リンクする。
    // 公式プリビルトは `scripts/fetch_filament_prebuilt.*` で展開し、
    // `third_party/filament_prebuilt/README.md` のパスを `FILAMENT_LIB_DIR` に指定する。
    let mut build = cc::Build::new();
    build.file("stub/filament_stub.c");
    build.std("c11");
    build.compile("filament_rs_dev_stub");
}
