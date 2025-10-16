use apputils::enable_logging;
use rattlesnake::{Field, GameState, PlayerEvent, play};
use snake_wasm::BrowserUI;

const LOG_DIR: &str = "var/log/";
const LOG_FILE: &str = "snake_wasm.log";

#[macroquad::main("SnakeWasm")]
async fn main() {
    enable_logging(LOG_DIR, LOG_FILE);

    let width: u16 = 20;
    let height: u16 = 20;
    let thickness: u16 = 20;
    let field = Field::new(width, height);

    let mut ui: BrowserUI;
    let mut state: GameState;
    let mut event: PlayerEvent;
    let mut start: std::time::Instant;
    loop {
        ui = BrowserUI::new(width, height, thickness);
        state = GameState::new();
        event = PlayerEvent::Idle;
        loop {
            start = std::time::Instant::now();
            event = ui.poll(250, &event);
            if let PlayerEvent::Quit = event {
                break;
            }
            while start.elapsed().as_millis() < 250 {}

            let result = play(&mut state, &field, &event);
            match result {
                rattlesnake::GameResult::Continue => {}
                rattlesnake::GameResult::GameOver => break,
            }

            ui.render(&state.snake, &state.food);

            ui.flush().await;
        }
        if let PlayerEvent::Quit = event {
            break;
        }
    }
}
