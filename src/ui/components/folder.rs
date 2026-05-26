use ratatui::{
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders, List, ListItem, ListState},
    Frame,
};

use crate::core::{app::App, mode::ClipboardOp};

const VIEW_HEIGHT: usize = 20;

pub fn render(frame: &mut Frame, area: Rect, app: &App) {
    let clipboard_path = app.clipboard.as_ref().map(|(p, _)| p);

    let items: Vec<ListItem> = app
        .entries
        .iter()
        .skip(app.scroll)
        .take(VIEW_HEIGHT)
        .map(|path| {
            let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("?");
            let is_clipboard = clipboard_path.map_or(false, |cp| cp == path);
            let style = if is_clipboard {
                match app.clipboard.as_ref().map(|(_, op)| op) {
                    Some(ClipboardOp::Copy) => Style::default().fg(Color::Cyan),
                    Some(ClipboardOp::Move) => Style::default().fg(Color::Yellow),
                    None => Style::default(),
                }
            } else {
                Style::default()
            };
            ListItem::new(name.to_string()).style(style)
        })
        .collect();

    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(app.theme.border))
                .title(" Folder "),
        )
        .highlight_symbol(">> ")
        .highlight_style(Style::default().bg(Color::Blue).fg(Color::Black));

    let mut state = ListState::default();
    state.select(Some(app.cursor.saturating_sub(app.scroll)));

    frame.render_stateful_widget(list, area, &mut state);
}
