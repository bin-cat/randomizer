#[tauri::command]
pub fn lists() -> Vec<String> {
    randomizer_core::lists()
}
