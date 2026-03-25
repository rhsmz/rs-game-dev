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
    env_logger::init();
    log::info!("Game Player starting...");
    let _app = GamePlayerApp::new();
    GamePlayerApp::run();
}
