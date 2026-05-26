use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq)]
pub enum Mode {
    Normal,
    Command,
    Search,
    PathInput,
    Rename,
    NewEntry,
    Confirm,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ClipboardOp {
    Copy,
    Move,
}

#[derive(Debug, Clone)]
pub enum ConfirmAction {
    Delete(PathBuf),
}
