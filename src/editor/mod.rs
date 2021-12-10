mod editor;
mod pixel_widget;
mod util;

use editor::ImageEditor;
use util::event::{Event, Events};

use std::io;
use termion::{event::Key, input::MouseTerminal, raw::IntoRawMode, screen::AlternateScreen};
use tui::{backend::TermionBackend, layout::Rect, Terminal};

pub fn run(file: &str, pixel_size: u32) {
    // Terminal initialization
    let stdout = io::stdout().into_raw_mode().unwrap();
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend).unwrap();
    let editor = ImageEditor::new(file, pixel_size);

    // Setup event handlers
    let events = Events::new();

    loop {
        // Draw UI
        terminal
            .draw(|f| {
                let size = f.size();
                let rect = Rect {
                    x: 2,
                    y: 1,
                    width: size.width,
                    height: size.height,
                };
                f.render_widget(&editor, rect);
            })
            .unwrap();

        // Handle input
        if let Event::Input(input) = events.next().unwrap() {
            match input {
                Key::Ctrl('c') | Key::Char('q') => break,
                _ => (),
            };
        }
    }
}
