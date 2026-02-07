use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use crate::rules::{Category, Rules};

pub fn list_files(dir: &Path) -> io::Result<Vec<PathBuf>> {
    let mut files = Vec::new();

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let file_type = entry.file_type()?;
        if file_type.is_file() {
            let path = entry.path();
            if is_state_file(&path) {
                continue;
            }
            files.push(path);
        }
    }

    Ok(files)
}

pub fn list_files_recursive(dir: &Path) -> io::Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    collect_files_recursive(dir, &mut files)?;
    Ok(files)
}

fn collect_files_recursive(dir: &Path, files: &mut Vec<PathBuf>) -> io::Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let file_type = entry.file_type()?;
        let path = entry.path();
        if file_type.is_file() {
            if is_state_file(&path) {
                continue;
            }
            files.push(path);
        } else if file_type.is_dir() {
            collect_files_recursive(&path, files)?;
        }
    }

    Ok(())
}

fn is_state_file(path: &Path) -> bool {
    matches!(path.file_name().and_then(|n| n.to_str()), Some(".rusty-sort-state.txt"))
}

pub struct MovePlan {
    pub source: PathBuf,
    pub target: PathBuf,
    pub category: Category,
}

pub struct MoveResult {
    pub moved: usize,
    pub skipped: usize,
    pub moved_by_category: CategoryCounts,
    pub skipped_by_category: CategoryCounts,
}

#[derive(Default, Clone, Copy)]
pub struct CategoryCounts {
    pub images: usize,
    pub documents: usize,
    pub videos: usize,
    pub audio: usize,
    pub archives: usize,
    pub others: usize,
}

impl CategoryCounts {
    fn inc(&mut self, category: Category) {
        match category {
            Category::Images => self.images += 1,
            Category::Documents => self.documents += 1,
            Category::Videos => self.videos += 1,
            Category::Audio => self.audio += 1,
            Category::Archives => self.archives += 1,
            Category::Others => self.others += 1,
        }
    }
}

pub fn plan_moves(dest_dir: &Path, files: &[PathBuf], rules: &Rules) -> Vec<MovePlan> {
    let mut plans = Vec::new();

    for file in files {
        let category = rules.classify(file);
        let file_name = match file.file_name() {
            Some(name) => name,
            None => continue,
        };
        let target_dir = dest_dir.join(category_folder_name(category));
        let target_path = target_dir.join(file_name);

        if *file == target_path {
            continue;
        }

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
    let mut moved_by_category = CategoryCounts::default();
    let mut skipped_by_category = CategoryCounts::default();

    for plan in plans {
        if plan.target.exists() {
            skipped += 1;
            skipped_by_category.inc(plan.category);
            continue;
        }
        if let Some(parent) = plan.target.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::rename(&plan.source, &plan.target)?;
        moved += 1;
        moved_by_category.inc(plan.category);
    }

    Ok(MoveResult {
        moved,
        skipped,
        moved_by_category,
        skipped_by_category,
    })
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
