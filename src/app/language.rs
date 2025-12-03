use tree_sitter::Parser;

#[derive(Debug, Clone, PartialEq)]
pub enum Language {
    Python,
    Go,
    Java,
    C,
    Cpp,
    Html,
    Css,
    CSharp,
    Yaml,
    Json,
    Toml,
    Bash,
    Markdown,
    Rust,
    TypeScript,
    JavaScript,
    Unknown,
}

impl Language {
    pub fn extension(&self) -> &str {
        match self {
            Language::Python => "py",
            Language::Go => "go",
            Language::Java => "java",
            Language::C => "c",
            Language::Cpp => "cpp",
            Language::Html => "html",
            Language::Css => "css",
            Language::CSharp => "cs",
            Language::Yaml => "yaml",
            Language::Json => "json",
            Language::Toml => "toml",
            Language::Bash => "sh",
            Language::Markdown => "md",
            Language::Rust => "rs",
            Language::TypeScript => "ts",
            Language::JavaScript => "js",
            Language::Unknown => "txt",
        }
    }

    pub fn name(&self) -> &str {
        match self {
            Language::Python => "Python",
            Language::Go => "Go",
            Language::Java => "Java",
            Language::C => "C",
            Language::Cpp => "C++",
            Language::Html => "HTML",
            Language::Css => "CSS",
            Language::CSharp => "C#",
            Language::Yaml => "YAML",
            Language::Json => "JSON",
            Language::Toml => "TOML",
            Language::Bash => "Bash",
            Language::Markdown => "Markdown",
            Language::Rust => "Rust",
            Language::TypeScript => "TypeScript",
            Language::JavaScript => "JavaScript",
            Language::Unknown => "Plain Text",
        }
    }
}

pub fn detect_language(code: &str) -> Language {
    let parsers: Vec<(Language, fn() -> tree_sitter::Language)> = vec![
        (Language::Python, || tree_sitter_python::LANGUAGE.into()),
        (Language::Go, || tree_sitter_go::LANGUAGE.into()),
        (Language::Java, || tree_sitter_java::LANGUAGE.into()),
        (Language::C, || tree_sitter_c::LANGUAGE.into()),
        (Language::Cpp, || tree_sitter_cpp::LANGUAGE.into()),
        (Language::Html, || tree_sitter_html::LANGUAGE.into()),
        (Language::Css, || tree_sitter_css::LANGUAGE.into()),
        (Language::CSharp, || tree_sitter_c_sharp::LANGUAGE.into()),
        (Language::Yaml, || tree_sitter_yaml::LANGUAGE.into()),
        (Language::Json, || tree_sitter_json::LANGUAGE.into()),
        (Language::Toml, || tree_sitter_toml_ng::LANGUAGE.into()),
        (Language::Bash, || tree_sitter_bash::LANGUAGE.into()),
        (Language::Markdown, || tree_sitter_md::LANGUAGE.into()),
        (Language::Rust, || tree_sitter_rust::LANGUAGE.into()),
        (Language::TypeScript, || {
            tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into()
        }),
        (Language::JavaScript, || {
            tree_sitter_javascript::LANGUAGE.into()
        }),
    ];

    let mut best_match: Option<(Language, usize)> = None;

    for (lang, get_language) in parsers {
        let mut parser = Parser::new();
        let language = get_language();
        if parser.set_language(&language).is_err() {
            continue;
        }

        if let Some(tree) = parser.parse(code, None) {
            let root = tree.root_node();

            let error_count = count_errors(&root);
            let node_count = count_nodes(&root);

            if error_count == 0 && node_count > 1 {
                return lang;
            }

            let score = node_count.saturating_sub(error_count * 10);
            if let Some((_, best_score)) = &best_match {
                if score > *best_score {
                    best_match = Some((lang, score));
                }
            } else if score > 0 {
                best_match = Some((lang, score));
            }
        }
    }

    best_match
        .map(|(lang, _)| lang)
        .unwrap_or(Language::Unknown)
}

fn count_errors(node: &tree_sitter::Node) -> usize {
    let mut count = if node.is_error() || node.is_missing() {
        1
    } else {
        0
    };

    let mut cursor = node.walk();

    for child in node.children(&mut cursor) {
        count += count_errors(&child);
    }
    count
}

fn count_nodes(node: &tree_sitter::Node) -> usize {
    let mut count = 1;
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        count += count_nodes(&child);
    }
    count
}
