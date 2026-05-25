use crossterm::event::{self, Event};
use crate::keymap::KeyMap;
use crate::action::Action;

pub fn read(keymap: &KeyMap) -> std::io::Result<Action> {
    if let Event::Key(key) = event::read()? {
        Ok(
            keymap
                .map
                .get(&key.code)
                .copied()
                .unwrap_or(Action::None)
        )
    } else {
        Ok(Action::None)
    }
}