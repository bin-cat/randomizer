use anyhow::Context;
use randomizer_core::list_audio_devices;

use crate::error::Result;

#[tauri::command]
pub fn get_audio_devices() -> Result<Vec<(String, String)>> {
    Ok(list_audio_devices().with_context(|| "Failed to get list of audio devices")?)
}
