use std::fs;
use std::io;
use std::path::{Path, PathBuf};

pub fn list_files(dir: &Path) -> io::Result<Vec<PathBuf>> {
    let mut files = Vec::new();

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let file_type = entry.file_type()?;
        if file_type.is_file() {
            files.push(entry.path());
        }
    }

    Ok(files)
}
