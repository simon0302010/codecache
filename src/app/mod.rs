use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{Frame, text::Text};

pub struct CodeCache {}

impl CodeCache {
    pub fn new() -> Self {
        CodeCache {}
    }

    pub fn run(&mut self) {
        let mut terminal = ratatui::init();
        loop {
            terminal.draw(|frame| self.draw(frame)).expect("failed to draw frame");
            match event::read().expect("failed to read event") {
                Event::Key(key) if key.kind == KeyEventKind::Press => match key.code {
                    KeyCode::Char('q') => break,
                    _ => {}
                } ,
                _ => {}
            }
        }
        ratatui::restore();
    }

    fn draw(&self, frame: &mut Frame) {
        let text = Text::raw("Hello World!");
        frame.render_widget(text, frame.area());
    }
}
