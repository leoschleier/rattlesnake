//! Snake game.
use log::{debug, info};

#[derive(Debug)]
pub struct Field {
    x_min: u16,
    x_max: u16,
    y_min: u16,
    y_max: u16,
}

impl Field {
    pub fn new(width: u16, height: u16) -> Self {
        Field {
            x_min: 1,
            x_max: width,
            y_min: 1,
            y_max: height,
        }
    }
}

pub enum PlayerEvent {
    Up,
    Down,
    Left,
    Right,
    Quit,
    Idle,
}

pub enum GameResult {
    Continue,
    GameOver,
}

#[derive(Debug, Default)]
pub struct GameState {
    pub snake: Vec<(u16, u16)>,
    pub food: Vec<(u16, u16)>,
    pub direction: (i16, i16),
}

impl GameState {
    pub fn new() -> Self {
        GameState {
            snake: Vec::new(),
            food: Vec::new(),
            direction: (0, 0),
        }
    }
}

// Run the game with given settings.
pub fn play(
    state: &mut GameState,
    field: &Field,
    event: &PlayerEvent,
) -> GameResult {
    debug!("Game state: {:?}", state);
    debug!("Field: {:?}", field);

    let spawn =
        random_position(field.x_min, field.x_max, field.y_min, field.y_max);

    // Initialize snake
    if state.snake.is_empty() {
        info!("Initializing snake...");
        let initial_position =
            (field.x_min + field.x_max / 2, field.y_min + field.y_max / 2);
        state.snake.push(initial_position);
    }
    // Initialize food
    if state.food.is_empty() {
        state.food.push(spawn());
    }

    // Move snake
    state.direction = find_direction(event, state.direction);
    let past_tail = state.snake[state.snake.len() - 1];
    locomote(&mut state.snake, state.direction.0, state.direction.1);
    debug!("Moved snake {:?}", state.snake);

    // Detect collisions
    if collided(&state.snake, field) {
        return GameResult::GameOver;
    }

    // Eat food and spawn new
    for f_idx in 0..state.food.len() {
        if past_tail == state.food[f_idx] {
            state.snake.push(past_tail);
            info!("Ate food at {:?}", past_tail);
            state.food.remove(f_idx);
            let exclude: Vec<(u16, u16)> =
                state.snake.iter().chain(&state.food).cloned().collect();
            state.food.push(random_exclude(&spawn, &exclude));
            break;
        }
    }

    GameResult::Continue
}

// Determine the new direction based on the key event and previous direction.
fn find_direction(event: &PlayerEvent, prev: (i16, i16)) -> (i16, i16) {
    let delta = match event {
        PlayerEvent::Up => (0, -1),
        PlayerEvent::Down => (0, 1),
        PlayerEvent::Left => (-1, 0),
        PlayerEvent::Right => (1, 0),
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
