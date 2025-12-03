use ansi_to_tui::IntoText;
use ratatui::prelude::Text;
use syntect::easy::HighlightLines;
use syntect::highlighting::ThemeSet;
use syntect::parsing::SyntaxSet;
use syntect::util::{LinesWithEndings, as_24_bit_terminal_escaped};

use crate::app::language;

pub struct Highlighter {
    pub ps: SyntaxSet,
    pub ts: ThemeSet,
}

impl Highlighter {
    pub fn new() -> Self {
        Self {
            ps: SyntaxSet::load_defaults_newlines(),
            ts: ThemeSet::load_defaults(),
        }
    }

    pub fn highlight(&self, code: &str) -> Text<'static> {
        let detected_language = language::detect_language(code);
        let extension = detected_language.extension();

        let syntax = self
            .ps
            .find_syntax_by_extension(extension)
            .unwrap_or_else(|| self.ps.find_syntax_plain_text());

        let mut h = HighlightLines::new(syntax, &self.ts.themes["base16-eighties.dark"]);

        let mut final_str = String::new();
        for line in LinesWithEndings::from(code) {
            let ranges = h.highlight_line(line, &self.ps).unwrap();
            let escaped = as_24_bit_terminal_escaped(&ranges[..], false);
            final_str.push_str(&escaped);
        }

        // convert string to owned Text<'static>
        match final_str.into_text() {
            Ok(text) => {
                // convert Text<'_> to Text<'static> by collecting owned lines
                let lines: Vec<ratatui::prelude::Line<'static>> = text
                    .lines
                    .into_iter()
                    .map(|line| {
                        let spans: Vec<ratatui::prelude::Span<'static>> = line
                            .spans
                            .into_iter()
                            .map(|span| {
                                ratatui::prelude::Span::styled(
                                    span.content.into_owned(),
                                    span.style,
                                )
                            })
                            .collect();
                        ratatui::prelude::Line::from(spans)
                    })
                    .collect();
                Text::from(lines)
            }
            Err(_) => Text::raw(code.to_string()),
        }
    }
}
