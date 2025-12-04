use std::fs;

mod app;

const FILE_PATH: &str = "snippets.json";

fn main() {
    let snippets_file = fs::read_to_string(FILE_PATH).unwrap_or("[]".to_string());
    let snippets: Vec<app::SaveSnippet> =
        serde_json::from_str(&snippets_file).unwrap_or_else(|_| Vec::new());

    // initialize app
    let mut codecache = app::CodeCache::new(snippets);

    let snippets = codecache.run();

    // save back to file
    let snippets_str =
        serde_json::to_string_pretty(&snippets).expect("failed to save snippets to file");
    fs::write(FILE_PATH, snippets_str).expect("failed to save snippets to file");
}
