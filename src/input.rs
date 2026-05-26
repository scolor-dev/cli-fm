use crossterm::event::{self, Event, KeyCode};
use crate::keymap::KeyMap;
use crate::action::Action;
use crate::app::Mode;

pub fn read(mode: &Mode, keymap: &KeyMap) -> std::io::Result<Action> {
    if let Event::Key(key) = event::read()? {
        let action = match mode {
            Mode::Normal => keymap
                .map
                .get(&key.code)
                .copied()
                .unwrap_or(Action::None),
            Mode::Command | Mode::Search => match key.code {
                KeyCode::Esc => Action::EnterNormalMode,
                KeyCode::Enter => Action::InputSubmit,
                KeyCode::Backspace => Action::InputBackspace,
                KeyCode::Char(c) => Action::InputChar(c),
                _ => Action::None,
            },
            Mode::PathInput => match key.code {
                KeyCode::Esc => Action::EnterNormalMode,
                KeyCode::Enter => Action::InputSubmit,
                KeyCode::Backspace => Action::InputBackspace,
                KeyCode::Tab => Action::TabComplete,
                KeyCode::Char(c) => Action::InputChar(c),
                _ => Action::None,
            },
            Mode::Rename => match key.code {
                KeyCode::Esc => Action::EnterNormalMode,
                KeyCode::Enter => Action::InputSubmit,
                KeyCode::Backspace => Action::InputBackspace,
                KeyCode::Char(c) => Action::InputChar(c),
                _ => Action::None,
            },
        };
        Ok(action)
    } else {
        Ok(Action::None)
    }
}
