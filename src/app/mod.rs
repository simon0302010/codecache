use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{Frame, text::Text};

pub struct CodeCache {
    running: bool
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
        let text = Text::raw("Hello World!");
        frame.render_widget(text, frame.area());
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
