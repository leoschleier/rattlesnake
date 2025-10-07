use apputils::enable_logging;
use rattlesnake::{Field, GameState, PlayerEvent, play};
use snake_term::TerminalUI;

const LOG_DIR: &str = "var/log/";
const LOG_FILE: &str = "snake-term.log";

fn main() {
    enable_logging(LOG_DIR, LOG_FILE);

    let width: u16 = 40;
    let height: u16 = 20;
    let field = Field::new(width, height);

    let mut ui: TerminalUI;
    let mut state: GameState;
    let mut event: PlayerEvent;
    loop {
        state = GameState::new();
        event = PlayerEvent::Idle;
        ui = TerminalUI::new();
        ui.init(width, height);
        loop {
            let result = play(&mut state, &field, &event);
            match result {
                rattlesnake::GameResult::Continue => {}
                rattlesnake::GameResult::GameOver => break,
            }

            ui.render(&state.snake, &state.food);

            event = ui.poll(250);
            if let PlayerEvent::Quit = event {
                break;
            }
        }
        if let PlayerEvent::Quit = event {
            break;
        }
    }

    ui.deinit();
}
