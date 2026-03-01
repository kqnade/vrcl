mod config;
mod vrchat;

use config::{load_config, save_config_to_file, Config, Profile};

#[tauri::command]
fn get_config() -> Config {
    load_config()
}

#[tauri::command]
fn save_config(config: Config) -> Result<(), String> {
    save_config_to_file(&config)
}

#[tauri::command]
fn detect_vrchat_path() -> Result<String, String> {
    vrchat::detect_vrchat_path()
}

#[tauri::command]
fn launch_vrchat(vrchat_path: String, profile: Profile) -> Result<(), String> {
    vrchat::launch_vrchat(&vrchat_path, &profile)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_config,
            save_config,
            detect_vrchat_path,
            launch_vrchat,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
