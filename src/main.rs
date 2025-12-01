mod app;

fn main() {
    let mut codecache = app::CodeCache::new();

    codecache.run();
}
