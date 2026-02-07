use std::env;
use std::io;
use std::path::{Path, PathBuf};

mod organizer;
mod rules;

fn main() {
    if let Err(err) = run() {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    }
}

fn run() -> io::Result<()> {
    let dir = parse_args()?;
    validate_directory(&dir)?;

    let files = organizer::list_files(&dir)?;
    if files.is_empty() {
        println!("No files found.");
        return Ok(());
    }

    organizer::move_files(&dir, &files)?;
    println!("Moved {} file(s).", files.len());

    Ok(())
}

fn parse_args() -> io::Result<PathBuf> {
    let mut args = env::args().skip(1);
    let path = match (args.next(), args.next()) {
        (Some(path), None) => PathBuf::from(path),
        _ => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Usage: rusty-sort <folder_path>",
            ))
        }
    };

    Ok(path)
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
