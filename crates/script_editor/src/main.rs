//! # rs-game-dev Script Editor Main

struct ScriptEditorApp;

impl ScriptEditorApp {
    const fn new() -> Self {
        Self
    }
    fn run(&self) {
        let _ = self; // unused self warning
        log::info!("running editor loop...");
    }
}

fn main() {
    env_logger::init();
    log::info!("Script Editor starting...");
    let app = ScriptEditorApp::new();
    app.run();
}
