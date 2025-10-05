use rattlesnake::{Settings, run};
use simplelog::WriteLogger;
use std::{env, fs};

const LOG_DIR: &str = "var/log/";
const LOG_FILE: &str = "var/log/app.log";

fn main() {
    if env::var("RS_LOG").unwrap_or("false".to_string()) == "true" {
        enable_logging();
    }

    let settings = Settings {
        width: 40,
        height: 20,
    };
    run(settings);
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
