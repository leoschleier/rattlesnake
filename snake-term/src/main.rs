use rattlesnake::{Handles, Settings, run};
use simplelog::WriteLogger;
use snake_term::{TerminalUI, prepare_terminal, reset_terminal};
use std::{env, fs};

const LOG_DIR: &str = "var/log/";
const LOG_FILE: &str = "var/log/app.log";

fn main() {
    if env::var("RS_LOG").unwrap_or("false".to_string()) == "true" {
        enable_logging();
    }

    let handles = Handles {
        ui: &mut TerminalUI::new(),
        on_startup: prepare_terminal,
        on_shutdown: reset_terminal,
    };

    let settings = Settings {
        width: 40,
        height: 20,
    };

    run(settings, handles);
}

fn enable_logging() {
    fs::create_dir_all(LOG_DIR).unwrap();
    let file = fs::File::create(LOG_FILE).unwrap();

    WriteLogger::init(
        simplelog::LevelFilter::Debug,
        simplelog::Config::default(),
        file,
    )
    .unwrap();
}
