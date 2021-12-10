mod editor;
mod pixel_widget;
mod util;

use crate::vm::Direction::{East, North, South, West};
use editor::ImageEditor;
use util::event::{Event, Events};

use std::io;
use termion::{event::Key, input::MouseTerminal, raw::IntoRawMode, screen::AlternateScreen};
use tui::{
    backend::TermionBackend,
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{Block, Borders, Paragraph},
    Terminal,
};

pub fn run(file: &str, pixel_size: u32) {
    // Terminal initialization
    let stdout = io::stdout().into_raw_mode().unwrap();
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend).unwrap();
    let mut editor = ImageEditor::new(file, pixel_size);

    // Setup event handlers
    let events = Events::new();

    loop {
        // Draw UI
        terminal
            .draw(|f| {
                let chunks = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints([Constraint::Percentage(80), Constraint::Percentage(20)].as_ref())
                    .split(f.size());
                editor.block(Block::default().borders(Borders::ALL));
                f.render_widget(&editor, chunks[0]);
                f.render_widget(
                    Paragraph::new(editor.pixel_info())
                        .block(Block::default().borders(Borders::ALL)),
                    chunks[1],
                );
            })
            .unwrap();

        // Handle input
        if let Event::Input(input) = events.next().unwrap() {
            match input {
                Key::Ctrl('c') | Key::Char('q') => break,
                Key::Char('h') | Key::Left => editor.go(West, 1),
                Key::Char('l') | Key::Right => editor.go(East, 1),
                Key::Char('j') | Key::Down => editor.go(South, 1),
                Key::Char('k') | Key::Up => editor.go(North, 1),
                _ => (),
            };
        }
    }
}
