//! Snake game.
use log::{debug, info};

pub trait UI {
    fn draw_field(&self, width: u16, height: u16);
    fn draw_snake(&self, pos: &(u16, u16));
    fn draw_food(&self, pos: (u16, u16));
    fn clear(&self, pos: (u16, u16));
    fn clear_field(&self, with: u16, height: u16);
    fn flush(&mut self);
    fn poll(&self, millis: u64) -> Option<KeyEvent>;
}

pub struct Handles<'a, U: UI> {
    pub ui: &'a mut U,
    pub on_startup: fn(),
    pub on_shutdown: fn(),
}

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

pub enum KeyEvent {
    Up,
    Down,
    Left,
    Right,
    Quit,
}

enum GameResult {
    GameOver,
    Quit,
}

// Run the game with given settings.
pub fn run<U: UI>(settings: Settings, handles: Handles<U>) {
    info!("Initializing...");
    debug!("Settings: {:#?}", settings);

    let ui = handles.ui;

    (handles.on_startup)();

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

    ui.draw_field(settings.width, settings.height);
    ui.flush();

    let initial_position =
        (field.x_min + field.x_max / 2, field.y_min + field.y_max / 2);

    info!("Starting game...");
    while let GameResult::GameOver = play(ui, &field, initial_position, &spawn)
    {
        ui.clear_field(settings.width, settings.height);
        ui.flush();
        info!("Game Over! Restarting...");
    }

    info!("Cleaning up...");
    (handles.on_shutdown)();
    info!("Exiting game...");
}

// Run a game session.
fn play(
    ui: &mut impl UI,
    field: &Field,
    initial_position: (u16, u16),
    spawn: &impl Fn() -> (u16, u16),
) -> GameResult {
    let mut snake: Vec<(u16, u16)> = vec![initial_position];
    let mut direction: (i16, i16) = (0, 0);

    let mut food = spawn();
    ui.draw_food(food);

    loop {
        // Check if an event is available (with timeout)
        if let Some(event) = ui.poll(500) {
            if let KeyEvent::Quit = event {
                return GameResult::Quit;
            }
            direction = find_direction(event, direction);
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
            ui.draw_food(food);
        }

        ui.clear(tail);
        for s in &snake {
            ui.draw_snake(s);
        }

        ui.flush();
    }
}

// Determine the new direction based on the key event and previous direction.
fn find_direction(event: KeyEvent, prev: (i16, i16)) -> (i16, i16) {
    let delta = match event {
        KeyEvent::Up => (0, -1),
        KeyEvent::Down => (0, 1),
        KeyEvent::Left => (-1, 0),
        KeyEvent::Right => (1, 0),
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
