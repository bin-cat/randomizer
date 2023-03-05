use anyhow::Context;

use crate::{audio_player::Player, config::Config, error::Result, AppState};

#[tauri::command]
pub fn set_config(config: Config, state: tauri::State<'_, AppState>) -> Result<()> {
    let mut state_config = state.config.blocking_write();
    *state_config = config;
    Player::set_volume(state_config.volume()).with_context(|| "Failed to set sound volume")?;
    Player::set_device(
        Player::find_device_index(state_config.audio_device())
            .with_context(|| "Failed to find audio device index")?,
    )
    .with_context(|| "Failed to set audio device")?;
    Ok(state_config
        .save()
        .with_context(|| "Failed to save configuration to file")?)
}
