//! # rs-game-dev Script Editor Main

struct ScriptEditorApp;

impl ScriptEditorApp {
    const fn new() -> Self {
        Self
    }
    fn run() {
        log::info!("running editor loop...");
    }
}

fn main() {
    env_logger::init();
    log::info!("Script Editor starting...");
    let _app = ScriptEditorApp::new();
    ScriptEditorApp::run();
}
