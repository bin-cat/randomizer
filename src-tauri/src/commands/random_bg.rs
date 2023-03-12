use anyhow::Context;

use crate::error::Result;

#[tauri::command]
pub fn random_bg(list_name: &str) -> Result<Option<(String, String)>> {
    if let Some(file_path) = randomizer_core::random_bg(list_name)
        .with_context(|| "Failed to select random background file")?
    {
        let mime_type = mime_guess::from_path(file_path.as_str()).first_or_octet_stream();

        return Ok(Some((file_path, mime_type.essence_str().to_string())));
    }

    Ok(None)
}
