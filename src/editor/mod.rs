mod editor;
mod pixel_widget;
mod util;

use crate::vm::Direction::{East, North, South, West};
use editor::{ImageEditor, State};
use util::event::{Event, Events};

use std::io;
use termion::{event::Key, input::MouseTerminal, raw::IntoRawMode, screen::AlternateScreen};
use tui::{
    backend::TermionBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
    Terminal,
};

use anyhow::anyhow;

pub fn run(file: &str, pixel_size: u32) {
    // Terminal initialization
    let stdout = io::stdout().into_raw_mode().unwrap();
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend).unwrap();
    let mut editor = ImageEditor::new(file, pixel_size);

    // Setup event handlers
    let mut events = Events::new();
    events.disable_exit_key();

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
                let info_chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([Constraint::Max(9), Constraint::Max(10)])
                    .split(chunks[1]);
                f.render_widget(
                    Paragraph::new(editor.pixel_info())
                        .block(Block::default().borders(Borders::ALL)),
                    info_chunks[0],
                );
                let input_box = match editor.state() {
                    state @ State::Normal => format!("{:?}", state),
                    state => {
                        let input = editor.input();
                        f.set_cursor(
                            info_chunks[1].x + input.len() as u16 + 3,
                            info_chunks[1].y + 2,
                        );
                        format!("{:?}\n> {}", state, editor.input())
                    }
                };
                f.render_widget(
                    Paragraph::new(input_box).block(Block::default().borders(Borders::ALL).style(
                        Style::default().fg(match editor.state() {
                            State::Replace => Color::Yellow,
                            _ => Color::White,
                        }),
                    )),
                    info_chunks[1],
                );
            })
            .unwrap();

        // Handle input
        if let Event::Input(input) = events.next().unwrap() {
            match editor.state() {
                State::Normal => match input {
                    Key::Ctrl('c') | Key::Char('q') => break,
                    Key::Char('h') | Key::Left => editor.go(West, 1),
                    Key::Char('l') | Key::Right => editor.go(East, 1),
                    Key::Char('j') | Key::Down => editor.go(South, 1),
                    Key::Char('k') | Key::Up => editor.go(North, 1),
                    Key::Char('r') => editor.set_state(State::Replace),
                    Key::Char(':') => editor.set_state(State::Command),
                    _ => (),
                },
                state => match input {
                    Key::Ctrl('c') | Key::Esc => editor.set_state(State::Normal),
                    Key::Char('\n') => {
                        let input = editor.submit();
                        // TODO: notify user of error
                        match state {
                            State::Replace => {
                                if let Ok(hue) = input.parse() {
                                    editor.replace_current(hue);
                                }
                            }
                            State::Command => {
                                let result = match input.as_str() {
                                    "w" | "wq" => editor.save(),
                                    "q" | "quit" => break,
                                    _ => Err(anyhow!("unrecognized command")),
                                };
                                // TODO: notify user of error
                                if result.is_ok() && input.as_str().ends_with("q") {
                                    break;
                                }
                            }
                            _ => unreachable!(),
                        }
                    }
                    Key::Backspace => editor.pop(),
                    Key::Char(c) => editor.push(c),
                    _ => (),
                },
            }
        }
    }
}
