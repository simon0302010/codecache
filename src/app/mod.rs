mod codesnippet;
mod highlight;
mod language;

// export for main.rs
pub use codesnippet::{SaveSnippet, SnippetList};

use codesnippet::CodeSnippet;
use highlight::Highlighter;
use tui_dialog::{Dialog, centered_rect};
use tui_popup::Popup;

use std::time::{Duration, Instant};

use arboard::Clipboard;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    layout::Constraint::{Length, Min},
    prelude::*,
    widgets::{Block, Scrollbar, ScrollbarOrientation, ScrollbarState},
};
use tui_widget_list::ListState;

pub struct CodeCache {
    running: bool,
    scroll_state: ScrollbarState,
    list_state: ListState,
    last_move: Instant,
    last_move_direction: String,
    highlighter: Highlighter,
    snippets: Vec<CodeSnippet>,
    save_snippets: Vec<SaveSnippet>,
    clipboard: Clipboard,
    dialog: Dialog,
    dialog_field: String,
    edit_idx: usize,
    notification: Option<(String, Instant)>,
}

impl CodeCache {
    pub fn new(snippets: Vec<SaveSnippet>) -> Self {
        CodeCache {
            running: true,
            scroll_state: ScrollbarState::default(),
            list_state: ListState::default(),
            last_move: Instant::now() - Duration::from_secs(1),
            last_move_direction: String::new(),
            highlighter: Highlighter::new(),
            snippets: convert_snippets(&snippets),
            save_snippets: snippets,
            clipboard: Clipboard::new().expect("failed to initialize clipboard"),
            dialog: new_dialog(),
            dialog_field: String::new(),
            notification: None,
            edit_idx: 0,
        }
    }

    pub fn notify(&mut self, msg: impl Into<String>) {
        self.notification = Some((msg.into(), Instant::now()));
    }

    pub fn run(&mut self) -> Vec<SaveSnippet> {
        let mut terminal = ratatui::init();
        while self.running {
            terminal
                .draw(|frame| self.draw(frame))
                .expect("failed to draw frame");
            self.handle_events();
        }
        ratatui::restore();
        self.save_snippets.clone()
    }

