use std::{
    fs,
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};

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
            HistoryEntry::Created { path, .. } => {
                format!("create {}", file_name(path))
            }
            HistoryEntry::Deleted { original, .. } => {
                format!("delete {}", file_name(original))
            }
            HistoryEntry::Copied { dest, .. } => {
                format!("copy → {}", file_name(dest))
            }
            HistoryEntry::Moved { dest, .. } => {
                format!("move → {}", file_name(dest))
            }
            HistoryEntry::Renamed { new, .. } => {
                format!("rename → {}", file_name(new))
            }
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
                if src.is_dir() {
                    copy_dir_all(src, dest)
                } else {
                    fs::copy(src, dest).map(|_| ())
                }
            }
            HistoryEntry::Moved { src, dest } => move_any(src, dest),
            HistoryEntry::Renamed { old, new } => fs::rename(old, new),
        }
    }
}

pub fn make_trash_path(original: &Path) -> std::io::Result<PathBuf> {
    let trash_dir = std::env::temp_dir().join("cli-fm-trash");
    fs::create_dir_all(&trash_dir)?;

    let name = original
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_else(|| "unknown".to_string());

    let ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis();

    Ok(trash_dir.join(format!("{}_{}", ts, name)))
}

pub fn copy_dir_all(src: &Path, dest: &Path) -> std::io::Result<()> {
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

/// `fs::rename` を試し、cross-device (os error 18) なら copy+delete にフォールバック
pub fn move_any(src: &Path, dest: &Path) -> std::io::Result<()> {
    match fs::rename(src, dest) {
        Ok(()) => Ok(()),
        Err(e) if e.raw_os_error() == Some(18) => {
            if src.is_dir() {
                copy_dir_all(src, dest)?;
                fs::remove_dir_all(src)
            } else {
                fs::copy(src, dest)?;
                fs::remove_file(src)
            }
        }
        Err(e) => Err(e),
    }
}

fn remove_any(path: &Path) -> std::io::Result<()> {
    if path.is_dir() {
        fs::remove_dir_all(path)
    } else {
        fs::remove_file(path)
    }
}

fn file_name(path: &Path) -> String {
    path.file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_default()
}
