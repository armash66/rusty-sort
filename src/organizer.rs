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

pub struct MovePlan {
    pub source: PathBuf,
    pub target: PathBuf,
    pub category: Category,
}

pub struct MoveResult {
    pub moved: usize,
    pub skipped: usize,
}

pub fn plan_moves(dir: &Path, files: &[PathBuf]) -> Vec<MovePlan> {
    let mut plans = Vec::new();

    for file in files {
        let category = rules::classify(file);
        let file_name = match file.file_name() {
            Some(name) => name,
            None => continue,
        };
        let target_dir = dir.join(category_folder_name(category));
        let target_path = target_dir.join(file_name);

        plans.push(MovePlan {
            source: file.clone(),
            target: target_path,
            category,
        });
    }

    plans
}

pub fn apply_moves(plans: &[MovePlan]) -> io::Result<MoveResult> {
    let mut moved = 0usize;
    let mut skipped = 0usize;

    for plan in plans {
        if plan.target.exists() {
            skipped += 1;
            continue;
        }
        if let Some(parent) = plan.target.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::rename(&plan.source, &plan.target)?;
        moved += 1;
    }

    Ok(MoveResult { moved, skipped })
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
