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

        match input::read(&app.mode, &keymap)? {
            action::Action::Up => { app.up(); app.update_preview(); }
            action::Action::Down => { app.down(); app.update_preview(); }
            action::Action::Enter => { app.enter(); app.update_preview(); }
            action::Action::Back => { app.back(); app.update_preview(); }
            action::Action::Quit => app.quit(),
            action::Action::EnterCommandMode => app.enter_command_mode(),
            action::Action::EnterSearchMode => app.enter_search_mode(),
            action::Action::EnterPathMode => app.enter_path_mode(),
            action::Action::EnterNormalMode => app.enter_normal_mode(),
            action::Action::EnterRenameMode => app.enter_rename_mode(),
            action::Action::InputChar(c) => app.push_input_char(c),
            action::Action::InputBackspace => app.pop_input_char(),
            action::Action::InputSubmit => { app.submit(); app.update_preview(); }
            action::Action::TabComplete => app.tab_complete(),
            action::Action::Copy => app.copy_to_clipboard(),
            action::Action::Cut => app.cut_to_clipboard(),
            action::Action::Paste => app.paste(),
            action::Action::Delete => app.delete_entry(),
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
