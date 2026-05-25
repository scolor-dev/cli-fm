mod app;
mod input;
mod theme;
mod ui;

use app::App;

use crossterm::{
    execute,
    terminal::{
        disable_raw_mode,
        enable_raw_mode,
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

    let mut app = App::new();

    loop {
        terminal.draw(|f| {
            ui::draw(f, &app);
        })?;

        match input::read()? {
            input::Action::Up => app.up(),

            input::Action::Down => app.down(),

            input::Action::Enter => app.enter(),

            input::Action::Back => app.back(),

            input::Action::Quit => app.quit(),

            input::Action::None => {}
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