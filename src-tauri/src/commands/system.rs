use crate::core::config::BuddyConfig;
use tauri::State;

#[tauri::command]
pub fn get_config(config: State<'_, BuddyConfig>) -> BuddyConfig {
    config.inner().clone()
}
