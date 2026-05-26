use std::{fs, path::PathBuf};

use crate::{
    config::theme::Theme,
    core::mode::{ClipboardOp, ConfirmAction, Mode},
    fs::{
        history::HistoryEntry,
        ops::{copy_dir_all, make_trash_path, move_any},
    },
};

const VIEW_HEIGHT: usize = 20;

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
    pub new_entry_input: String,

    pub completions: Vec<String>,
    pub completion_index: usize,
    pub completion_base: Option<String>,

    pub clipboard: Option<(PathBuf, ClipboardOp)>,
    pub confirm_action: Option<ConfirmAction>,
    pub status_message: Option<String>,

    pub undo_stack: Vec<HistoryEntry>,
    pub redo_stack: Vec<HistoryEntry>,
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
            new_entry_input: String::new(),
            completions: Vec::new(),
            completion_index: 0,
            completion_base: None,
            clipboard: None,
            confirm_action: None,
            status_message: None,
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
        };
        app.refresh();
        app.update_preview();
        app
    }

    // ── ディレクトリ ──────────────────────────────────────────────────

    pub fn refresh(&mut self) {
        self.entries = match fs::read_dir(&self.cwd) {
            Ok(rd) => rd.filter_map(|e| e.ok()).map(|e| e.path()).collect(),
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
            self.preview = fs::read_to_string(path)
                .ok()
                .map(|s| s.chars().take(2000).collect());
        } else {
            self.preview = Some("[directory]".to_string());
        }
    }

    // ── ナビゲーション ────────────────────────────────────────────────

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
        let Some(path) = self.entries.get(self.cursor) else { return };
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

    // ── モード遷移 ────────────────────────────────────────────────────

    pub fn enter_normal_mode(&mut self) {
        self.mode = Mode::Normal;
        self.confirm_action = None;
    }

    pub fn enter_command_mode(&mut self) {
        self.mode = Mode::Command;
        self.command_input.clear();
    }

    pub fn enter_search_mode(&mut self) {
        self.mode = Mode::Search;
        self.search_input.clear();
    }

    pub fn enter_path_mode(&mut self) {
        self.path_input = self.cwd.to_string_lossy().to_string();
        self.mode = Mode::PathInput;
    }

    pub fn enter_rename_mode(&mut self) {
        let Some(path) = self.entries.get(self.cursor) else { return };
        self.rename_input = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("")
            .to_string();
        self.mode = Mode::Rename;
    }

    pub fn enter_new_entry_mode(&mut self) {
        self.new_entry_input.clear();
        self.mode = Mode::NewEntry;
    }

    // ── テキスト入力 ──────────────────────────────────────────────────

    pub fn push_input_char(&mut self, c: char) {
        match self.mode {
            Mode::Command => self.command_input.push(c),
            Mode::Search => self.search_input.push(c),
            Mode::PathInput => {
                self.path_input.push(c);
                self.completion_base = None;
            }
            Mode::Rename => self.rename_input.push(c),
            Mode::NewEntry => self.new_entry_input.push(c),
            _ => {}
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
            Mode::NewEntry => { self.new_entry_input.pop(); }
            _ => {}
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
            Mode::Rename => self.submit_rename(),
            Mode::NewEntry => self.create_new_entry(),
            _ => self.mode = Mode::Normal,
        }
    }

    // ── ファイル操作 ──────────────────────────────────────────────────

    pub fn copy_to_clipboard(&mut self) {
        let Some(path) = self.entries.get(self.cursor) else { return };
        let name = file_name_str(path);
        self.clipboard = Some((path.clone(), ClipboardOp::Copy));
        self.status_message = Some(format!("Copied: {}", name));
    }

    pub fn cut_to_clipboard(&mut self) {
        let Some(path) = self.entries.get(self.cursor) else { return };
        let name = file_name_str(path);
        self.clipboard = Some((path.clone(), ClipboardOp::Move));
        self.status_message = Some(format!("Cut: {}", name));
    }

    pub fn paste(&mut self) {
        let Some((src, op)) = self.clipboard.clone() else { return };
        let Some(raw_name) = src.file_name() else { return };
        let name = raw_name.to_string_lossy().into_owned();
        let dest = self.cwd.join(raw_name);

        if dest == src {
            self.status_message = Some("Already in this directory".to_string());
            return;
        }

        let result = match &op {
            ClipboardOp::Copy => {
                if src.is_dir() { copy_dir_all(&src, &dest) }
                else { fs::copy(&src, &dest).map(|_| ()) }
            }
            ClipboardOp::Move => move_any(&src, &dest),
        };

        match result {
            Ok(()) => {
                let entry = match op {
                    ClipboardOp::Copy => HistoryEntry::Copied { src, dest: dest.clone() },
                    ClipboardOp::Move => {
                        self.clipboard = None;
                        HistoryEntry::Moved { src, dest: dest.clone() }
                    }
                };
                self.push_history(entry);
                self.status_message = Some(format!("Pasted: {}", name));
                self.refresh();
                self.update_preview();
            }
            Err(e) => self.status_message = Some(format!("Error: {}", e)),
        }
    }

    pub fn request_delete(&mut self) {
        let Some(path) = self.entries.get(self.cursor).cloned() else { return };
        self.confirm_action = Some(ConfirmAction::Delete(path));
        self.mode = Mode::Confirm;
    }

    pub fn confirm_yes(&mut self) {
        if let Some(ConfirmAction::Delete(path)) = self.confirm_action.take() {
            self.execute_delete(path);
        }
        self.mode = Mode::Normal;
    }

    fn execute_delete(&mut self, path: PathBuf) {
        let name = file_name_str(&path);
        match make_trash_path(&path) {
            Ok(trash) => match move_any(&path, &trash) {
                Ok(()) => {
                    self.push_history(HistoryEntry::Deleted { original: path, trash });
                    self.status_message = Some(format!("Deleted: {}", name));
                    self.refresh();
                    self.update_preview();
                }
                Err(e) => self.status_message = Some(format!("Error: {}", e)),
            },
            Err(e) => self.status_message = Some(format!("Error creating trash: {}", e)),
        }
    }

    fn submit_rename(&mut self) {
        let Some(src) = self.entries.get(self.cursor).cloned() else {
            self.mode = Mode::Normal;
            return;
        };
        let new_name = self.rename_input.trim().to_string();
        let old_name = file_name_str(&src);

        if new_name.is_empty() || new_name == old_name {
            self.mode = Mode::Normal;
            return;
        }

        let dest = self.cwd.join(&new_name);
        match fs::rename(&src, &dest) {
            Ok(()) => {
                self.push_history(HistoryEntry::Renamed { old: src, new: dest });
                self.status_message = Some(format!("Renamed to: {}", new_name));
                self.refresh();
                self.update_preview();
            }
            Err(e) => self.status_message = Some(format!("Error: {}", e)),
        }
        self.mode = Mode::Normal;
    }

    fn create_new_entry(&mut self) {
        let input = self.new_entry_input.trim().to_string();
        if input.is_empty() {
            self.mode = Mode::Normal;
            return;
        }

        let is_dir = input.ends_with('/');
        let rel: PathBuf = input.trim_end_matches('/').into();
        let target = self.cwd.join(&rel);

        let result: std::io::Result<()> = if is_dir {
            fs::create_dir_all(&target)
        } else {
            (|| {
                if let Some(parent) = target.parent() {
                    fs::create_dir_all(parent)?;
                }
                fs::File::create(&target).map(|_| ())
            })()
        };

        match result {
            Ok(()) => {
                self.push_history(HistoryEntry::Created { path: target, is_dir });
                self.status_message = Some(format!("Created: {}", rel.display()));
                self.refresh();
                self.update_preview();
            }
            Err(e) => self.status_message = Some(format!("Error: {}", e)),
        }
        self.mode = Mode::Normal;
    }

    // ── Undo / Redo ───────────────────────────────────────────────────

    pub fn undo(&mut self) {
        let Some(entry) = self.undo_stack.pop() else {
            self.status_message = Some("Nothing to undo".to_string());
            return;
        };
        let desc = entry.description();
        match entry.undo() {
            Ok(()) => {
                self.status_message = Some(format!("Undone: {}", desc));
                self.redo_stack.push(entry);
                self.refresh();
                self.update_preview();
            }
            Err(e) => self.status_message = Some(format!("Undo failed: {}", e)),
        }
    }

    pub fn redo(&mut self) {
        let Some(entry) = self.redo_stack.pop() else {
            self.status_message = Some("Nothing to redo".to_string());
            return;
        };
        let desc = entry.description();
        match entry.redo() {
            Ok(()) => {
                self.status_message = Some(format!("Redone: {}", desc));
                self.undo_stack.push(entry);
                self.refresh();
                self.update_preview();
            }
            Err(e) => self.status_message = Some(format!("Redo failed: {}", e)),
        }
    }

    fn push_history(&mut self, entry: HistoryEntry) {
        self.undo_stack.push(entry);
        self.redo_stack.clear();
    }
}

fn file_name_str(path: &std::path::Path) -> String {
    path.file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("")
        .to_string()
}

fn path_completions(input: &str) -> Vec<String> {
    let (base_dir, prefix) = match input.rfind('/') {
        Some(pos) => (&input[..=pos], &input[pos + 1..]),
        None => ("./", input),
    };
    let Ok(entries) = fs::read_dir(base_dir) else { return Vec::new() };
    let mut matches: Vec<String> = entries
        .filter_map(|e| e.ok())
        .filter_map(|e| {
            let name = e.file_name().to_string_lossy().to_string();
            if !name.starts_with(prefix) { return None; }
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
