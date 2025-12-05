use ratatui::text::Text;
use syntect::easy::HighlightLines;
use syntect::highlighting::ThemeSet;
use syntect::parsing::SyntaxSet;
use syntect::util::{LinesWithEndings, as_24_bit_terminal_escaped};

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

    pub fn highlight(&self, code: &str, lang: &str) -> Result<Text<'static>, Text<'static>> {
        /*let syntax = self
        .ps
        .find_syntax_by_first_line(code.as_str())
        .unwrap_or_else(|| self.ps.find_syntax_plain_text());*/

        let syntax = match self.ps.find_syntax_by_extension(lang) {
            Some(s) => s,
            None => {
                return Err(Text::raw(code.to_string()));
            }
        };

        let mut h = HighlightLines::new(syntax, &self.ts.themes["base16-eighties.dark"]);

        let mut final_str = String::new();
        for line in LinesWithEndings::from(&code) {
            let ranges = h.highlight_line(line, &self.ps).unwrap();
            let escaped = as_24_bit_terminal_escaped(&ranges[..], false);
            final_str.push_str(&escaped);
        }

        let text = ansi_to_tui::IntoText::into_text(&final_str)
            .unwrap_or_else(|_| Text::raw(code.to_string()));
        Ok(text)
    }
}
