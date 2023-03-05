use crate::{audio_player::Player, error::Result};

#[tauri::command]
pub fn get_audio_devices() -> Result<Vec<(String, String)>> {
    Ok(Player::list_devices()?)
}
