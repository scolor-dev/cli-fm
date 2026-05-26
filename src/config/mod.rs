pub mod keymap;
pub mod theme;

use serde::Deserialize;
use std::{fs, path::Path};

pub use keymap::KeymapConfig;

const DEFAULT_CONFIG: &str = r#"
[keymap.normal]
up = ["k", "up"]
down = ["j", "down"]
enter = ["enter", "l"]
back = ["h", "backspace"]
quit = ["q"]
command_mode = [":"]
search_mode = ["/"]
path_mode = ["P"]
copy = ["c"]
cut = ["m"]
paste = ["p"]
delete = ["d"]
rename = ["r"]
new_entry = ["a"]
undo = ["Z"]
redo = ["Y"]

[keymap.input]
submit = ["enter"]
cancel = ["escape"]
backspace = ["backspace"]
tab_complete = ["tab"]

[keymap.confirm]
yes = ["y"]
no = ["n", "escape"]
"#;

#[derive(Deserialize)]
pub struct Config {
    pub keymap: KeymapConfig,
}

impl Config {
    pub fn load() -> Self {
        let path = "config.toml";
        if !Path::new(path).exists() {
            fs::write(path, DEFAULT_CONFIG).expect("failed to write default config");
        }
        let text = fs::read_to_string(path).expect("failed to read config.toml");
        toml::from_str(&text).expect("invalid config.toml")
    }
}
