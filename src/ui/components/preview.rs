use ratatui::{
    layout::Rect,
    style::Style,
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::core::app::App;

pub fn render(frame: &mut Frame, area: Rect, app: &App) {
    let text = app.preview.as_deref().unwrap_or("");
    let widget = Paragraph::new(text).block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(app.theme.border))
            .title(" Preview "),
    );
    frame.render_widget(widget, area);
}
