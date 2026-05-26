pub mod keymap;

use crossterm::event::{self, Event, KeyCode};

use crate::{core::{action::Action, mode::Mode}, input::keymap::KeyMap};

pub fn read(mode: &Mode, keymap: &KeyMap) -> std::io::Result<Action> {
    if let Event::Key(key) = event::read()? {
        let action = match mode {
            Mode::Normal => keymap
                .normal
                .get(&key.code)
                .copied()
                .unwrap_or(Action::None),
            Mode::Command | Mode::Search | Mode::PathInput | Mode::Rename | Mode::NewEntry => {
                keymap
                    .input
                    .get(&key.code)
                    .copied()
                    .unwrap_or_else(|| match key.code {
                        KeyCode::Char(c) => Action::InputChar(c),
                        _ => Action::None,
                    })
            }
            Mode::Confirm => keymap
                .confirm
                .get(&key.code)
                .copied()
                .unwrap_or(Action::None),
        };
        Ok(action)
    } else {
        Ok(Action::None)
    }
}
