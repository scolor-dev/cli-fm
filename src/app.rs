use std::{
    fs,
    path::{Path, PathBuf},
};

use crate::theme::Theme;

const VIEW_HEIGHT: usize = 20;

#[derive(Debug, Clone, PartialEq)]
pub enum Mode {
    Normal,
    Command,
    Search,
    PathInput,
    Rename,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ClipboardOp {
    Copy,
    Move,
}

pub struct App {
    pub cwd: PathBuf,
    pub entries: Vec<PathBuf>,

    pub cursor: usize,
    pub scroll: usize,

    pub should_quit: bool,

    pub theme: Theme,

    pub preview: Option<String>,

    pub mode: Mode,
    pub command_input: String,
    pub search_input: String,
    pub path_input: String,
    pub rename_input: String,

    pub completions: Vec<String>,
    pub completion_index: usize,
    pub completion_base: Option<String>,

    pub clipboard: Option<(PathBuf, ClipboardOp)>,
    pub status_message: Option<String>,
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
            mode: Mode::Normal,
            command_input: String::new(),
            search_input: String::new(),
            path_input: String::new(),
            rename_input: String::new(),
            completions: Vec::new(),
            completion_index: 0,
            completion_base: None,
            clipboard: None,
            status_message: None,
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

    pub fn enter_command_mode(&mut self) {
        self.mode = Mode::Command;
        self.command_input.clear();
    }

    pub fn enter_search_mode(&mut self) {
        self.mode = Mode::Search;
        self.search_input.clear();
    }

    pub fn enter_normal_mode(&mut self) {
        self.mode = Mode::Normal;
    }

    pub fn enter_path_mode(&mut self) {
        self.path_input = self.cwd.to_string_lossy().to_string();
        self.mode = Mode::PathInput;
    }

    pub fn enter_rename_mode(&mut self) {
        let Some(path) = self.entries.get(self.cursor) else {
            return;
        };
        self.rename_input = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("")
            .to_string();
        self.mode = Mode::Rename;
    }

    pub fn copy_to_clipboard(&mut self) {
        let Some(path) = self.entries.get(self.cursor) else {
            return;
        };
        self.clipboard = Some((path.clone(), ClipboardOp::Copy));
        self.status_message = Some(format!(
            "Copied: {}",
            path.file_name().and_then(|n| n.to_str()).unwrap_or("")
        ));
    }

    pub fn cut_to_clipboard(&mut self) {
        let Some(path) = self.entries.get(self.cursor) else {
            return;
        };
        self.clipboard = Some((path.clone(), ClipboardOp::Move));
        self.status_message = Some(format!(
            "Cut: {}",
            path.file_name().and_then(|n| n.to_str()).unwrap_or("")
        ));
    }

    pub fn paste(&mut self) {
        let Some((src, op)) = self.clipboard.clone() else {
            return;
        };

        let Some(name) = src.file_name() else {
            return;
        };

        let dest = self.cwd.join(name);

        if dest == src {
            self.status_message = Some("Already in this directory".to_string());
            return;
        }

        let result = match op {
            ClipboardOp::Copy => {
                if src.is_dir() {
                    copy_dir_all(&src, &dest)
                } else {
                    fs::copy(&src, &dest).map(|_| ())
                }
            }
            ClipboardOp::Move => fs::rename(&src, &dest).map_err(Into::into),
        };

        match result {
            Ok(()) => {
                if op == ClipboardOp::Move {
                    self.clipboard = None;
                }
                self.status_message = Some(format!(
                    "Pasted: {}",
                    name.to_string_lossy()
                ));
                self.refresh();
                self.update_preview();
            }
            Err(e) => {
                self.status_message = Some(format!("Error: {}", e));
            }
        }
    }

    pub fn delete_entry(&mut self) {
        let Some(path) = self.entries.get(self.cursor).cloned() else {
            return;
        };

        let name = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("")
            .to_string();

        let result = if path.is_dir() {
            fs::remove_dir_all(&path)
        } else {
            fs::remove_file(&path)
        };

        match result {
            Ok(()) => {
                self.status_message = Some(format!("Deleted: {}", name));
                self.refresh();
                self.update_preview();
            }
            Err(e) => {
                self.status_message = Some(format!("Error: {}", e));
            }
        }
    }

