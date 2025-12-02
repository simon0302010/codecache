use ratatui::{
    layout::Constraint::Percentage,
    prelude::*,
    widgets::{Block, Paragraph},
};
use tui_widget_list::{ListBuilder, ListState, ListView};

#[derive(Debug, Clone)]
pub struct CodeSnippet {
    title: String,
    text: String,
    text_style: Style,
    border_style: Style,
}

pub struct SnippetList<'a> {
    pub state: &'a mut ListState,
    pub items: Vec<CodeSnippet>,
}

impl CodeSnippet {
    pub fn new<T: Into<String>>(title: T, text: T) -> Self {
        Self {
            text: text.into(),
            text_style: Style::default(),
            border_style: Style::default(),
            title: title.into(),
        }
    }

    pub fn height(&self) -> u16 {
        let content_lines = self.text.lines().count() as u16;
        let border_height = 2;
        content_lines + border_height
    }
}

impl Widget for CodeSnippet {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [_, block_area, _] =
            Layout::horizontal([Percentage(25), Percentage(50), Percentage(25)]).areas(area);

        let block = Block::bordered()
            .title(self.title)
            .title_alignment(Alignment::Center)
            .border_style(self.border_style);

        let inner_area = block.inner(block_area);
        block.render(block_area, buf);

        Paragraph::new(self.text)
            .style(self.text_style)
            .render(inner_area, buf);
    }
}

impl<'a> ratatui::prelude::Widget for SnippetList<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let items = self.items;
        let item_count = items.len();

        let builder = ListBuilder::new(move |context| {
            let mut item = items[context.index].clone();

            if context.index % 2 == 0 {
                item.text_style = Style::default().bg(Color::Rgb(60, 56, 54));
            } else {
                item.text_style = Style::default().bg(Color::Rgb(80, 73, 69));
            }

            // border color stays the same
            item.border_style = Style::default().fg(Color::Rgb(124, 111, 100));

            if context.is_selected {
                item.text_style = Style::default()
                    .bg(Color::Rgb(254, 128, 25))
                    .fg(Color::Rgb(28, 28, 32));
                item.border_style = Style::default().fg(Color::Rgb(250, 189, 47));
            }

            let item_height = item.height();

            (item, item_height)
        });

        let list = ListView::new(builder, item_count).scroll_padding(4);
        list.render(area, buf, self.state);
    }
}
