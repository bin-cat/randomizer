use anyhow::Context;
use log::error;
use tauri::Window;

use crate::{error::Result, AppState};

#[tauri::command]
pub async fn roll(
    list_name: String,
    state: tauri::State<'_, AppState>,
    window: Window,
) -> Result<()> {
    state
        .randomizer
        .read()
        .await
        .roll(list_name.as_str(), |items| {
            if let Err(e) = window
                .emit("wheel-list", items)
                .with_context(|| "Failed to emit 'wheel-list' event")
            {
                error!("{}", e);
            }
        })
        .await
        .with_context(|| "Failed to roll wheel")?;

    Ok(window
        .emit("stop", ())
        .with_context(|| "Failed to emit 'stop' signal")?)
}
