mod config;
mod core;
mod fs;
mod input;
mod ui;

use config::Config;
use core::{action::Action, app::App};
use input::{keymap::KeyMap, read};

use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
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
        terminal.draw(|f| ui::draw(f, &app))?;

        match read(&app.mode, &keymap)? {
            Action::Up => { app.up(); app.update_preview(); }
            Action::Down => { app.down(); app.update_preview(); }
            Action::Enter => { app.enter(); app.update_preview(); }
            Action::Back => { app.back(); app.update_preview(); }
            Action::Quit => app.quit(),
            Action::EnterCommandMode => app.enter_command_mode(),
            Action::EnterSearchMode => app.enter_search_mode(),
            Action::EnterPathMode => app.enter_path_mode(),
            Action::EnterNormalMode => app.enter_normal_mode(),
            Action::EnterRenameMode => app.enter_rename_mode(),
            Action::EnterNewEntryMode => app.enter_new_entry_mode(),
            Action::InputChar(c) => app.push_input_char(c),
            Action::InputBackspace => app.pop_input_char(),
            Action::InputSubmit => { app.submit(); app.update_preview(); }
            Action::TabComplete => app.tab_complete(),
            Action::Copy => app.copy_to_clipboard(),
            Action::Cut => app.cut_to_clipboard(),
            Action::Paste => app.paste(),
            Action::Delete => app.request_delete(),
            Action::ConfirmYes => { app.confirm_yes(); app.update_preview(); }
            Action::ConfirmNo => app.enter_normal_mode(),
            Action::Undo => app.undo(),
            Action::Redo => app.redo(),
            Action::None => {}
        }

        if app.should_quit { break; }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    Ok(())
}
