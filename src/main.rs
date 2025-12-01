use crossterm::event::{self, Event, KeyCode, KeyEvent};
use ratatui::{Frame, text::Text};

fn main() {
    let mut terminal = ratatui::init();
    loop {
        terminal.draw(draw).expect("failed to draw frame");
        if matches!(
            event::read().expect("failed to read event"),
            Event::Key(KeyEvent { code: KeyCode::Char('q'), .. })
        ) {
            break;
        }
    }
    ratatui::restore();
}

fn draw(frame: &mut Frame) {
    let text = Text::raw("Hello World!");
    frame.render_widget(text, frame.area());
}