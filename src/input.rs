use crossterm::event::{self, Event, KeyCode};

pub enum Action {
    Up,
    Down,
    Quit,
    None,
}

pub fn read() -> std::io::Result<Action> {
    if let Event::Key(key) = event::read()? {
        let action = match key.code {
            KeyCode::Char('k') | KeyCode::Up => Action::Up,
            KeyCode::Char('j') | KeyCode::Down => Action::Down,
            KeyCode::Char('q') => Action::Quit,
            _ => Action::None,
        };

        Ok(action)
    } else {
        Ok(Action::None)
    }
}