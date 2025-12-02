use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    Frame,
    layout::{
        Constraint::{Length, Min},
        Layout, Margin,
    },
    style::{Style, Stylize},
    widgets::{Block, Borders, List, Scrollbar, ScrollbarOrientation, ScrollbarState},
};

pub struct CodeCache {
    running: bool,
    scroll_position: usize,
}

impl CodeCache {
    pub fn new() -> Self {
        CodeCache {
            running: true,
            scroll_position: 0,
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

    fn draw(&self, frame: &mut Frame) {
        let vertical = Layout::vertical([Length(1), Min(0), Length(1)]);
        let [title_area, main_area, status_area] = vertical.areas(frame.area());
        // let horizontal = Layout::horizontal([Fill(1); 2]);
        // let [left_area, right_area] = horizontal.areas(main_area);

        // define code snippets
        let snippets = (1..=100)
            .map(|i| format!("Item {}", i))
            .collect::<Vec<String>>();

        let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
            .begin_symbol(Some("↑"))
            .end_symbol(Some("↓"));

        let mut scrollbar_state =
            ScrollbarState::new(snippets.len()).position(self.scroll_position);

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
            List::new(snippets).block(
                Block::new()
                    .borders(Borders::TOP | Borders::BOTTOM)
                    .title("Snippets")
                    .title_alignment(ratatui::layout::Alignment::Center),
            ),
            main_area,
        );
        frame.render_stateful_widget(scrollbar, main_area.inner(Margin { vertical: 1, horizontal: 0 }), &mut scrollbar_state);
    }

    fn handle_events(&mut self) {
        match event::read().expect("failed to read event") {
            Event::Key(key) if key.kind == KeyEventKind::Press => match key.code {
                KeyCode::Char('q') => self.running = false,
                _ => {}
            },
            _ => {}
        }
    }
}
