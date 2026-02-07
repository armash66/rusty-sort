use std::collections::HashSet;
use std::env;
use std::io;
use std::io::Write;
use std::path::{Path, PathBuf};

mod organizer;
mod rules;

struct Config {
    src: PathBuf,
    dest: PathBuf,
    dry_run: bool,
    recursive: bool,
    config_path: Option<PathBuf>,
}

fn main() {
    if let Err(err) = run() {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    }
}

fn run() -> io::Result<()> {
    let config = parse_args()?;
    validate_directory(&config.src)?;
    ensure_destination(&config.dest)?;

    let rules = load_rules(&config)?;

    let files = gather_files(&config)?;
    if files.is_empty() {
        println!("No files found.");
        return Ok(());
    }

    print_banner("Rusty Sort");
    println!("Read:  {}", config.src.display());
    println!("Write: {}", config.dest.display());

    let previous_state = load_previous_state(&config.src)?;
    if !previous_state.is_empty() {
        let (added, removed) = diff_state(&previous_state, &files, &config.src);
        print_section("Change Summary");
        println!("Added:   +{}", added);
        println!("Removed: -{}", removed);
    }

    let mut plans = organizer::plan_moves(&config.dest, &files, &rules);

    let scan_counts = count_files_by_category(&files, &rules);
    print_scan_summary(&scan_counts, files.len(), plans.len());
    print_plan("Plan", &plans);
    print_plan_summary(&plans);

    if config.dry_run {
        print_section("Dry Run");
        println!("Preview complete.");
        if !prompt_yes_no("Proceed with these moves? (y/n): ")? {
            println!("No changes made.");
            return Ok(());
        }

        let latest_files = gather_files(&config)?;
        let (added, removed) = diff_files(&files, &latest_files);
        if added > 0 || removed > 0 {
            println!(
                "Changes since preview: +{} new, -{} removed.",
                added, removed
            );
        }

        plans = organizer::plan_moves(&config.dest, &latest_files, &rules);
        if added > 0 || removed > 0 {
            let latest_counts = count_files_by_category(&latest_files, &rules);
            print_scan_summary(&latest_counts, latest_files.len(), plans.len());
            print_plan("Updated Plan", &plans);
            print_plan_summary(&plans);
        }
    }

    let result = organizer::apply_moves(&plans)?;
    print_section("Result");
    println!("Moved:   {}", result.moved);
    println!("Skipped: {}", result.skipped);
    if result.moved > 0 {
        print_section("Moved By Category");
        print_category_counts(&result.moved_by_category);
    }
    if result.skipped > 0 {
        print_section("Skipped By Category");
        print_category_counts(&result.skipped_by_category);
    }

    let final_files = gather_files(&config)?;
    save_state(&config.src, &final_files)?;

    Ok(())
}

fn parse_args() -> io::Result<Config> {
    let mut dry_run = false;
    let mut recursive = false;
    let mut src: Option<PathBuf> = None;
    let mut dest: Option<PathBuf> = None;
    let mut config_path: Option<PathBuf> = None;

    let mut args = env::args().skip(1).peekable();
    while let Some(arg) = args.next() {
        if arg == "--dry-run" || arg == "-n" {
            dry_run = true;
        } else if arg == "--recursive" || arg == "-r" {
            recursive = true;
        } else if arg == "--config" {
            let Some(value) = args.next() else {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "Usage: rusty-sort <source> [--to <dest>] [--config <file>] [--dry-run] [--recursive]",
                ));
            };
            config_path = Some(PathBuf::from(value));
        } else if arg == "--to" {
            let Some(value) = args.next() else {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "Usage: rusty-sort <source> [--to <dest>] [--config <file>] [--dry-run] [--recursive]",
                ));
            };
            dest = Some(PathBuf::from(value));
        } else if src.is_none() {
            src = Some(PathBuf::from(arg));
        } else {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Usage: rusty-sort <source> [--to <dest>] [--config <file>] [--dry-run] [--recursive]",
            ));
        }
    }

    let Some(src) = src else {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Usage: rusty-sort <source> [--to <dest>] [--config <file>] [--dry-run] [--recursive]",
        ));
    };

    let dest = dest.unwrap_or_else(|| src.clone());

    Ok(Config {
        src,
        dest,
        dry_run,
        recursive,
        config_path,
    })
}

fn prompt_yes_no(message: &str) -> io::Result<bool> {
    let mut input = String::new();

    loop {
        print!("{}", message);
        io::stdout().flush()?;
        input.clear();
        io::stdin().read_line(&mut input)?;

        match input.trim().to_ascii_lowercase().as_str() {
            "y" | "yes" => return Ok(true),
            "n" | "no" => return Ok(false),
            _ => println!("Please enter y or n."),
        }
    }
}

fn gather_files(config: &Config) -> io::Result<Vec<PathBuf>> {
    if config.recursive {
        organizer::list_files_recursive(&config.src)
    } else {
        organizer::list_files(&config.src)
    }
}

fn diff_files(before: &[PathBuf], after: &[PathBuf]) -> (usize, usize) {
    let before_set: HashSet<&PathBuf> = before.iter().collect();
    let after_set: HashSet<&PathBuf> = after.iter().collect();

    let added = after_set.difference(&before_set).count();
    let removed = before_set.difference(&after_set).count();

    (added, removed)
}

