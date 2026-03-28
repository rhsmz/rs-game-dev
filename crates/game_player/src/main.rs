//! # rs-game-dev Game Player
//!
//! デバッグ機能を省いた配布用軽量ランタイム。
//! アセットアーカイブの解読とメインループの実行に特化する。

struct GamePlayerApp;

impl GamePlayerApp {
    const fn new() -> Self {
        Self
    }
    fn run() {
        log::info!("running main loop...");
    }
}

fn main() {
    match engine_core::logging::try_init_app_logging(env!("CARGO_PKG_NAME")) {
        Ok(Some(path)) => {
            eprintln!("[rs-game-dev] log file: {}", path.display());
            log::info!("writing logs to {}", path.display());
        }
        Ok(None) => log::info!("logging to stderr only (no log file)"),
        Err(e) => {
            eprintln!("logger init failed: {e}");
            std::process::exit(1);
        }
    }
    log::info!("Game Player starting...");
    let _app = GamePlayerApp::new();
    GamePlayerApp::run();
}
