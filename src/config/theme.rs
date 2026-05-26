use ratatui::style::Color;

pub struct Theme {
    pub background: Color,
    pub foreground: Color,
    pub border: Color,
    pub highlight_bg: Color,
    pub highlight_fg: Color,
    pub directory: Color,
    pub file: Color,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            background: Color::Black,
            foreground: Color::White,
            border: Color::DarkGray,
            highlight_bg: Color::Blue,
            highlight_fg: Color::Black,
            directory: Color::Cyan,
            file: Color::White,
        }
    }
}