    fn draw(&mut self, frame: &mut Frame) {
        let vertical = Layout::vertical([Length(1), Min(0), Length(1)]);
        let [title_area, main_area, status_area] = vertical.areas(frame.area());

        // focused styles
        let general_scrollbar_style = Style::default().fg(Color::Rgb(102, 92, 84));
        let focused_scrollbal_style = Style::default().fg(Color::Rgb(250, 130, 28));

        let is_active = self.last_move.elapsed().as_millis() <= 500;

        let scrollbar_style = if is_active {
            focused_scrollbal_style
        } else {
            general_scrollbar_style
        };

        let arrow_up_style = if is_active && self.last_move_direction.as_str() == "up" {
            focused_scrollbal_style
        } else {
            general_scrollbar_style
        };

        let arrow_down_style = if is_active && self.last_move_direction.as_str() == "down" {
            focused_scrollbal_style
        } else {
            general_scrollbar_style
        };

        let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
            .begin_symbol(Some("▄"))
            .end_symbol(Some("▀"))
            .track_symbol(Some("┃"))
            .thumb_style(scrollbar_style)
            .track_style(general_scrollbar_style)
            .begin_style(arrow_up_style)
            .end_style(arrow_down_style);

        let selected = self.list_state.selected.unwrap_or(0);
        self.scroll_state = ScrollbarState::new(self.snippets.len()).position(selected);

        frame.render_widget(
            Block::new()
                .title(format!("CodeCache v{}", env!("CARGO_PKG_VERSION")))
                .title_alignment(ratatui::layout::Alignment::Center)
                .title_style(Style::new().fg(Color::Rgb(251, 73, 52)).bold()),
            title_area,
        );
        frame.render_widget(
            Block::new()
                .title(format!(
                    "{} snippet(s) stored ({} lines) - press v to paste from clipboard, d to delete selected, c to copy selected, q to quit, e to edit",
                    self.snippets.len(),
                    self.save_snippets.iter().map(|s| s.code.lines().count()).sum::<usize>()
                ))
                .title_alignment(ratatui::layout::Alignment::Center)
                .title_style(Style::default().fg(Color::Cyan)),
            status_area,
        );
        frame.render_widget(
            SnippetList {
                state: &mut self.list_state,
                items: self.snippets.clone(),
                highlighter: &self.highlighter,
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

        if self.dialog.open {
            let dialog_area = centered_rect(frame.area(), 60, 10, 0, 0);
            frame.render_widget(self.dialog.clone(), dialog_area);
        } else {
            // clear area if its closed
            let dialog_area = centered_rect(frame.area(), 60, 10, 0, 0);
            frame.render_widget(Block::new(), dialog_area);
        }

        if let Some((msg, shown_at)) = &self.notification {
            if shown_at.elapsed() < Duration::from_secs(2) {
                frame.render_widget(
                    &Popup::new(msg.as_str())
                        .title("Info")
                        .style(Style::default().fg(Color::LightBlue)),
                    frame.area(),
                );
            }
        }
    }

    fn handle_events(&mut self) {
        if event::poll(Duration::from_millis(16)).expect("failed to poll event") {
            match event::read().expect("failed to read event") {
                Event::Key(key) if key.kind == KeyEventKind::Press => {
                    if self.dialog.open {
                        self.dialog.key_action(&key.code);
                        if self.dialog.submitted {
                            let input = self.dialog.submitted_input.clone();
                            if !input.is_empty() {
                                if self.dialog_field == "title" {
                                    if let Some(item) = self.save_snippets.get_mut(self.edit_idx) {
                                        item.title = input;
                                    }
                                } else if self.dialog_field == "desc" {
                                    if let Some(item) = self.save_snippets.get_mut(self.edit_idx) {
                                        item.desc = input;
                                    }
                                } else if self.dialog_field == "lang" {
                                    if let Some(item) = self.save_snippets.get_mut(self.edit_idx) {
                                        item.lang = input;
                                    }
                                }
                            }
                            self.snippets = convert_snippets(&self.save_snippets);
                            self.dialog = new_dialog();
                            if self.dialog_field == "title" {
                                self.dialog_field = "desc".to_string();
                                self.dialog.open = true;
                                self.dialog = self.dialog.title_top("Enter Description");
                            } else if self.dialog_field == "desc" {
                                self.dialog_field = "lang".to_string();
                                self.dialog.open = true;
                                self.dialog = self.dialog.title_top(
                                    "Enter programming language extension (\"rs\" for rust)",
                                );
                            } else {
                                self.dialog_field = String::new();
                            }
                        }
                    } else {
                        match key.code {
                            KeyCode::Char('q') | KeyCode::Char('Q') => self.running = false,
                            KeyCode::Down | KeyCode::PageDown => {
                                self.list_state.next();
                                self.scroll_state.next();
                                self.last_move = Instant::now();
                                self.last_move_direction = "down".to_string();
                            }
                            KeyCode::Up | KeyCode::PageUp => {
                                self.list_state.previous();
                                self.scroll_state.prev();
                                self.last_move = Instant::now();
                                self.last_move_direction = "up".to_string();
                            }
                            KeyCode::Char('v') | KeyCode::Char('V') => {
                                match self.clipboard.get_text() {
                                    Ok(clipboard_text) => {
                                        // clean up
                                        if clipboard_text
                                            .find(|c: char| {
                                                c.is_ascii()
                                                    && !c.is_whitespace()
                                                    && !c.is_control()
                                            })
                                            .is_none()
                                        {
                                            return;
                                        }
                                        let cleaned: String = clipboard_text
                                            .chars()
                                            .filter(|c| !c.is_control() || *c == '\n' || *c == '\t')
                                            .collect();
                                        let trimmed = cleaned.trim_ascii_start();
                                        if !trimmed.is_empty() {
                                            self.save_snippets.push(SaveSnippet {
                                                title: String::new(),
                                                desc: String::new(),
                                                code: trimmed.to_string(),
                                                lang: "txt".to_string(),
                                            });
                                            self.snippets = convert_snippets(&self.save_snippets);
                                            self.dialog.open = true;
                                            self.dialog = self.dialog.title_top("Enter Title");
                                            self.dialog_field = "title".to_string();
                                            self.edit_idx =
                                                self.save_snippets.len().saturating_sub(1);
                                        } else {
                                            self.notify("Clipboard is empty");
                                        }
                                    }
                                    Err(_) => {
                                        self.notify("Failed to access clipboard");
                                    }
                                }
                            }
                            KeyCode::Char('d') | KeyCode::Char('D') => {
                                if let Some(idx) = self.list_state.selected {
                                    self.save_snippets.remove(idx);
                                    self.snippets.remove(idx);
                                    if self.save_snippets.is_empty() {
                                        self.list_state.select(None);
                                    } else if idx >= self.save_snippets.len() {
                                        self.list_state.select(Some(idx - 1));
                                    }
                                }
                            }
                            KeyCode::Char('c') | KeyCode::Char('C') => {
                                if let Some(idx) = self.list_state.selected
                                    && let Some(snippet) = self.save_snippets.get(idx)
                                {
                                    if self.clipboard.set_text(snippet.code.clone()).is_err() {
                                        self.notify("Failed to copy code to clipboard");
                                    }
                                }
                            }
                            KeyCode::Char('e') | KeyCode::Char('E') => {
                                if let Some(idx) = self.list_state.selected {
                                    self.edit_idx = idx;
                                    self.dialog.open = true;
                                    self.dialog = self.dialog.title_top("Enter Title");
                                    self.dialog_field = "title".to_string();
                                }
                            }
                            // TODO: add editing of snippets
                            _ => {}
                        }
                    }
                }
                _ => {}
            }
        }
    }
}

/// converts Vec<SaveSnippet> to Vec<CodeSnippet>
fn convert_snippets(snippets: &[SaveSnippet]) -> Vec<CodeSnippet> {
    snippets
        .iter()
        .map(|snip| {
            CodeSnippet::new(
                snip.title.clone(),
                snip.desc.clone(),
                snip.code.clone(),
                snip.lang.clone(),
            )
        })
        .collect()
}

/// creates a new dialog with custom options
fn new_dialog() -> Dialog {
    Dialog::default().style(Style::default().fg(Color::LightBlue))
}