fn state_path(base_dir: &Path) -> PathBuf {
    base_dir.join(".rusty-sort-state.txt")
}

fn load_previous_state(base_dir: &Path) -> io::Result<Vec<PathBuf>> {
    let path = state_path(base_dir);
    if !path.exists() {
        return Ok(Vec::new());
    }

    let content = std::fs::read_to_string(path)?;
    let mut files = Vec::new();
    for line in content.lines() {
        if line.trim().is_empty() {
            continue;
        }
        files.push(base_dir.join(line));
    }
    Ok(files)
}

fn save_state(base_dir: &Path, files: &[PathBuf]) -> io::Result<()> {
    let mut lines = String::new();
    for file in files {
        if let Ok(rel) = file.strip_prefix(base_dir) {
            lines.push_str(&rel.to_string_lossy());
            lines.push('\n');
        }
    }
    std::fs::write(state_path(base_dir), lines)
}

fn diff_state(previous: &[PathBuf], current: &[PathBuf], base_dir: &Path) -> (usize, usize) {
    let prev_rel: HashSet<PathBuf> = previous
        .iter()
        .filter_map(|p| p.strip_prefix(base_dir).ok().map(|r| r.to_path_buf()))
        .collect();
    let curr_rel: HashSet<PathBuf> = current
        .iter()
        .filter_map(|p| p.strip_prefix(base_dir).ok().map(|r| r.to_path_buf()))
        .collect();

    let added = curr_rel.difference(&prev_rel).count();
    let removed = prev_rel.difference(&curr_rel).count();

    (added, removed)
}

fn print_plan_summary(plans: &[organizer::MovePlan]) {
    let mut images = 0usize;
    let mut documents = 0usize;
    let mut videos = 0usize;
    let mut audio = 0usize;
    let mut archives = 0usize;
    let mut others = 0usize;

    for plan in plans {
        match plan.category {
            rules::Category::Images => images += 1,
            rules::Category::Documents => documents += 1,
            rules::Category::Videos => videos += 1,
            rules::Category::Audio => audio += 1,
            rules::Category::Archives => archives += 1,
            rules::Category::Others => others += 1,
        }
    }

    let total = plans.len();
    print_section("Plan Summary");
    println!("Images: {}", images);
    println!("Documents: {}", documents);
    println!("Videos: {}", videos);
    println!("Audio: {}", audio);
    println!("Archives: {}", archives);
    println!("Others: {}", others);
    println!("Total: {}", total);
}

fn print_plan(title: &str, plans: &[organizer::MovePlan]) {
    print_section(title);
    for plan in plans {
        let exists_note = if plan.target.exists() {
            " (target exists)"
        } else {
            ""
        };
        println!(
            "[{}] {} -> {}{}",
            plan.category,
            plan.source.display(),
            plan.target.display(),
            exists_note
        );
    }
}

fn count_files_by_category(files: &[PathBuf], rules: &rules::Rules) -> CategoryCounts {
    let mut counts = CategoryCounts::default();
    for file in files {
        counts.inc(rules.classify(file));
    }
    counts
}

fn print_scan_summary(counts: &CategoryCounts, total: usize, to_move: usize) {
    let already_sorted = total.saturating_sub(to_move);
    print_section("Scan Summary");
    println!("Images: {}", counts.images);
    println!("Documents: {}", counts.documents);
    println!("Videos: {}", counts.videos);
    println!("Audio: {}", counts.audio);
    println!("Archives: {}", counts.archives);
    println!("Others: {}", counts.others);
    println!("Total files: {}", total);
    println!("Already sorted: {}", already_sorted);
    println!("To move: {}", to_move);
}

fn print_category_counts(counts: &organizer::CategoryCounts) {
    println!("Images: {}", counts.images);
    println!("Documents: {}", counts.documents);
    println!("Videos: {}", counts.videos);
    println!("Audio: {}", counts.audio);
    println!("Archives: {}", counts.archives);
    println!("Others: {}", counts.others);
}

#[derive(Default)]
struct CategoryCounts {
    images: usize,
    documents: usize,
    videos: usize,
    audio: usize,
    archives: usize,
    others: usize,
}

impl CategoryCounts {
    fn inc(&mut self, category: rules::Category) {
        match category {
            rules::Category::Images => self.images += 1,
            rules::Category::Documents => self.documents += 1,
            rules::Category::Videos => self.videos += 1,
            rules::Category::Audio => self.audio += 1,
            rules::Category::Archives => self.archives += 1,
            rules::Category::Others => self.others += 1,
        }
    }
}

fn validate_directory(path: &Path) -> io::Result<()> {
    if !path.exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "Path does not exist",
        ));
    }

    if !path.is_dir() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Path is not a directory",
        ));
    }

    Ok(())
}

fn ensure_destination(path: &Path) -> io::Result<()> {
    if !path.exists() {
        std::fs::create_dir_all(path)?;
        return Ok(());
    }
    if !path.is_dir() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Destination path is not a directory",
        ));
    }
    Ok(())
}

fn load_rules(config: &Config) -> io::Result<rules::Rules> {
    match &config.config_path {
        Some(path) => rules::Rules::from_config(path),
        None => Ok(rules::Rules::default()),
    }
}
fn print_banner(title: &str) {
    println!("== {} ==", title);
}

fn print_section(title: &str) {
    println!();
    println!("-- {} --", title);
}
