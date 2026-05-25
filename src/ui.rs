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
        .map(|e| ListItem::new(e.as_str()))
        .collect();

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("mini-yazi"))
        .highlight_style(
            Style::default()
                .bg(Color::Blue)
                .fg(Color::Black),
        );

    let mut state = ListState::default();
    state.select(Some(app.cursor));

    frame.render_stateful_widget(list, frame.area(), &mut state);
}