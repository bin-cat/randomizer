use std::path::{Path, PathBuf};

use directories::BaseDirs;
use once_cell::sync::Lazy;

const BUNDLE_IDENTIFIER: &str = "ru.oyashiro.randomizer";
const DATA_DIR: &str = "data";

pub const LIST_EXTENSION: &str = "txt";

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
