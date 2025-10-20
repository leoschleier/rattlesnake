use apputils::enable_logging;
use macroquad::rand;
use rattlesnake::RandomRange;
use rattlesnake::{Field, GameState, PlayerEvent, play};
use snake_wasm::{BrowserUI, now_millis};

const LOG_DIR: &str = "var/log/";
const LOG_FILE: &str = "snake_wasm.log";

#[macroquad::main("SnakeWasm")]
async fn main() {
    enable_logging(LOG_DIR, LOG_FILE);

    let width: u16 = 19;
    let height: u16 = 19;
    let thickness: u16 = 20;
    let field = Field::new(width, height);

    let mut ui: BrowserUI;
    let mut state: GameState;
    let mut event: PlayerEvent;
    let mut start: u64;
    let random_range: RandomRange = rand::gen_range;
    loop {
        ui = BrowserUI::new(width, height, thickness);
        state = GameState::new();
        event = PlayerEvent::Idle;
        loop {
            start = now_millis();
            event = ui.poll(250, &event);
            if let PlayerEvent::Quit = event {
                break;
            }
            while now_millis() - start < 250 {}

            let result = play(&mut state, &field, &event, &random_range);
            match result {
                rattlesnake::GameResult::Continue => {}
                rattlesnake::GameResult::GameOver => break,
            }

            ui.render(&state.snake, &state.food, state.score);

            ui.flush().await;
        }
        if let PlayerEvent::Quit = event {
            break;
        }
    }
}
