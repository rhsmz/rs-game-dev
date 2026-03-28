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
    log::info!("Script Editor starting...");
    let _app = ScriptEditorApp::new();
    ScriptEditorApp::run();
}
