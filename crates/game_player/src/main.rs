//! # rs-game-dev Game Player
//!
//! デバッグ機能を省いた配布用軽量ランタイム。
//! アセットアーカイブの解読とメインループの実行に特化する。

struct GamePlayerApp;

impl GamePlayerApp {
    const fn new() -> Self {
        Self
    }
    fn run(&self) {
        let _ = self; // unused self warning
        log::info!("running main loop...");
    }
}

fn main() {
    env_logger::init();
    log::info!("Game Player starting...");
    let app = GamePlayerApp::new();
    app.run();
}
