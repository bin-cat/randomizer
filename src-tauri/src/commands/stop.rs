use crate::AppState;

#[tauri::command]
pub fn stop(state: tauri::State<AppState>) {
    let mut w = state.stop_roll.blocking_write();
    *w = true;
}
