mod audio_player;
mod config;
mod constants;
mod error;
mod func;
mod randomizer;

use std::{collections::HashSet, path::PathBuf};

use constants::{DATA_PATH, LIST_EXTENSION};
use func::{data_files_for_list, dir_entry_extension};
use once_cell::sync::Lazy;
use rand::seq::IteratorRandom;
use walkdir::WalkDir;

use crate::audio_player::Player;

pub use crate::{
    config::Config,
    error::{Error, Result},
    randomizer::Randomizer,
};

static BG_EXTENSIONS: Lazy<HashSet<&'static str>> = Lazy::new(|| {
    HashSet::from([
        "bmp", "gif", "jpg", "jpeg", "png", "svg", "webp", "mp4", "webm",
    ])
});

pub fn data_path() -> PathBuf {
    DATA_PATH.clone()
}

pub fn list_audio_devices() -> Result<Vec<(String, String)>> {
    Player::list_devices()
}

pub fn lists() -> Vec<String> {
    let mut lists_dir = DATA_PATH.clone();
    lists_dir.push("lists");

    let mut lists = vec![];

    for file_path in WalkDir::new(lists_dir)
        .follow_links(false)
        .max_depth(1)
        .into_iter()
        .filter_map(std::result::Result::ok)
        .filter(|file| {
            file.path()
                .extension()
                .map_or(false, |ext| ext == LIST_EXTENSION)
        })
    {
        if let Some(stem) = file_path.path().file_stem() {
            lists.push(stem.to_string_lossy().to_string());
        }
    }

    lists
}

pub fn random_bg(list_name: &str) -> Result<Option<String>> {
    let mut bg_dir = DATA_PATH.clone();
    bg_dir.push("bg");
    bg_dir.push(list_name);

    let mut rng = rand::thread_rng();

    for dir in data_files_for_list("bg", list_name, |file| {
        BG_EXTENSIONS.contains(&dir_entry_extension(file).as_str())
    }) {
        if let Some(entry) = dir.choose(&mut rng) {
            let file_path = entry.path().to_path_buf();
            let file_path = file_path.strip_prefix(DATA_PATH.clone())?;

            return Ok(Some(file_path.to_string_lossy().to_string()));
        }
    }

    Ok(None)
}
