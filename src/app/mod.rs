use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    layout::Constraint::{Length, Min, Percentage},
    prelude::*,
    widgets::{Block, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState},
};
use tui_widget_list::{ListBuilder, ListState, ListView};

pub struct CodeCache {
    running: bool,
    scroll_state: ScrollbarState,
    list_state: ListState,
}

#[derive(Debug, Clone)]
struct CodeSnippet {
    title: String,
    text: String,
    style: Style,
}

struct SnippetList<'a> {
    state: &'a mut ListState,
    items: Vec<CodeSnippet>,
}

impl CodeSnippet {
    pub fn new<T: Into<String>>(title: T, text: T) -> Self {
        Self {
            text: text.into(),
            style: Style::default(),
            title: title.into(),
        }
    }
}

impl Widget for CodeSnippet {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [_, block_area, _] =
            Layout::horizontal([Percentage(25), Percentage(50), Percentage(25)]).areas(area);

        let block = Block::bordered()
            .title(self.title)
            .title_alignment(Alignment::Center);

        let inner_area = block.inner(block_area);
        block.render(block_area, buf);

        Paragraph::new(self.text)
            .style(self.style)
            .render(inner_area, buf);
    }
}

impl CodeCache {
    pub fn new() -> Self {
        CodeCache {
            running: true,
            scroll_state: ScrollbarState::default(),
            list_state: ListState::default(),
        }
    }

    pub fn run(&mut self) {
        let mut terminal = ratatui::init();
        while self.running {
            terminal
                .draw(|frame| self.draw(frame))
                .expect("failed to draw frame");
            self.handle_events();
        }
        ratatui::restore();
    }

    fn draw(&mut self, frame: &mut Frame) {
        let vertical = Layout::vertical([Length(1), Min(0), Length(1)]);
        let [title_area, main_area, status_area] = vertical.areas(frame.area());

        // define code snippets
        let snippets: Vec<CodeSnippet> = (1..=100)
            .map(|i| {
                CodeSnippet::new(
                    format!("Item {}", i),
                    "Content Line 1\nContent Line 2".to_string(),
                )
            })
            .collect();

        let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
            .begin_symbol(Some("↑"))
            .end_symbol(Some("↓"));

        let selected = self.list_state.selected.unwrap_or(0);
        self.scroll_state = ScrollbarState::new(snippets.len()).position(selected);

        frame.render_widget(
            Block::new()
                .title(format!("CodeCache v{}", env!("CARGO_PKG_VERSION")))
                .title_alignment(ratatui::layout::Alignment::Center)
                .title_style(Style::new().red().bold()),
            title_area,
        );
        frame.render_widget(
            Block::new()
                .title(format!("{} snippets in storage", snippets.len()))
                .title_alignment(ratatui::layout::Alignment::Center),
            status_area,
        );
        frame.render_widget(
            SnippetList {
                state: &mut self.list_state,
                items: snippets,
            },
            main_area,
        );
        frame.render_stateful_widget(
            scrollbar,
            main_area.inner(Margin {
                vertical: 1,
                horizontal: 0,
            }),
            &mut self.scroll_state,
        );
    }

    fn handle_events(&mut self) {
        match event::read().expect("failed to read event") {
            Event::Key(key) if key.kind == KeyEventKind::Press => match key.code {
                KeyCode::Char('q') => self.running = false,
                KeyCode::Down => {
                    self.list_state.next();
                    self.scroll_state.next();
                }
                KeyCode::Up => {
                    self.list_state.previous();
                    self.scroll_state.prev();
                }
                _ => {}
            },
            _ => {}
        }
    }
}

impl<'a> ratatui::prelude::Widget for SnippetList<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let items = self.items;
        let item_count = items.len();

        let builder = ListBuilder::new(move |context| {
            let mut item = items[context.index].clone();

            if context.index % 2 == 0 {
                item.style = Style::default().bg(Color::Rgb(28, 28, 32));
            } else {
                item.style = Style::default().bg(Color::Rgb(24, 24, 28));
            }

            if context.is_selected {
                item.style = Style::default()
                    .bg(Color::Rgb(255, 153, 0))
                    .fg(Color::Rgb(28, 28, 32));
            }

            (item, 10)
        });

        let list = ListView::new(builder, item_count);
        list.render(area, buf, self.state);
    }
}
