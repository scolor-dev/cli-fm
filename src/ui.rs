use ratatui::{
    widgets::{
        Block,
        Borders,
        List,
        ListItem,
        ListState,
    },
    style::{
        Style,
    },
    Frame,
};

use crate::app::App;

const VIEW_HEIGHT: usize = 20;

pub fn draw(frame: &mut Frame, app: &App) {
    let visible_entries = app
        .entries
        .iter()
        .skip(app.scroll)
        .take(VIEW_HEIGHT);

    let items: Vec<ListItem> = visible_entries
        .map(|path| {
            let name = path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("?");

            if path.is_dir() {
                ListItem::new(name.to_string())
                    .style(
                        Style::default()
                            .fg(app.theme.directory)
                    )
            } else {
                ListItem::new(name.to_string())
                    .style(
                        Style::default()
                            .fg(app.theme.file)
                    )
            }
        })
        .collect();

    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(
                    Style::default()
                        .fg(app.theme.border)
                )
                .title(app.cwd.to_string_lossy())
        )
        .highlight_symbol(">> ")
        .highlight_style(
            Style::default()
                .bg(app.theme.highlight_bg)
                .fg(app.theme.highlight_fg),
        );

    let mut state = ListState::default();

    state.select(Some(
        app.cursor.saturating_sub(app.scroll)
    ));

    frame.render_stateful_widget(
        list,
        frame.area(),
        &mut state,
    );
}