    pub fn push_input_char(&mut self, c: char) {
        match self.mode {
            Mode::Command => self.command_input.push(c),
            Mode::Search => self.search_input.push(c),
            Mode::PathInput => {
                self.path_input.push(c);
                self.completion_base = None;
            }
            Mode::Rename => self.rename_input.push(c),
            Mode::Normal => {}
        }
    }

    pub fn pop_input_char(&mut self) {
        match self.mode {
            Mode::Command => { self.command_input.pop(); }
            Mode::Search => { self.search_input.pop(); }
            Mode::PathInput => {
                self.pop_path_segment();
                self.completion_base = None;
            }
            Mode::Rename => { self.rename_input.pop(); }
            Mode::Normal => {}
        }
    }

    fn pop_path_segment(&mut self) {
        if self.path_input.ends_with('/') && self.path_input.len() > 1 {
            self.path_input.pop();
        } else if let Some(pos) = self.path_input.rfind('/') {
            self.path_input.truncate(pos + 1);
        } else {
            self.path_input.clear();
        }
    }

    pub fn tab_complete(&mut self) {
        let in_cycle = self.completion_base.is_some()
            && self.completions.iter().any(|c| c == &self.path_input);

        if in_cycle {
            self.completion_index = (self.completion_index + 1) % self.completions.len();
        } else {
            self.completion_base = Some(self.path_input.clone());
            self.completions = path_completions(&self.path_input);
            self.completion_index = 0;
        }

        if let Some(c) = self.completions.get(self.completion_index) {
            self.path_input = c.clone();
        }
    }

    pub fn submit(&mut self) {
        match self.mode {
            Mode::PathInput => {
                let path = PathBuf::from(&self.path_input);
                if path.is_dir() {
                    self.cwd = path;
                    self.cursor = 0;
                    self.scroll = 0;
                    self.refresh();
                    self.update_preview();
                }
                self.mode = Mode::Normal;
            }
            Mode::Rename => {
                self.submit_rename();
            }
            _ => self.mode = Mode::Normal,
        }
    }

    fn submit_rename(&mut self) {
        let Some(src) = self.entries.get(self.cursor).cloned() else {
            self.mode = Mode::Normal;
            return;
        };

        let new_name = self.rename_input.trim().to_string();
        if new_name.is_empty() || new_name == src.file_name().and_then(|n| n.to_str()).unwrap_or("") {
            self.mode = Mode::Normal;
            return;
        }

        let dest = self.cwd.join(&new_name);
        match fs::rename(&src, &dest) {
            Ok(()) => {
                self.status_message = Some(format!("Renamed to: {}", new_name));
                self.refresh();
                self.update_preview();
            }
            Err(e) => {
                self.status_message = Some(format!("Error: {}", e));
            }
        }
        self.mode = Mode::Normal;
    }
}

fn copy_dir_all(src: &Path, dest: &Path) -> std::io::Result<()> {
    fs::create_dir_all(dest)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let dest_path = dest.join(entry.file_name());
        if entry.path().is_dir() {
            copy_dir_all(&entry.path(), &dest_path)?;
        } else {
            fs::copy(entry.path(), dest_path)?;
        }
    }
    Ok(())
}

fn path_completions(input: &str) -> Vec<String> {
    let (base_dir, prefix) = match input.rfind('/') {
        Some(pos) => (&input[..=pos], &input[pos + 1..]),
        None => ("./", input),
    };

    let Ok(entries) = fs::read_dir(base_dir) else {
        return Vec::new();
    };

    let mut matches: Vec<String> = entries
        .filter_map(|e| e.ok())
        .filter_map(|e| {
            let name = e.file_name().to_string_lossy().to_string();
            if !name.starts_with(prefix) {
                return None;
            }
            if e.path().is_dir() {
                Some(format!("{}{}/", base_dir, name))
            } else {
                Some(format!("{}{}", base_dir, name))
            }
        })
        .collect();

    matches.sort();
    matches
}
