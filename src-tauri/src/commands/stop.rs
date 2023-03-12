use crate::AppState;

#[tauri::command]
pub fn stop(state: tauri::State<AppState>) {
    state.randomizer.blocking_read().stop_roll();
}
