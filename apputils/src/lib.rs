use simplelog::WriteLogger;
use std::{env, fs};

pub fn enable_logging(log_dir: &str, log_file: &str) {
    if env::var("RS_LOG").unwrap_or("false".to_string()) == "true" {
        fs::create_dir_all(log_dir).unwrap();
        let file = fs::File::create(format!("{log_dir}/{log_file}")).unwrap();

        WriteLogger::init(
            simplelog::LevelFilter::Debug,
            simplelog::Config::default(),
            file,
        )
        .unwrap();
    }
}
