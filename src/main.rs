use std::fs;

mod app;

fn main() {
    let snippets_file = fs::read_to_string("snippets.json").unwrap_or("[]".to_string());
    let snippets: Vec<app::SaveSnippet> =
        serde_json::from_str(&snippets_file).unwrap_or_else(|_| Vec::new());

    let mut codecache = app::CodeCache::new(snippets);

    codecache.run();
}
