//! # rs-game-dev Script Editor Main

struct ScriptEditorApp;

impl ScriptEditorApp {
    fn new() -> Self {
        Self
    }
    fn run(&mut self) {
        log::info!("running editor loop...");
    }
}

fn main() {
    env_logger::init();
    log::info!("Script Editor starting...");
    let mut app = ScriptEditorApp::new();
    app.run();
}
