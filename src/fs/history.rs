use std::{fs, path::{Path, PathBuf}};

use crate::fs::ops::{copy_dir_all, move_any};

#[derive(Debug, Clone)]
pub enum HistoryEntry {
    Created { path: PathBuf, is_dir: bool },
    Deleted { original: PathBuf, trash: PathBuf },
    Copied { src: PathBuf, dest: PathBuf },
    Moved { src: PathBuf, dest: PathBuf },
    Renamed { old: PathBuf, new: PathBuf },
}

impl HistoryEntry {
    pub fn description(&self) -> String {
        match self {
            HistoryEntry::Created { path, .. } => format!("create {}", file_name(path)),
            HistoryEntry::Deleted { original, .. } => format!("delete {}", file_name(original)),
            HistoryEntry::Copied { dest, .. } => format!("copy → {}", file_name(dest)),
            HistoryEntry::Moved { dest, .. } => format!("move → {}", file_name(dest)),
            HistoryEntry::Renamed { new, .. } => format!("rename → {}", file_name(new)),
        }
    }

    pub fn undo(&self) -> std::io::Result<()> {
        match self {
            HistoryEntry::Created { path, .. } => remove_any(path),
            HistoryEntry::Deleted { original, trash } => move_any(trash, original),
            HistoryEntry::Copied { dest, .. } => remove_any(dest),
            HistoryEntry::Moved { src, dest } => move_any(dest, src),
            HistoryEntry::Renamed { old, new } => fs::rename(new, old),
        }
    }

    pub fn redo(&self) -> std::io::Result<()> {
        match self {
            HistoryEntry::Created { path, is_dir } => {
                if *is_dir {
                    fs::create_dir_all(path)
                } else {
                    if let Some(parent) = path.parent() {
                        fs::create_dir_all(parent)?;
                    }
                    fs::File::create(path)?;
                    Ok(())
                }
            }
            HistoryEntry::Deleted { original, trash } => {
                if let Some(parent) = trash.parent() {
                    fs::create_dir_all(parent)?;
                }
                move_any(original, trash)
            }
            HistoryEntry::Copied { src, dest } => {
                if src.is_dir() { copy_dir_all(src, dest) }
                else { fs::copy(src, dest).map(|_| ()) }
            }
            HistoryEntry::Moved { src, dest } => move_any(src, dest),
            HistoryEntry::Renamed { old, new } => fs::rename(old, new),
        }
    }
}

fn remove_any(path: &Path) -> std::io::Result<()> {
    if path.is_dir() { fs::remove_dir_all(path) }
    else { fs::remove_file(path) }
}

fn file_name(path: &Path) -> String {
    path.file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_default()
}
