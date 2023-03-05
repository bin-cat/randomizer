use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
    path::{Path, PathBuf},
};

use once_cell::sync::Lazy;
use walkdir::{DirEntry, WalkDir};

use crate::DATA_PATH;

static SOUND_EXTENSIONS: Lazy<HashSet<&'static str>> = Lazy::new(|| {
    HashSet::from([
        "aiff", "aif", "aifc", "flac", "mp3", "mp4", "oga", "ogg", "wav", "wma",
    ])
});

pub fn lines_from_file(path: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(path).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not read line").trim().to_string())
        .filter(|x| !x.starts_with('#'))
        .collect()
}

pub fn data_files_for_list(
    base_dir: impl AsRef<Path>,
    list_name: &str,
    filter: fn(&DirEntry) -> bool,
) -> Vec<impl Iterator<Item = DirEntry>> {
    let mut common_dir = DATA_PATH.clone();
    common_dir.push(base_dir);

    let mut list_dir = common_dir.clone();
    list_dir.push(list_name);

    vec![
        files_from_dir(&list_dir, filter),
        files_from_dir(&common_dir, filter),
    ]
}

pub fn dir_entry_extension(entry: &DirEntry) -> String {
    entry
        .path()
        .extension()
        .unwrap_or_default()
        .to_string_lossy()
        .to_lowercase()
}

pub fn files_from_dir(
    dir: &PathBuf,
    filter: fn(&DirEntry) -> bool,
) -> impl Iterator<Item = DirEntry> {
    WalkDir::new(dir)
        .follow_links(false)
        .max_depth(1)
        .into_iter()
        .filter_map(std::result::Result::ok)
        .filter(move |entry| entry.path().is_file() && filter(entry))
}

pub fn load_sound_lists(category: &str) -> HashMap<String, Vec<PathBuf>> {
    let mut sounds_dir = DATA_PATH.clone();
    sounds_dir.push("sounds");
    sounds_dir.push(category);

    let mut result = HashMap::from([(String::new(), load_tracks(&sounds_dir))]);

    for entry in WalkDir::new(&sounds_dir)
        .follow_links(false)
        .max_depth(1)
        .into_iter()
        .filter_map(std::result::Result::ok)
        .filter(|entry| entry.path().is_dir() && entry.path() != sounds_dir.as_path())
    {
        result.insert(
            entry.file_name().to_string_lossy().to_string(),
            load_tracks(&entry.path().to_path_buf()),
        );
    }

    result
}

fn load_tracks(base_dir: &PathBuf) -> Vec<PathBuf> {
    let mut result: Vec<PathBuf> = files_from_dir(base_dir, |file| {
        SOUND_EXTENSIONS.contains(&dir_entry_extension(file).as_str())
    })
    .map(|x| x.path().to_path_buf())
    .collect();

    let mut playlist_path = base_dir.clone();
    playlist_path.push(format!("!playlist.m3u8"));
    if playlist_path.is_file() {
        result.extend(
            lines_from_file(playlist_path)
                .iter()
                .map(|x| PathBuf::from(x))
                .filter(|x| x.is_file()),
        );
    }

    result
}
