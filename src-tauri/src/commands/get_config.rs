use randomizer_core::Config;

use crate::AppState;

#[tauri::command]
pub fn get_config(state: tauri::State<'_, AppState>) -> Config {
    state.randomizer.blocking_read().config().clone()
}
