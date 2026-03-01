use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileOptions {
    pub no_vr: bool,
    pub enable_debug_gui: bool,
    pub enable_udon_debug_logging: bool,
    pub enable_sdk_log_levels: bool,
    pub disable_hw_video_decoding: bool,
    pub fps: Option<i32>,
    pub process_priority: Option<i32>,
    pub screen_width: Option<i32>,
    pub screen_height: Option<i32>,
    pub fullscreen: Option<bool>,
    pub d3d11: bool,
    pub popupwindow: bool,
    pub custom: String,
}

impl Default for ProfileOptions {
    fn default() -> Self {
        ProfileOptions {
            no_vr: false,
            enable_debug_gui: false,
            enable_udon_debug_logging: false,
            enable_sdk_log_levels: false,
            disable_hw_video_decoding: false,
            fps: None,
            process_priority: None,
            screen_width: None,
            screen_height: None,
            fullscreen: None,
            d3d11: false,
            popupwindow: false,
            custom: String::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Profile {
    pub id: String,
    pub name: String,
    pub options: ProfileOptions,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub vrchat_path: String,
    pub profiles: Vec<Profile>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            vrchat_path: String::new(),
            profiles: Vec::new(),
        }
    }
}

pub fn get_config_path() -> PathBuf {
    let appdata = std::env::var("APPDATA").unwrap_or_else(|_| ".".to_string());
    PathBuf::from(appdata).join("vrcl").join("config.json")
}

pub fn load_config() -> Config {
    let path = get_config_path();
    if path.exists() {
        let content = fs::read_to_string(&path).unwrap_or_default();
        serde_json::from_str(&content).unwrap_or_default()
    } else {
        Config::default()
    }
}

pub fn save_config_to_file(config: &Config) -> Result<(), String> {
    let path = get_config_path();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    let content = serde_json::to_string_pretty(config).map_err(|e| e.to_string())?;
    fs::write(&path, content).map_err(|e| e.to_string())?;
    Ok(())
}
