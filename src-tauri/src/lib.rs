pub mod commands;
pub mod core;
pub mod llm;
pub mod storage;
pub mod system;

use crate::core::config::BuddyConfig;
use std::env;
use std::path::PathBuf;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Determine the base path for config
    let mut config_path = PathBuf::from("buddy-data");
    config_path.push("config.toml");
    
    // Load config (or generate default if not present)
    let config = BuddyConfig::load(&config_path);

    tauri::Builder::default()
        .manage(config)
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            commands::system::get_config
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
