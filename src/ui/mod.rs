pub mod components;

use ratatui::{
    layout::{Constraint, Direction, Layout},
    Frame,
};

use crate::core::app::App;
use components::{folder, pathbar, preview, status};

pub fn draw(frame: &mut Frame, app: &App) {
    let root = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(1), Constraint::Length(1)])
        .split(frame.area());

    let columns = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(root[0]);

    let left = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(1)])
        .split(columns[0]);

    pathbar::render(frame, left[0], app);
    folder::render(frame, left[1], app);
    preview::render(frame, columns[1], app);
    status::render(frame, root[1], app);
}
