# rusty-sort

A simple Rust CLI that organizes files in a folder into category subfolders by extension.

## Features

- Scans a single directory (no recursion by default).
- Categorizes by extension: Images, Documents, Videos, Audio, Archives, Others.
- Creates category folders only when needed.
- Safe by default: no overwrites (existing targets are skipped).
- `--dry-run` preview with optional confirmation.
- Summary counts per category.
- Moved/skipped counts per category.

## Requirements

- Rust (stable, edition 2021)
- Windows (MSVC or GNU toolchain)

## Build

```powershell
cargo build
```

## Usage

```powershell
cargo run -- <source>
```

Preview only:

```powershell
cargo run -- <source> --dry-run
```

Write to a different folder:

```powershell
cargo run -- <source> --to <dest>
```

Recursive scan:

```powershell
cargo run -- <source> --recursive
```

Short flag:

```powershell
cargo run -- <source> -n
```

Short flag for recursive:

```powershell
cargo run -- <source> -r
```

## Example

```powershell
cargo run -- .\test-data --dry-run
```

Source to destination:

```powershell
cargo run -- .\test-data-2 --to .\test-data --recursive --dry-run
```

Sample output:

```
Reading from: .\test-data
Writing to: .\test-data
Plan:
[Images] .\test-data\photo.jpg -> .\test-data\Images\photo.jpg
[Documents] .\test-data\readme.txt -> .\test-data\Documents\readme.txt
Summary:
Images: 1
Documents: 1
Videos: 0
Audio: 0
Archives: 0
Others: 0
Total: 2
Dry run: no files have been moved.
Proceed with these moves? (y/n):
```

## Notes

- The tool ignores subfolders by default (use `--recursive` to include them).
- If a target file already exists, that file is skipped.
- If you want it to continuously watch a folder and auto-sort new files, that would be a separate "watch mode" feature.
- The tool stores the last scan in `.rusty-sort-state.txt` to report changes between runs.

## Project Structure

```
rusty-sort/
├── Cargo.toml
├── src/
│   ├── main.rs
│   ├── organizer.rs
│   └── rules.rs
└── README.md
```
