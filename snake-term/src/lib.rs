use crossterm::{ExecutableCommand, cursor, event, style, terminal};
use rattlesnake::{KeyEvent, UI};
use std::fmt;
use std::io::{Stdout, Write, stdout};
use std::time;

const SYMBOL_FOOD: &str = "@";
const SYMBOL_SNAKE: &str = "S";
const SYMBOL_WALL: &str = "#";
const SYMBOL_EMPTY: &str = " ";

pub struct TerminalUI {
    stdout: Stdout,
}

impl TerminalUI {
    pub fn new() -> Self {
        TerminalUI { stdout: stdout() }
    }
}

impl Default for TerminalUI {
    fn default() -> Self {
        TerminalUI::new()
    }
}

impl UI for TerminalUI {
    fn draw_field(&self, width: u16, height: u16) {
        draw_box(&SYMBOL_WALL, 0, 0, width + 1, height + 1);
    }

    fn draw_snake(&self, pos: &(u16, u16)) {
        draw(SYMBOL_SNAKE, pos.0, pos.1);
    }

    fn draw_food(&self, pos: (u16, u16)) {
        draw(SYMBOL_FOOD, pos.0, pos.1);
    }

    fn clear(&self, pos: (u16, u16)) {
        draw(SYMBOL_EMPTY, pos.0, pos.1);
    }

    fn clear_field(&self, width: u16, height: u16) {
        for x in 1..width {
            for y in 1..height {
                self.clear((x, y));
            }
        }
    }

    fn flush(&mut self) {
        self.stdout.flush().unwrap();
    }

    fn poll(&self, millis: u64) -> Option<KeyEvent> {
        if event::poll(time::Duration::from_millis(millis)).unwrap()
            && let event::Event::Key(event) = event::read().unwrap()
        {
            let key_event = match event.code {
                event::KeyCode::Up => KeyEvent::Up,
                event::KeyCode::Down => KeyEvent::Down,
                event::KeyCode::Left => KeyEvent::Left,
                event::KeyCode::Right => KeyEvent::Right,
                event::KeyCode::Char('q') => KeyEvent::Quit,
                _ => return None,
            };
            return Some(key_event);
        }
        None
    }
}

// Prepare terminal state.
pub fn prepare_terminal() {
    let mut stdout = stdout();
    stdout
        .execute(terminal::Clear(terminal::ClearType::All))
        .unwrap();
    stdout.execute(cursor::Hide).unwrap();
    terminal::enable_raw_mode().unwrap();
}

// Restore terminal state.
pub fn reset_terminal() {
    let mut stdout = stdout();
    stdout.execute(cursor::Show).unwrap();
}

// Draw a box of width `w` and height `h` with top-left corner at `(x, y)`.
fn draw_box<T: fmt::Display>(s: &T, x: u16, y: u16, w: u16, h: u16) {
    if w > 0 {
        for i in 0..w {
            draw(s, x + i, y);
            draw(s, x + i, y + h - 1);
        }
    }

    if h > 1 {
        for j in 1..(h - 1) {
            draw(s, x, y + j);
            draw(s, x + w - 1, y + j);
        }
    }
}

// Draw `s` at position `(x, y)`.
fn draw<T: fmt::Display>(s: T, x: u16, y: u16) {
    stdout()
        .execute(cursor::MoveTo(x, y))
        .unwrap()
        .execute(style::Print(s))
        .unwrap();
}
