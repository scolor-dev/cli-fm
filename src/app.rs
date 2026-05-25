use std::{
    fs,
    path::PathBuf,
};

use crate::theme::Theme;

const VIEW_HEIGHT: usize = 20;

pub struct App {
    pub cwd: PathBuf,
    pub entries: Vec<PathBuf>,

    pub cursor: usize,
    pub scroll: usize,

    pub should_quit: bool,

    pub theme: Theme,

    pub preview: Option<String>,
}

impl App {
    pub fn new() -> Self {
        let cwd = std::env::current_dir().unwrap();

        let mut app = Self {
            cwd,
            entries: Vec::new(),
            cursor: 0,
            scroll: 0,
            should_quit: false,
            theme: Theme::default(),
            preview: None,
        };

        app.refresh();
        app.update_preview();

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

    pub fn update_preview(&mut self) {
        let Some(path) = self.entries.get(self.cursor) else {
            self.preview = None;
            return;
        };

        if path.is_file() {
            self.preview = std::fs::read_to_string(path)
                .ok()
                .map(|s| s.chars().take(2000).collect());
        } else {
            self.preview = Some("[directory]".to_string());
        }
    }

    pub fn up(&mut self) {
        if self.cursor > 0 {
            self.cursor -= 1;
        }

        if self.cursor < self.scroll {
            self.scroll = self.cursor;
        }
    }

    pub fn down(&mut self) {
        if self.cursor + 1 < self.entries.len() {
            self.cursor += 1;
        }

        if self.cursor >= self.scroll + VIEW_HEIGHT {
            self.scroll = self.cursor - VIEW_HEIGHT + 1;
        }
    }

    pub fn enter(&mut self) {
        let Some(path) = self.entries.get(self.cursor) else {
            return;
        };

        if path.is_dir() {
            self.cwd = path.clone();
            self.cursor = 0;
            self.scroll = 0;
            self.refresh();
        }
    }

    pub fn back(&mut self) {
        if self.cwd.pop() {
            self.cursor = 0;
            self.scroll = 0;
            self.refresh();
        }
    }

    pub fn quit(&mut self) {
        self.should_quit = true;
    }
}