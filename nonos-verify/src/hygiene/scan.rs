use std::path::Path;

use super::patterns::line_violation;
use super::roots::{is_rust, skip};

pub fn scan_path(path: &Path, out: &mut Vec<String>) -> std::io::Result<()> {
    if skip(path) || !is_rust(path) {
        return Ok(());
    }
    let text = std::fs::read_to_string(path)?;
    for (idx, line) in text.lines().enumerate() {
        if let Some(kind) = line_violation(line) {
            out.push(format!("{}:{}:{kind}", path.display(), idx + 1));
        }
    }
    Ok(())
}

pub fn walk_dir(dir: &Path, out: &mut Vec<String>) -> std::io::Result<()> {
    if !dir.exists() {
        return Ok(());
    }
    for entry in std::fs::read_dir(dir)? {
        let path = entry?.path();
        if path.is_dir() {
            walk_dir(&path, out)?;
        } else {
            scan_path(&path, out)?;
        }
    }
    Ok(())
}
