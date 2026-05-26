use ratatui::{
    layout::Rect,
    style::{Color, Style},
    widgets::Paragraph,
    Frame,
};

use crate::core::{
    app::App,
    mode::{ClipboardOp, ConfirmAction, Mode},
};

pub fn render(frame: &mut Frame, area: Rect, app: &App) {
    let (text, style) = build(app);
    frame.render_widget(Paragraph::new(text).style(style), area);
}

fn build(app: &App) -> (String, Style) {
    match &app.mode {
        Mode::Normal => {
            let text = if let Some(msg) = &app.status_message {
                format!("-- NORMAL -- | {}", msg)
            } else if let Some((path, op)) = &app.clipboard {
                let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
                let op_str = match op {
                    ClipboardOp::Copy => "copy",
                    ClipboardOp::Move => "move",
                };
                format!("-- NORMAL -- | [{}] {}", op_str, name)
            } else {
                "-- NORMAL --".to_string()
            };
            (text, Style::default().fg(Color::Green))
        }
        Mode::Command => (
            format!(":{}", app.command_input),
            Style::default().fg(Color::Yellow),
        ),
        Mode::Search => (
            format!("/{}", app.search_input),
            Style::default().fg(Color::Cyan),
        ),
        Mode::PathInput => (
            format!("-- PATH -- {}", app.path_input),
            Style::default().fg(Color::Yellow),
        ),
        Mode::Rename => (
            format!("-- RENAME -- {}", app.rename_input),
            Style::default().fg(Color::Magenta),
        ),
        Mode::NewEntry => (
            format!("-- NEW -- {} (end with / for dir)", app.new_entry_input),
            Style::default().fg(Color::LightGreen),
        ),
        Mode::Confirm => {
            let target = app.confirm_action.as_ref().map(|a| match a {
                ConfirmAction::Delete(p) => {
                    p.file_name().and_then(|n| n.to_str()).unwrap_or("").to_string()
                }
            }).unwrap_or_default();
            (
                format!("Delete \"{}\"? [y/n]", target),
                Style::default().fg(Color::Red),
            )
        }
    }
}
