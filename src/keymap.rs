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

        k if k.len() == 1 => {
            Some(KeyCode::Char(k.chars().next()?))
        }

        _ => None,
    }
}