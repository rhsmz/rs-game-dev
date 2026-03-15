use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowConfig {
    pub title: String,
    pub width: u32,
    pub height: u32,
    pub fullscreen: bool,
    pub vsync: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioConfig {
    pub master_volume: f32,
    pub bgm_volume: f32,
    pub se_volume: f32,
    pub voice_volume: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocaleConfig {
    pub language: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub window: WindowConfig,
    pub audio: AudioConfig,
    pub locale: LocaleConfig,
}

impl Config {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let config: Config = toml::from_str(&content)?;
        Ok(config)
    }
}
