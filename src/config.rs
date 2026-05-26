use serde::Deserialize;
use std::{fs, path::Path};

pub const DEFAULT_CONFIG: &str = r#"
[keymap]
up = ["k", "up"]
down = ["j", "down"]
enter = ["enter", "l"]
back = ["h", "backspace"]
quit = ["q"]
command_mode = [":"]
search_mode = ["/"]
path_mode = ["P"]
"#;

#[derive(Deserialize)]
pub struct Config {
    pub keymap: KeymapConfig,
}

#[derive(Deserialize)]
pub struct KeymapConfig {
    pub up: Vec<String>,
    pub down: Vec<String>,
    pub enter: Vec<String>,
    pub back: Vec<String>,
    pub quit: Vec<String>,
    pub command_mode: Vec<String>,
    pub search_mode: Vec<String>,
    pub path_mode: Vec<String>,
}

impl Config {
    pub fn load() -> Self {
        let path = "config.toml";

        if !Path::new(path).exists() {
            fs::write(path, DEFAULT_CONFIG)
                .expect("failed to write default config");
        }

        let text = fs::read_to_string(path)
            .expect("failed to read config.toml");

        toml::from_str(&text)
            .expect("invalid config.toml")
    }
}
