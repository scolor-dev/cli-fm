mod app;
mod input;
mod ui;
mod theme;
mod config;
mod keymap;
mod action;

use app::App;
use config::Config;
use keymap::KeyMap;

use crossterm::{
    execute,
    terminal::{
        enable_raw_mode,
        disable_raw_mode,
        EnterAlternateScreen,
        LeaveAlternateScreen,
    },
};

use ratatui::{
    backend::CrosstermBackend,
    Terminal,
};

use std::io;

fn main() -> std::io::Result<()> {
    enable_raw_mode()?;

    let mut stdout = io::stdout();

    execute!(stdout, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let config = Config::load();
    let keymap = KeyMap::from_config(&config);

    let mut app = App::new();

    loop {
        terminal.draw(|f| {
            ui::draw(f, &app);
        })?;

        match input::read(&keymap)? {
            action::Action::Up => app.up(),
            action::Action::Down => app.down(),
            action::Action::Enter => app.enter(),
            action::Action::Back => app.back(),
            action::Action::Quit => app.quit(),
            action::Action::None => {}
        }

        if app.should_quit {
            break;
        }
    }

    disable_raw_mode()?;

    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen
    )?;

    Ok(())
}