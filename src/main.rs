use std::{fs, path::PathBuf};

mod app;

fn main() {
    let file_path = get_data_path();
    let snippets_file = fs::read_to_string(&file_path).unwrap_or("[]".to_string());
    let snippets: Vec<app::SaveSnippet> =
        serde_json::from_str(&snippets_file).unwrap_or_else(|_| Vec::new());

    // initialize app
    let mut codecache = app::CodeCache::new(snippets);

    let snippets = codecache.run();

    // save back to file
    let snippets_str =
        serde_json::to_string_pretty(&snippets).expect("failed to save snippets to file");
    fs::write(&file_path, snippets_str).expect("failed to save snippets to file");
}

fn get_data_path() -> PathBuf {
    let mut path = dirs::data_local_dir().unwrap_or_else(|| {
        #[cfg(target_os = "windows")]
        return PathBuf::from(std::env::var("APPDATA").unwrap_or_else(|_| ".".to_string()));
        #[cfg(not(target_os = "windows"))]
        return PathBuf::from("~/.local/share");
    });

    path.push("codecache");
    fs::create_dir_all(&path).expect("failed to create data directory");

    path.push("snippets.json");
    path
}
