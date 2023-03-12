use anyhow::Context;
use randomizer_core::Config;

use crate::{error::Result, AppState};

#[tauri::command]
pub fn set_config(config: Config, state: tauri::State<'_, AppState>) -> Result<()> {
    Ok(state
        .randomizer
        .blocking_write()
        .set_config(config)
        .with_context(|| "Failed to apply new config")?)
}
