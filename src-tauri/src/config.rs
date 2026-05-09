use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub folder_path: String,
    pub enabled: bool,
    pub copy_mode: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            folder_path: "".to_string(),
            enabled: true,
            copy_mode: "path".to_string(),
        }
    }
}

pub fn load_config() -> Config {
    let path = get_config_path();
    if path.exists() {
        let content = fs::read_to_string(path).unwrap_or_default();
        serde_json::from_str(&content).unwrap_or_default()
    } else {
        Config::default()
    }
}

pub fn save_config(config: &Config) {
    let path = get_config_path();
    if let Some(parent) = path.parent() {
        let _ = fs::create_dir_all(parent);
    }
    let content = serde_json::to_string_pretty(config).unwrap();
    let _ = fs::write(path, content);
}

fn get_config_path() -> PathBuf {
    let mut path = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
    path.push("shotpath");
    path.push("config.json");
    path
}
