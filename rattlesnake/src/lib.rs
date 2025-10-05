//! Snake game.
use crossterm::{ExecutableCommand, cursor, event, style, terminal};
use log::{debug, info};
use std::{
    fmt,
    io::{Write, stdout},
    time,
};

const SYMBOL_EMPTY: &str = " ";
const SYMBOL_FOOD: &str = "@";
const SYMBOL_SNAKE: &str = "S";
const SYMBOL_WALL: &str = "#";

#[derive(Debug)]
pub struct Settings {
    pub width: u16,
    pub height: u16,
}

#[derive(Debug)]
struct Field {
    x_min: u16,
    x_max: u16,
    y_min: u16,
    y_max: u16,
}

enum GameResult {
    GameOver,
    Quit,
}

// Run the game with given settings.
pub fn run(settings: Settings) {
    info!("Initializing...");
    debug!("Settings: {:#?}", settings);

    prepare();

    let mut stdout = stdout();

    let anchor_point = (0, 0);

    let field = Field {
        x_min: anchor_point.0 + 1,
        x_max: anchor_point.0 + settings.width - 2,
        y_min: anchor_point.1 + 1,
        y_max: anchor_point.1 + settings.height - 2,
    };

    debug!("Field: {:?}", field);

    let spawn =
        random_position(field.x_min, field.x_max, field.y_min, field.y_max);

    draw_box(
        &SYMBOL_WALL,
        anchor_point.0,
        anchor_point.1,
        settings.width,
        settings.height,
    );

    let initial_position =
        (field.x_min + field.x_max / 2, field.y_min + field.y_max / 2);

    stdout.flush().unwrap();

    info!("Starting game...");
    while let GameResult::GameOver = play(&field, initial_position, &spawn) {
        clear(&field);
        stdout.flush().unwrap();
        info!("Game Over! Restarting...");
    }

    info!("Cleaning up...");
    cleanup();
    info!("Exiting game...");
}

// Run a game session.
fn play(
    field: &Field,
    initial_position: (u16, u16),
    spawn: impl Fn() -> (u16, u16),
) -> GameResult {
    let mut snake: Vec<(u16, u16)> = vec![initial_position];
    let mut direction: (i16, i16) = (0, 0);

    let mut food = spawn();
    draw(SYMBOL_FOOD, food.0, food.1);

    let mut stdout = stdout();
    loop {
        // Check if an event is available (with timeout)
        if event::poll(time::Duration::from_millis(500)).unwrap() {
            // Read the event
            if let event::Event::Key(key_event) = event::read().unwrap() {
                let action = resolve_event(key_event);
                if let Action::Quit = action {
                    return GameResult::Quit;
                }
                direction = find_direction(key_event, direction);
            }
        }
        let tail = snake[snake.len() - 1];
        locomote(&mut snake, direction.0, direction.1);

        debug!("New position {:?}", snake);

        if collided(&snake, field) {
            return GameResult::GameOver;
        }

        if tail == food {
            snake.push(food);
            food = random_exclude(&spawn, &snake);
            draw(SYMBOL_FOOD, food.0, food.1);
        }

        for s in &snake {
            draw(SYMBOL_EMPTY, tail.0, tail.1);
            draw(SYMBOL_SNAKE, s.0, s.1);
        }

        stdout.flush().unwrap();
    }
}

// Prepare terminal state.
fn prepare() {
    let mut stdout = stdout();
    stdout
        .execute(terminal::Clear(terminal::ClearType::All))
        .unwrap();
    stdout.execute(cursor::Hide).unwrap();
    terminal::enable_raw_mode().unwrap();
}

// Restore terminal state.
fn cleanup() {
    let mut stdout = stdout();
    stdout.execute(cursor::Show).unwrap();
}

// Clear the field.
fn clear(field: &Field) {
    for x in field.x_min..=field.x_max {
        for y in field.y_min..=field.y_max {
            draw(SYMBOL_EMPTY, x, y);
        }
    }
}

enum Action {
    Quit,
    None,
}

// Determine the action based on the key event.
fn resolve_event(key_event: event::KeyEvent) -> Action {
    match key_event.code {
        event::KeyCode::Char('q') => Action::Quit,
        _ => Action::None,
    }
}

// Determine the new direction based on the key event and previous direction.
fn find_direction(key_event: event::KeyEvent, prev: (i16, i16)) -> (i16, i16) {
    let delta = match key_event.code {
        event::KeyCode::Up => (0, -1),
        event::KeyCode::Down => (0, 1),
        event::KeyCode::Left => (-1, 0),
        event::KeyCode::Right => (1, 0),
        _ => prev,
    };
    if prev.0 + delta.0 == 0 && prev.1 + delta.1 == 0 {
        prev
    } else {
        delta
    }
}

// Check if the snake has collided with walls or itself.
fn collided(snake: &[(u16, u16)], field: &Field) -> bool {
    let head = match snake.first() {
        Some(&pos) => pos,
        None => return false, // Snake is empty
    };

    if head.0 < field.x_min
        || head.0 > field.x_max
        || head.1 < field.y_min
        || head.1 > field.y_max
    {
        // Collided with wall
        info!("Collided with wall at position {:?}", head);
        return true;
    }

    if snake.len() > 1 {
        for s in snake[1..].iter() {
            if head == *s {
                // Collided with itself
                info!("Collided with itself {:?}", head);
                return true;
            }
        }
    }

    false
}

// Move the snake by `(dx, dy)`.
fn locomote(snake: &mut Vec<(u16, u16)>, dx: i16, dy: i16) {
    if snake.is_empty() {
        snake.push((0, 0));
    }

    let (x, y) = snake[0];

    snake.remove(snake.len() - 1);

    let new_x = ((x as i16) + dx).max(0) as u16;
    let new_y = ((y as i16) + dy).max(0) as u16;

    snake.insert(0, (new_x, new_y));
}

// Generate a random position within given bounds.
fn random_position(
    x_min: u16,
    x_max: u16,
    y_min: u16,
    y_max: u16,
) -> impl Fn() -> (u16, u16) {
    move || {
        let x = rand::random_range(x_min..=x_max);
        let y = rand::random_range(y_min..=y_max);
        (x, y)
    }
}

// Generate a random values excluding certain values.
//
// # Arguments
//
// * `randomizer` - a function that generates random values
// * `exclude` - values to exclude
fn random_exclude<T: PartialEq>(
    randomizer: &impl Fn() -> T,
    exclude: &[T],
) -> T {
    loop {
        let v = randomizer();
        if !exclude.contains(&v) {
            return v;
        }
    }
}

// Draw a box with top-left corner at `(x, y)`, width `w` and height `h`.
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
