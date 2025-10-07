use crossterm::{ExecutableCommand, cursor, event, style, terminal};
use rattlesnake::PlayerEvent;
use std::fmt;
use std::io::{Stdout, Write, stdout};
use std::time;

const SYMBOL_FOOD: &str = "@";
const SYMBOL_SNAKE: &str = "S";
const SYMBOL_WALL: &str = "#";
const SYMBOL_EMPTY: &str = " ";

pub struct TerminalUI {
    stdout: Stdout,
    tail_cache: (u16, u16),
    food_cache: Vec<(u16, u16)>,
}

impl TerminalUI {
    pub fn new() -> Self {
        TerminalUI {
            stdout: stdout(),
            tail_cache: (0, 0),
            food_cache: Vec::new(),
        }
    }
}

impl Default for TerminalUI {
    fn default() -> Self {
        TerminalUI::new()
    }
}

impl TerminalUI {
    pub fn init(&mut self, field_width: u16, field_height: u16) {
        self.prepare_terminal();
        self.draw_field(field_width, field_height);
        self.clear_field(field_width, field_height);
        self.flush();
    }

    pub fn deinit(&mut self) {
        self.reset_terminal();
    }

    pub fn render(&mut self, snake: &[(u16, u16)], food: &Vec<(u16, u16)>) {
        for pos in food {
            if self.food_cache.contains(pos) {
                continue;
            }
            self.draw_food(pos);
        }

        self.draw_snake(&snake[0]);

        let tail = snake[snake.len() - 1];
        if tail != self.tail_cache {
            self.clear(self.tail_cache);
        }

        self.tail_cache = tail;
        self.food_cache = food.clone();

        self.flush();
    }

    // Poll event for `millis` milliseconds.
    pub fn poll(&self, millis: u64) -> PlayerEvent {
        if event::poll(time::Duration::from_millis(millis)).unwrap()
            && let event::Event::Key(event) = event::read().unwrap()
        {
            let key_event = match event.code {
                event::KeyCode::Up => PlayerEvent::Up,
                event::KeyCode::Down => PlayerEvent::Down,
                event::KeyCode::Left => PlayerEvent::Left,
                event::KeyCode::Right => PlayerEvent::Right,
                event::KeyCode::Char('q') => PlayerEvent::Quit,
                _ => PlayerEvent::Idle,
            };
            return key_event;
        }
        PlayerEvent::Idle
    }
    fn draw_field(&mut self, width: u16, height: u16) {
        self.draw_box(&SYMBOL_WALL, 0, 0, width + 2, height + 2);
    }

    fn draw_snake(&mut self, pos: &(u16, u16)) {
        self.draw(SYMBOL_SNAKE, pos.0, pos.1);
    }

    fn draw_food(&mut self, pos: &(u16, u16)) {
        self.draw(SYMBOL_FOOD, pos.0, pos.1);
    }

    fn clear(&mut self, pos: (u16, u16)) {
        self.draw(SYMBOL_EMPTY, pos.0, pos.1);
    }

    // Draw a box of width `w` and height `h` with top-left corner at `(x, y)`.
    fn draw_box<T: fmt::Display>(
        &mut self,
        s: &T,
        x: u16,
        y: u16,
        w: u16,
        h: u16,
    ) {
        if w > 0 {
            for i in 0..w {
                self.draw(s, x + i, y);
                self.draw(s, x + i, y + h - 1);
            }
        }

        if h > 1 {
            for j in 1..(h - 1) {
                self.draw(s, x, y + j);
                self.draw(s, x + w - 1, y + j);
            }
        }
    }

    // Draw `s` at position `(x, y)`.
    fn draw<T: fmt::Display>(&mut self, s: T, x: u16, y: u16) {
        self.stdout
            .execute(cursor::MoveTo(x, y))
            .unwrap()
            .execute(style::Print(s))
            .unwrap();
    }

    // Clear the field area.
    fn clear_field(&mut self, width: u16, height: u16) {
        for y in 1..height {
            for x in 1..width {
                self.clear((x, y));
            }
        }
    }

    // Flush the output buffer to the terminal.
    fn flush(&mut self) {
        self.stdout.flush().unwrap();
    }

    // Prepare terminal state.
    pub fn prepare_terminal(&mut self) {
        self.stdout
            .execute(terminal::Clear(terminal::ClearType::All))
            .unwrap()
            .execute(cursor::Hide)
            .unwrap();
        terminal::enable_raw_mode().unwrap();
    }

    // Restore terminal state.
    pub fn reset_terminal(&mut self) {
        self.stdout
            .execute(terminal::Clear(terminal::ClearType::All))
            .unwrap()
            .execute(cursor::Show)
            .unwrap();
        terminal::disable_raw_mode().unwrap();
    }
}
