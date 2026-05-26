use std::collections::HashMap;
use crossterm::event::KeyCode;

use crate::{action::Action, config::Config};

pub struct KeyMap {
    pub map: HashMap<KeyCode, Action>,
}

impl KeyMap {
    pub fn from_config(config: &Config) -> Self {
        let mut map = HashMap::new();

        bind(&mut map, &config.keymap.up, Action::Up);
        bind(&mut map, &config.keymap.down, Action::Down);
        bind(&mut map, &config.keymap.enter, Action::Enter);
        bind(&mut map, &config.keymap.back, Action::Back);
        bind(&mut map, &config.keymap.quit, Action::Quit);
        bind(&mut map, &config.keymap.command_mode, Action::EnterCommandMode);
        bind(&mut map, &config.keymap.search_mode, Action::EnterSearchMode);
        bind(&mut map, &config.keymap.path_mode, Action::EnterPathMode);
        bind(&mut map, &config.keymap.copy, Action::Copy);
        bind(&mut map, &config.keymap.cut, Action::Cut);
        bind(&mut map, &config.keymap.paste, Action::Paste);
        bind(&mut map, &config.keymap.delete, Action::Delete);
        bind(&mut map, &config.keymap.rename, Action::EnterRenameMode);

        Self { map }
    }
}

fn bind(
    map: &mut HashMap<KeyCode, Action>,
    keys: &[String],
    action: Action,
) {
    for key in keys {
        if let Some(code) = parse_key(key) {
            map.insert(code, action);
        }
    }
}

fn parse_key(key: &str) -> Option<KeyCode> {
    match key.to_lowercase().as_str() {
        "up" => Some(KeyCode::Up),
        "down" => Some(KeyCode::Down),
        "left" => Some(KeyCode::Left),
        "right" => Some(KeyCode::Right),
        "enter" => Some(KeyCode::Enter),
        "backspace" => Some(KeyCode::Backspace),
        _ if key.len() == 1 => Some(KeyCode::Char(key.chars().next()?)),
        _ => None,
    }
}
