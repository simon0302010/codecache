use ratatui::{
    layout::Constraint::{Fill, Length, Min},
    prelude::*,
    widgets::{Block, Paragraph},
};
use tui_widget_list::{ListBuilder, ListState, ListView};

use crate::app::highlight::Highlighter;

#[derive(Debug, Clone)]
pub struct CodeSnippet {
    title: String,
    text: String,
    code: String,
    text_style: Style,
    code_style: Style,
    code_frame_style: Style,
    border_style: Style,
    highlighted_code: Option<Text<'static>>,
}

pub struct SnippetList<'a> {
    pub state: &'a mut ListState,
    pub items: Vec<CodeSnippet>,
    pub highlighter: &'a Highlighter,
}

impl CodeSnippet {
    pub fn new<T: Into<String>>(title: T, text: T, code: T) -> Self {
        Self {
            text: text.into(),
            code: code.into(),
            text_style: Style::default(),
            border_style: Style::default(),
            code_style: Style::default(),
            code_frame_style: Style::default(),
            title: title.into(),
            highlighted_code: None,
        }
    }

    pub fn height(&self) -> u16 {
        let description_lines = self.text.lines().count() as u16;
        let code_lines = self.code.lines().count() as u16;
        let border_height = 4;
        description_lines + code_lines + border_height
    }
}

impl Widget for CodeSnippet {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // Center the card horizontally
        let [_, block_area, _] = Layout::horizontal([Fill(1), Min(70), Fill(1)]).areas(area);

        // Outer block with title
        let block = Block::bordered()
            .title(self.title)
            .title_alignment(Alignment::Center)
            .border_style(self.border_style);

        let inner_area = block.inner(block_area);
        block.render(block_area, buf);

        let desc_lines = self.text.lines().count().max(1) as u16;
        let code_lines = self.code.lines().count().max(1) as u16 + 2; // + 2 for border

        // Split inner area: description (1 line), code (flexible)
        let [desc_area, code_area] =
            Layout::vertical([Length(desc_lines), Length(code_lines)]).areas(inner_area);

        // Render description
        Paragraph::new(self.text)
            .style(self.text_style)
            .render(desc_area, buf);

        // Render code block
        let code_block = Block::bordered()
            .border_style(self.code_frame_style)
            .title("Placeholder for Programming Language")
            .title_alignment(Alignment::Center);
        let code_inner = code_block.inner(code_area);
        code_block.render(code_area, buf);

        let code_text = self
            .highlighted_code
            .unwrap_or_else(|| Text::raw(self.code));

        Paragraph::new(code_text)
            .style(self.code_style)
            .render(code_inner, buf);
    }
}

impl<'a> ratatui::prelude::Widget for SnippetList<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // pre highlight
        let items: Vec<CodeSnippet> = self
            .items
            .into_iter()
            .map(|mut item| {
                item.highlighted_code = Some(self.highlighter.highlight(&item.code));
                item
            })
            .collect();

        let item_count = items.len();

        let builder = ListBuilder::new(move |context| {
            let mut item = items[context.index].clone();

            item.text_style = Style::default().fg(Color::Rgb(120, 112, 108));
            item.border_style = Style::default().fg(Color::Rgb(124, 111, 100));
            item.code_frame_style = Style::default().fg(Color::Rgb(124, 111, 100));

            if context.is_selected {
                item.text_style = Style::default()
                    .bg(Color::Rgb(254, 128, 25))
                    .fg(Color::Rgb(28, 28, 32));
                item.border_style = Style::default().fg(Color::Rgb(250, 189, 47));
                item.code_frame_style = Style::default().fg(Color::Rgb(180, 119, 0));
            }

            let item_height = item.height();

            (item, item_height)
        });

        let list = ListView::new(builder, item_count)
            .scroll_padding(4)
            .infinite_scrolling(false);
        list.render(area, buf, self.state);
    }
}
