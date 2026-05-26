use ratatui::{
    layout::{Layout, Constraint, Direction},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    Frame,
    style::{Style, Color},
};

use crate::app::{App, Mode};

const VIEW_HEIGHT: usize = 20;

pub fn draw(frame: &mut Frame, app: &App) {
    let outer = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(1),
            Constraint::Length(1),
        ])
        .split(frame.area());

    let columns = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ])
        .split(outer[0]);

    let left = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
        ])
        .split(columns[0]);

    // Path ボックス
    let (path_text, path_border_style) = if matches!(app.mode, Mode::PathInput) {
        (app.path_input.clone(), Style::default().fg(Color::Yellow))
    } else {
        (app.cwd.to_string_lossy().to_string(), Style::default().fg(app.theme.border))
    };
    let path_block = Paragraph::new(path_text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(path_border_style)
                .title(" Path "),
        );
    frame.render_widget(path_block, left[0]);

    // Folder ボックス（ファイル一覧）
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

    let folder_list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(app.theme.border))
                .title(" Folder "),
        )
        .highlight_symbol(">> ")
        .highlight_style(
            Style::default()
                .bg(Color::Blue)
                .fg(Color::Black),
        );

    let mut state = ListState::default();
    state.select(Some(app.cursor.saturating_sub(app.scroll)));

    frame.render_stateful_widget(folder_list, left[1], &mut state);

    // Preview ボックス
    let preview_text = app.preview.as_deref().unwrap_or("");
    let preview = Paragraph::new(preview_text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(app.theme.border))
                .title(" preview "),
        );
    frame.render_widget(preview, columns[1]);

    // ステータスバー
    let (status_text, status_style) = match app.mode {
        Mode::Normal => (
            "-- NORMAL --".to_string(),
            Style::default().fg(Color::Green),
        ),
        Mode::Command => (
            format!(":{}", app.command_input),
            Style::default().fg(Color::Yellow),
        ),
        Mode::Search => (
            format!("/{}", app.search_input),
            Style::default().fg(Color::Cyan),
        ),
        Mode::PathInput => (
            "-- PATH --".to_string(),
            Style::default().fg(Color::Yellow),
        ),
    };

    let status_bar = Paragraph::new(status_text).style(status_style);
    frame.render_widget(status_bar, outer[1]);
}
