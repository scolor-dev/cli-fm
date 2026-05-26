use std::{
    fs,
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};

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

/// `fs::rename` を試し、cross-device (os error 18) ならコピー+削除にフォールバック
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
