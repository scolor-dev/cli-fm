pub struct App {
    pub entries: Vec<String>,
    pub cursor: usize,
    pub should_quit: bool,
}

impl App {
    pub fn new() -> Self {
        Self {
            entries: vec![
                "src".into(),
                "Cargo.toml".into(),
                "README.md".into(),
            ],
            cursor: 0,
            should_quit: false,
        }
    }

    pub fn up(&mut self) {
        if self.cursor > 0 {
            self.cursor -= 1;
        }
    }

    pub fn down(&mut self) {
        if self.cursor + 1 < self.entries.len() {
            self.cursor += 1;
        }
    }

    pub fn quit(&mut self) {
        self.should_quit = true;
    }
}