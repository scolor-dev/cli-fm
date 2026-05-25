use std::path::Path;

use ratatui::{
    widgets::{Block, Borders, List, ListItem, ListState},
    style::{Color, Style},
    Frame,
};

use crate::app::App;

pub fn draw(frame: &mut Frame, app: &App) {
    let items: Vec<ListItem> = app
        .entries
        .iter()
        .map(|path| {
            let name = path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("?");

            ListItem::new(name.to_string())
        })
        .collect();

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title(app.cwd.to_string_lossy()))
        .highlight_style(
            Style::default()
                .bg(Color::Blue)
                .fg(Color::Black),
        );

    let mut state = ListState::default();
    state.select(Some(app.cursor));

    frame.render_stateful_widget(list, frame.area(), &mut state);
}