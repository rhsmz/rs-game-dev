fn main() {
    // Filament 本体が存在しない開発環境ではリンクエラーになるため、
    // Cargo feature `filament` が有効な場合のみリンク設定を適用する。
    if std::env::var_os("CARGO_FEATURE_FILAMENT").is_none() {
        return;
    }

    println!("cargo:rerun-if-env-changed=FILAMENT_LIB_DIR");
    println!("cargo:rerun-if-env-changed=FILAMENT_LIB_NAME");
    println!("cargo:rerun-if-changed=stub/filament_stub.c");

    if let Ok(lib_dir) = std::env::var("FILAMENT_LIB_DIR") {
        let lib_name =
            std::env::var("FILAMENT_LIB_NAME").unwrap_or_else(|_| "filament".to_string());
        println!("cargo:rustc-link-search=native={lib_dir}");
        println!("cargo:rustc-link-lib={lib_name}");
        return;
    }

    // `FILAMENT_LIB_DIR` 未設定時は最小スタブを静的リンクし、CI / ローカルで
    // `cargo test --features filament` が成立するようにする（実 GPU 描画は行わない）。
    let mut build = cc::Build::new();
    build.file("stub/filament_stub.c");
    build.std("c11");
    build.compile("filament_rs_dev_stub");
}
