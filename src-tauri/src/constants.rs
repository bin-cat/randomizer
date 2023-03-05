use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use directories::BaseDirs;
use once_cell::sync::{Lazy, OnceCell};
use tokio::sync::RwLock;

use crate::audio_player::Player;

const BUNDLE_IDENTIFIER: &'static str = "ru.oyashiro.randomizer";
const DATA_DIR: &'static str = "data";

pub static APP_PATH: Lazy<PathBuf> = Lazy::new(|| {
    std::env::current_exe()
        .ok()
        .and_then(|x| x.parent().map(Path::to_path_buf))
        .unwrap_or_default()
});

pub static CONFIG_PATH: Lazy<PathBuf> = Lazy::new(|| match BaseDirs::new() {
    Some(value) => {
        let mut path = value.config_dir().to_path_buf();
        path.push(BUNDLE_IDENTIFIER);
        path
    }
    None => APP_PATH.clone(),
});

pub static DATA_PATH: Lazy<PathBuf> = Lazy::new(|| {
    let mut path = APP_PATH.clone();
    path.push(DATA_DIR);
    path
});

pub static PLAYER: Lazy<RwLock<Player>> = Lazy::new(|| RwLock::new(Player::new()));

pub static ROLL_SOUNDS: OnceCell<HashMap<String, Vec<PathBuf>>> = OnceCell::new();
pub static STOP_SOUNDS: OnceCell<HashMap<String, Vec<PathBuf>>> = OnceCell::new();
