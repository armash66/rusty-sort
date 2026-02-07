use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use crate::rules::{self, Category};

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

pub fn move_files(dir: &Path, files: &[PathBuf]) -> io::Result<()> {
    for file in files {
        let category = rules::classify(file);
        let target_dir = dir.join(category_folder_name(category));
        if !target_dir.exists() {
            fs::create_dir_all(&target_dir)?;
        }

        let file_name = match file.file_name() {
            Some(name) => name,
            None => continue,
        };
        let target_path = target_dir.join(file_name);
        if target_path.exists() {
            return Err(io::Error::new(
                io::ErrorKind::AlreadyExists,
                format!("Target already exists: {}", target_path.display()),
            ));
        }

        fs::rename(file, target_path)?;
    }

    Ok(())
}

fn category_folder_name(category: Category) -> &'static str {
    match category {
        Category::Images => "Images",
        Category::Documents => "Documents",
        Category::Videos => "Videos",
        Category::Audio => "Audio",
        Category::Archives => "Archives",
        Category::Others => "Others",
    }
}
