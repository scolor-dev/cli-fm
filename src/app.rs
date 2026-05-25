use std::{
    fs,
    path::PathBuf,
};

pub struct App {
    pub cwd: PathBuf,
    pub entries: Vec<PathBuf>,
    pub cursor: usize,
    pub should_quit: bool,
}

impl App {
    pub fn new() -> Self {
        let cwd = std::env::current_dir().unwrap();

        let mut app = Self {
            cwd,
            entries: Vec::new(),
            cursor: 0,
            should_quit: false,
        };

        app.refresh();

        app
    }

    pub fn refresh(&mut self) {
        self.entries = match fs::read_dir(&self.cwd) {
            Ok(read_dir) => read_dir
                .filter_map(|e| e.ok())
                .map(|e| e.path())
                .collect(),

            Err(_) => Vec::new(),
        };

        self.entries.sort();

        if self.cursor >= self.entries.len() {
            self.cursor = self.entries.len().saturating_sub(1);
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

    pub fn enter(&mut self) {
        let Some(path) = self.entries.get(self.cursor) else {
            return;
        };

        if path.is_dir() {
            self.cwd = path.clone();
            self.cursor = 0;
            self.refresh();
        }
    }

    pub fn back(&mut self) {
        if self.cwd.pop() {
            self.cursor = 0;
            self.refresh();
        }
    }

    pub fn quit(&mut self) {
        self.should_quit = true;
    }
}