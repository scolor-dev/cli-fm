use std::collections::HashMap;
use crossterm::event::KeyCode;

use crate::{config::Config, core::action::Action};

pub struct KeyMap {
    pub normal: HashMap<KeyCode, Action>,
    pub input: HashMap<KeyCode, Action>,
    pub confirm: HashMap<KeyCode, Action>,
}

impl KeyMap {
    pub fn from_config(config: &Config) -> Self {
        let n = &config.keymap.normal;
        let mut normal = HashMap::new();
        bind(&mut normal, &n.up, Action::Up);
        bind(&mut normal, &n.down, Action::Down);
        bind(&mut normal, &n.enter, Action::Enter);
        bind(&mut normal, &n.back, Action::Back);
        bind(&mut normal, &n.quit, Action::Quit);
        bind(&mut normal, &n.command_mode, Action::EnterCommandMode);
        bind(&mut normal, &n.search_mode, Action::EnterSearchMode);
        bind(&mut normal, &n.path_mode, Action::EnterPathMode);
        bind(&mut normal, &n.copy, Action::Copy);
        bind(&mut normal, &n.cut, Action::Cut);
        bind(&mut normal, &n.paste, Action::Paste);
        bind(&mut normal, &n.delete, Action::Delete);
        bind(&mut normal, &n.rename, Action::EnterRenameMode);
        bind(&mut normal, &n.new_entry, Action::EnterNewEntryMode);
        bind(&mut normal, &n.undo, Action::Undo);
        bind(&mut normal, &n.redo, Action::Redo);

        let i = &config.keymap.input;
        let mut input = HashMap::new();
        bind(&mut input, &i.submit, Action::InputSubmit);
        bind(&mut input, &i.cancel, Action::EnterNormalMode);
        bind(&mut input, &i.backspace, Action::InputBackspace);
        bind(&mut input, &i.tab_complete, Action::TabComplete);

        let c = &config.keymap.confirm;
        let mut confirm = HashMap::new();
        bind(&mut confirm, &c.yes, Action::ConfirmYes);
        bind(&mut confirm, &c.no, Action::ConfirmNo);

        Self { normal, input, confirm }
    }
}

fn bind(map: &mut HashMap<KeyCode, Action>, keys: &[String], action: Action) {
    for key in keys {
        if let Some(code) = parse_key(key) {
            map.insert(code, action);
        }
    }
}

pub fn parse_key(key: &str) -> Option<KeyCode> {
    match key.to_lowercase().as_str() {
        "up" => Some(KeyCode::Up),
        "down" => Some(KeyCode::Down),
        "left" => Some(KeyCode::Left),
        "right" => Some(KeyCode::Right),
        "enter" => Some(KeyCode::Enter),
        "backspace" => Some(KeyCode::Backspace),
        "escape" | "esc" => Some(KeyCode::Esc),
        "tab" => Some(KeyCode::Tab),
        "space" => Some(KeyCode::Char(' ')),
        _ if key.len() == 1 => Some(KeyCode::Char(key.chars().next()?)),
        _ => None,
    }
}
