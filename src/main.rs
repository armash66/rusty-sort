use std::env;
use std::io;
use std::io::Write;
use std::path::{Path, PathBuf};

mod organizer;
mod rules;

struct Config {
    dir: PathBuf,
    dry_run: bool,
}

fn main() {
    if let Err(err) = run() {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    }
}

fn run() -> io::Result<()> {
    let config = parse_args()?;
    validate_directory(&config.dir)?;

    let files = organizer::list_files(&config.dir)?;
    if files.is_empty() {
        println!("No files found.");
        return Ok(());
    }

    println!("Reading from: {}", config.dir.display());
    println!("Writing to: {}", config.dir.display());

    let plans = organizer::plan_moves(&config.dir, &files);

    println!("Plan:");
    for plan in &plans {
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

    print_summary(&plans);

    if config.dry_run {
        println!("Dry run: no files have been moved.");
        if !prompt_yes_no("Proceed with these moves? (y/n): ")? {
            println!("No changes made.");
            return Ok(());
        }
    }

    let result = organizer::apply_moves(&plans)?;
    println!("Moved {} file(s).", result.moved);
    if result.skipped > 0 {
        println!("Skipped {} file(s) (target exists).", result.skipped);
    }

    Ok(())
}

fn parse_args() -> io::Result<Config> {
    let mut dry_run = false;
    let mut path: Option<PathBuf> = None;

    for arg in env::args().skip(1) {
        if arg == "--dry-run" || arg == "-n" {
            dry_run = true;
        } else if path.is_none() {
            path = Some(PathBuf::from(arg));
        } else {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Usage: rusty-sort <folder_path> [--dry-run]",
            ));
        }
    };

    let Some(path) = path else {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Usage: rusty-sort <folder_path> [--dry-run]",
        ));
    };

    Ok(Config { dir: path, dry_run })
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

fn print_summary(plans: &[organizer::MovePlan]) {
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
    println!("Summary:");
    println!("Images: {}", images);
    println!("Documents: {}", documents);
    println!("Videos: {}", videos);
    println!("Audio: {}", audio);
    println!("Archives: {}", archives);
    println!("Others: {}", others);
    println!("Total: {}", total);
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
