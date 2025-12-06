/// function to get lang name from file extension
pub fn get_lang(ext: &str) -> String {
    match ext {
        "py" => "Python",
        "go" => "Go",
        "java" => "Java",
        "c" => "C",
        "cpp" => "C++",
        "html" => "HTML",
        "css" => "CSS",
        "cs" => "C#",
        "yaml" => "YAML",
        "json" => "JSON",
        "toml" => "TOML",
        "sh" => "Bash",
        "md" => "Markdown",
        "rs" => "Rust",
        "ts" => "TypeScript",
        "js" => "JavaScript",
        _ => "Unknown Language",
    }
    .to_string()
}
