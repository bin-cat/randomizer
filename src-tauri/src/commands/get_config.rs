use crate::{config::Config, AppState};

#[tauri::command]
pub fn get_config(state: tauri::State<'_, AppState>) -> Config {
    state.config.blocking_read().clone()
}
