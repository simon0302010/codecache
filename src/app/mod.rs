use crossterm::event::{self, Event, KeyCode, KeyEvent};
use ratatui::{Frame, text::Text};

pub struct CodeCache {}

impl CodeCache {}

fn draw(frame: &mut Frame) {
    let text = Text::raw("Hello World!");
    frame.render_widget(text, frame.area());
}
