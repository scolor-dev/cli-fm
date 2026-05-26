use ratatui::{
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::core::{app::App, mode::Mode};

pub fn render(frame: &mut Frame, area: Rect, app: &App) {
    let (text, border_style) = if matches!(app.mode, Mode::PathInput) {
        (app.path_input.clone(), Style::default().fg(Color::Yellow))
    } else {
        (
            app.cwd.to_string_lossy().to_string(),
            Style::default().fg(app.theme.border),
        )
    };

    let widget = Paragraph::new(text).block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(border_style)
            .title(" Path "),
    );
    frame.render_widget(widget, area);
}
