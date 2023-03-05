use std::collections::HashSet;

use anyhow::Context;
use once_cell::sync::Lazy;
use rand::seq::IteratorRandom;
use tauri::Window;

use crate::{
    error::Result,
    func::{data_files_for_list, dir_entry_extension},
    AppState, DATA_PATH,
};

static BG_EXTENSIONS: Lazy<HashSet<&'static str>> = Lazy::new(|| {
    HashSet::from([
        "bmp", "gif", "jpg", "jpeg", "png", "svg", "webp", "mp4", "webm",
    ])
});

#[tauri::command]
pub fn random_bg(
    list_name: &str,
    _state: tauri::State<'_, AppState>,
    _window: Window,
) -> Result<Option<(String, String)>> {
    let mut bg_dir = DATA_PATH.clone();
    bg_dir.push("bg");
    bg_dir.push(list_name);

    let mut rng = rand::thread_rng();

    for dir in data_files_for_list("bg", list_name, |file| {
        BG_EXTENSIONS.contains(&dir_entry_extension(file).as_str())
    }) {
        if let Some(entry) = dir.choose(&mut rng) {
            let file_path = entry.path().to_path_buf();
            let file_path = file_path
                .strip_prefix(DATA_PATH.clone())
                .with_context(|| "Failed to strip BG path prefix")?;

            let mime_type = mime_guess::from_path(file_path).first_or_octet_stream();

            return Ok(Some((
                file_path.to_string_lossy().to_string(),
                mime_type.essence_str().to_string(),
            )));
        }
    }

    Ok(None)
}
