//! # rs-game-dev Game Player
//!
//! デバッグ機能を省いた配布用軽量ランタイム。
//! アセットアーカイブの解読とメインループの実行に特化する。

pub mod archive;
pub mod runtime;

use archive::AssetArchive;
use engine_core::{EngineCore, TitleScene};
use runtime::config::Config;
use runtime::game_loop::GameLoop;

fn main() -> anyhow::Result<()> {
    env_logger::init();
    log::info!("Game Player starting...");

    // Optional config load (mock failure handled gracefully for now)
    let _config = Config::load("config.toml").unwrap_or_else(|_| {
        log::warn!("Failed to load config.toml, using defaults");
        Config {
            window: runtime::config::WindowConfig {
                title: "Game Title".to_string(),
                width: 1920,
                height: 1080,
                fullscreen: false,
                vsync: true,
            },
            audio: runtime::config::AudioConfig {
                master_volume: 1.0,
                bgm_volume: 0.8,
                se_volume: 1.0,
                voice_volume: 1.0,
            },
            locale: runtime::config::LocaleConfig {
                language: "ja".to_string(),
            },
        }
    });

    let mut engine = EngineCore::new();

    // Attempt to load archive, default if unavailable for tests
    if let Ok(archive) = AssetArchive::open("assets.pak") {
        engine.set_asset_source(Some(archive));
    } else {
        log::warn!("assets.pak not found. Running without assets.");
        engine.set_asset_source::<AssetArchive>(None);
    }

    engine.push_scene(TitleScene::new());

    let mut game_loop = GameLoop::new(engine);
    game_loop.run();

    log::info!("Game Player finished.");
    Ok(())
}
