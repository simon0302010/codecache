use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    Frame,
    layout::{
        Constraint::{Fill, Length, Min},
        Layout,
    },
    style::{Style, Stylize},
    widgets::Block,
};

pub struct CodeCache {
    running: bool,
}

impl CodeCache {
    pub fn new() -> Self {
        CodeCache { running: true }
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
        let horizontal = Layout::horizontal([Fill(1); 2]);
        let [left_area, right_area] = horizontal.areas(main_area);

        frame.render_widget(
            Block::new()
                .title("CodeCache")
                .title_alignment(ratatui::layout::Alignment::Center)
                .title_style(Style::new().red().bold()),
            title_area,
        );
        frame.render_widget(
            Block::new()
                .title("0 Snippets saved")
                .title_alignment(ratatui::layout::Alignment::Center),
            status_area,
        );
        frame.render_widget(Block::bordered().title("Left"), left_area);
        frame.render_widget(Block::bordered().title("Right"), right_area);
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
