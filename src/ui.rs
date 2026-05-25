use ratatui::{
    layout::{Layout, Constraint, Direction},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    Frame,
    style::{Style, Color},
};

use crate::app::App;

const VIEW_HEIGHT: usize = 20;

pub fn draw(frame: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ])
        .split(frame.area());

    // 左：ファイル一覧
    let visible = app
        .entries
        .iter()
        .skip(app.scroll)
        .take(VIEW_HEIGHT);

    let items: Vec<ListItem> = visible
        .map(|path| {
            let name = path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("?");

            ListItem::new(name.to_string())
        })
        .collect();

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(app.theme.border)).title(app.cwd.to_string_lossy()))
        .highlight_symbol(">> ")
        .highlight_style(
            Style::default()
                .bg(Color::Blue)
                .fg(Color::Black),
        );

    let mut state = ListState::default();
    state.select(Some(app.cursor.saturating_sub(app.scroll)));

    frame.render_stateful_widget(list, chunks[0], &mut state);

    // 右：preview
    let preview_text = app.preview.as_deref().unwrap_or("");

    let preview = Paragraph::new(preview_text)
        .block(Block::default().borders(Borders::ALL).title("preview"));

    frame.render_widget(preview, chunks[1]);
}