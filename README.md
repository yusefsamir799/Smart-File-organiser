# Smart Organizer

A fast, configurable CLI tool that sorts files into category folders by extension. Built in Rust.

```
Downloads/                          Downloads/
├── vacation.jpg                    ├── Images/
├── report.pdf                      │   ├── vacation.jpg
├── song.mp3          ──────►       │   └── screenshot.png
├── screenshot.png                  ├── Documents/
├── movie.mp4                       │   └── report.pdf
└── archive.zip                     ├── Music/
                                    │   └── song.mp3
                                    ├── Videos/
                                    │   └── movie.mp4
                                    ├── Archives/
                                    │   └── archive.zip
                                    └── organizer_log.txt
```

## Installation

Requires [Rust](https://rustup.rs/) 1.70+.

```bash
git clone https://github.com/yourusername/smart-organizer.git
cd smart-organizer
cargo build --release
```

The binary will be at `target/release/smart-organizer`.

## Usage

Always preview before organizing:

```bash
# Preview changes (nothing gets moved)
cargo run --release -- --dry-run --path ~/Downloads

# Apply for real
cargo run --release -- --path ~/Downloads
```

### Options

| Flag | Description |
|------|-------------|
| `--path <DIR>` | Directory to organize (default: current directory) |
| `--dry-run` | Preview without moving files |
| `--find-duplicates` | Skip files with identical name, size, and modification date |
| `--keep-structure` | Preserve sub-folder hierarchy inside category folders |

### Examples

```bash
# Organize Downloads with duplicate detection
smart-organizer --find-duplicates --path ~/Downloads

# Preserve folder structure (Work/report.pdf → Documents/Work/report.pdf)
smart-organizer --keep-structure --path ~/Projects

# Combine flags
smart-organizer --dry-run --find-duplicates --keep-structure --path ~/Desktop
```

## Configuration

Edit `config.toml` to define your own categories:

```toml
[categories]
Images    = ["jpg", "jpeg", "png", "gif", "bmp", "webp", "svg"]
Documents = ["pdf", "doc", "docx", "txt", "rtf", "odt", "xlsx", "csv"]
Videos    = ["mp4", "mkv", "mov", "avi", "webm"]
Music     = ["mp3", "wav", "flac", "aac", "ogg"]
Archives  = ["zip", "rar", "7z", "tar", "gz"]
Code      = ["py", "rs", "js", "ts", "html", "css", "go", "c", "cpp"]
```

Files with extensions not listed in any category are left in place. Categories are created as sub-directories inside the target folder.

## How It Works

1. Recursively scans the target directory for files.
2. Skips hidden files (`.DS_Store`, `.gitignore`), OS metadata (`Thumbs.db`, `desktop.ini`), previously sorted category folders, and the organizer's own log file.
3. Matches each file's extension against the configured categories.
4. Moves the file into the matching category folder. If a file with that name already exists at the destination, it appends a date (`photo_2026-02-11.jpg`) and, if still needed, a version number (`photo_2026-02-11_v2.jpg`).
5. Logs every move to `organizer_log.txt`.

### Duplicate Detection

When `--find-duplicates` is enabled, files are fingerprinted by name + modification date + size. If a match is found, the duplicate is skipped. This is a lightweight heuristic — for cryptographic accuracy, consider extending with SHA-256 hashing.

### Safety

The tool only moves files — it never deletes or overwrites. Every operation is recorded in the log, and `--dry-run` lets you verify behavior before committing.

## Project Structure

```
├── Cargo.toml
├── config.toml              # Category definitions
├── src/
│   ├── lib.rs               # Core logic (public API)
│   └── main.rs              # CLI entry point
└── tests/
    └── integration.rs        # 30 integration & unit tests
```

## Testing

```bash
# Run all 30 tests
cargo test

# Run with visible output
cargo test -- --nocapture

# Run a subset
cargo test collision          # collision resolution tests
cargo test categorize         # extension categorization tests
cargo test duplicate          # duplicate detection tests
cargo test sorts_by_extension # single test by name
```

Test coverage includes: extension categorization, case-insensitivity, junk file filtering, collision resolution chains, recursive file collection, directory skipping, file moving with content preservation, dry-run mode, `--keep-structure`, duplicate detection, extensionless file handling, log file creation, and empty directory edge cases.

## Limitations

- Categorization is extension-based only. Files are not inspected by content.
- Duplicate detection uses name + date + size, not cryptographic hashes. Renamed duplicates will not be caught.
- The log file (`organizer_log.txt`) is written to the current working directory, not the target directory.
- No built-in undo. Use the log to manually reverse moves if needed.
- Not tested with symlinks or files requiring elevated permissions.

## Platform Support

Tested on Windows 10/11, macOS, and Linux. 

## License

MIT
