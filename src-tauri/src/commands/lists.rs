use walkdir::WalkDir;

use crate::DATA_PATH;

#[tauri::command]
pub fn lists() -> Vec<String> {
    let mut lists_dir = DATA_PATH.clone();
    lists_dir.push("lists");

    let mut lists = vec![];

    for file_path in WalkDir::new(lists_dir)
        .follow_links(false)
        .max_depth(1)
        .into_iter()
        .filter_map(std::result::Result::ok)
        .filter(|file| file.path().extension().map_or(false, |ext| ext == "txt"))
    {
        if let Some(stem) = file_path.path().file_stem() {
            lists.push(stem.to_string_lossy().to_string());
        }
    }

    lists
}
