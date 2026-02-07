# rusty-sort

A simple Rust CLI that scans a directory and organizes files into category folders by extension.

## Features

- Safe file organization (no overwrites; existing targets are skipped).
- `--dry-run` preview with confirmation.
- Optional recursive scan.
- Optional source → destination mode.
- Custom extension rules via config file.
- Clear summaries and change tracking between runs.

## Requirements

- Rust (stable, edition 2021)
- Windows (MSVC or GNU toolchain)

## Install / Build

```powershell
cargo build
```

## Basic Usage

Organize files inside the same folder:

```powershell
cargo run -- <source>
```

Preview only:

```powershell
cargo run -- <source> --dry-run
```

Short flag:

```powershell
cargo run -- <source> -n
```

Recursive scan:

```powershell
cargo run -- <source> --recursive
```

Short flag:

```powershell
cargo run -- <source> -r
```

## Source → Destination

Move files from one folder into categorized folders in another:

```powershell
cargo run -- <source> --to <dest>
```

Example:

```powershell
cargo run -- .\test-data-2 --to .\test-data --recursive --dry-run
```

## Custom Rules

Create a rules file (e.g. `rules.txt`) and pass it with `--config`:

```powershell
cargo run -- <source> --config .\rules.txt
```

Format (one category per line):

```
Images=jpg,jpeg,png,webp,heic
Documents=pdf,docx,md,txt,epub,xlsx,pptx,csv,log,yaml,yml,json
Videos=mp4,mkv,mov
Audio=mp3,wav,flac
Archives=zip,7z,tar,gz
Others=
```

Notes:

- Category names are case-insensitive.
- Extensions may include or omit the leading dot.
- Lines starting with `#` are comments.

## Output Notes

- The tool ignores subfolders by default unless `--recursive` is set.
- It records the last scan in `.rusty-sort-state.txt` in the **source** folder to report changes between runs.
- If you want it to continuously watch a folder and auto-sort new files, that would be a separate "watch mode" feature.

## Project Structure

```
rusty-sort/
├── Cargo.toml
├── src/
│   ├── main.rs
│   ├── organizer.rs
│   └── rules.rs
├── rules.txt
└── README.md
```
