use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

use chrono::Local;
use colored::*;
use serde::Deserialize;

// ──────────────────────────────────────────────
//  Configuration
//  This is where we define which file types
//  go into which folders (e.g. .jpg -> Images)
// ──────────────────────────────────────────────

// This struct stores the categories and their file extensions
// Example: "Images" -> ["jpg", "png", "gif"]
#[derive(Deserialize, Debug)]
pub struct Config {
    #[serde(default = "default_categories")]
    pub categories: HashMap<String, Vec<String>>,
}

impl Config {
    // Try to read categories from config.toml file
    // If the file doesn't exist or has errors, use the built-in defaults
    pub fn load() -> Self {
        match fs::read_to_string("config.toml") {
            Ok(text) => match toml::from_str::<Config>(&text) {
                Ok(cfg) => {
                    println!("{} Loaded config.toml", "✓".green());
                    cfg
                }
                Err(e) => {
                    eprintln!(
                        "{} config.toml has errors ({}), using defaults",
                        "⚠".yellow(),
                        e
                    );
                    Config::default()
                }
            },
            Err(_) => {
                println!("{} No config.toml found, using defaults", "ℹ".blue());
                Config::default()
            }
        }
    }

    // Given a file extension like "jpg", find which category it belongs to
    // Returns Some("Images") or None if no category matches
    pub fn categorize(&self, extension: &str) -> Option<&str> {
        for (category, extensions) in &self.categories {
            if extensions.iter().any(|e| e.eq_ignore_ascii_case(extension)) {
                return Some(category);
            }
        }
        None
    }
}

// These are the default categories if no config.toml file is found
impl Default for Config {
    fn default() -> Self {
        let mut categories = HashMap::new();
        categories.insert(
            "Images".into(),
            vec!["jpg", "jpeg", "png", "gif", "bmp", "webp", "svg"]
                .into_iter()
                .map(String::from)
                .collect(),
        );
        categories.insert(
            "Documents".into(),
            vec!["pdf", "doc", "docx", "txt", "rtf", "odt", "xlsx", "csv"]
                .into_iter()
                .map(String::from)
                .collect(),
        );
        categories.insert(
            "Videos".into(),
            vec!["mp4", "mkv", "mov", "avi", "webm"]
                .into_iter()
                .map(String::from)
                .collect(),
        );
        categories.insert(
            "Music".into(),
            vec!["mp3", "wav", "flac", "aac", "ogg"]
                .into_iter()
                .map(String::from)
                .collect(),
        );
        categories.insert(
            "Archives".into(),
            vec!["zip", "rar", "7z", "tar", "gz"]
                .into_iter()
                .map(String::from)
                .collect(),
        );
        Config { categories }
    }
}

fn default_categories() -> HashMap<String, Vec<String>> {
    Config::default().categories
}

// ──────────────────────────────────────────────
//  Options
//  These are the settings the user picks
//  (which folder, dry-run, duplicates, etc.)
// ──────────────────────────────────────────────

pub struct OrganizeOpts {
    pub path: PathBuf,        // the folder to organize
    pub dry_run: bool,        // just preview, don't move
    pub find_duplicates: bool, // skip duplicate files
    pub keep_structure: bool,  // keep subfolder layout
}

// ──────────────────────────────────────────────
//  Statistics
//  Keeps track of what happened during organizing
// ──────────────────────────────────────────────

pub struct Stats {
    pub moved: usize,      // how many files we moved
    pub duplicates: usize,  // how many duplicates we found
    pub skipped: usize,     // files with no matching category
    pub errors: usize,      // files that failed to move
}

// ──────────────────────────────────────────────
//  Duplicate tracking
//  Used to remember files we've already seen
// ──────────────────────────────────────────────

#[derive(Debug)]
struct FilePrint {
    first_seen: PathBuf,   // where we first found this file
    #[allow(dead_code)]
    size: u64,             // file size in bytes
}

// ──────────────────────────────────────────────
//  Main organize function
//  This is where the magic happens — it goes
//  through all files and sorts them into folders
// ──────────────────────────────────────────────

pub fn organize(opts: &OrganizeOpts, config: &Config) -> std::io::Result<Stats> {
    let base = &opts.path;

    // Get the names of category folders (Images, Documents, etc.)
    // so we don't accidentally try to organize files inside them
    let category_names: Vec<&str> = config.categories.keys().map(String::as_str).collect();

    // Get a list of all files in the folder
    let files = collect_files(base, &category_names)?;
    if files.is_empty() {
        println!("No files to organize.");
        return Ok(Stats {
            moved: 0,
            duplicates: 0,
            skipped: 0,
            errors: 0,
        });
    }

    println!("Found {} file(s)\n", files.len());

    // Create a log file to record what we did (only in real mode, not dry-run)
    let mut log: Option<fs::File> = if !opts.dry_run {
        let f = fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open("organizer_log.txt")?;
        Some(f)
    } else {
        None
    };

    // Write a header to the log file with the date and settings
    if let Some(ref mut f) = log {
        let ts = Local::now().format("%Y-%m-%d %H:%M:%S");
        writeln!(f, "\n{}", "=".repeat(40))?;
        writeln!(f, "Run started:  {ts}")?;
        writeln!(f, "Directory:    {}", base.display())?;
        writeln!(f, "Dry-run:      {}", opts.dry_run)?;
        writeln!(f, "{}\n", "=".repeat(40))?;
    }

    // Start counting
    let mut stats = Stats {
        moved: 0,
        duplicates: 0,
        skipped: 0,
        errors: 0,
    };

    // This hashmap remembers files we've seen (for duplicate detection)
    let mut seen: HashMap<String, FilePrint> = HashMap::new();

    // Loop through every file we found
    for file_path in &files {

        // Skip hidden files and junk files like .DS_Store or Thumbs.db
        if is_hidden_or_junk(file_path) {
            continue;
        }

        // Get the file extension (e.g. "jpg" from "photo.jpg")
        // If the file has no extension, skip it
        let ext = match file_path.extension() {
            Some(e) => e.to_string_lossy().to_lowercase(),
            None => {
                stats.skipped += 1;
                continue;
            }
        };

        // Get file info: size and when it was last changed
        let meta = fs::metadata(file_path)?;
        let file_size = meta.len();
        let modified_date = chrono::DateTime::<Local>::from(meta.modified()?)
            .format("%Y-%m-%d")
            .to_string();

        let file_name = file_path
            .file_name()
            .unwrap_or_default()
            .to_string_lossy();

        // If duplicate detection is on, check if we've seen this file before
        // We identify duplicates by: same name + same date + same size
        if opts.find_duplicates {
            let key = format!("{file_name}|{modified_date}|{file_size}");

            if let Some(existing) = seen.get(&key) {
                println!(
                    "{} {} (duplicate of {})",
                    "⚠ SKIP:".yellow(),
                    file_name,
                    existing.first_seen.display()
                );
                stats.duplicates += 1;
                continue;
            }
            // Remember this file for future duplicate checks
            seen.insert(
                key,
                FilePrint {
                    first_seen: file_path.clone(),
                    size: file_size,
                },
            );
        }

        // Find which category this file belongs to based on its extension
        // e.g. "jpg" -> "Images"
        let category = match config.categorize(&ext) {
            Some(c) => c,
            None => {
                stats.skipped += 1;
                continue;
            }
        };

        // Figure out where to put the file
        // If keep_structure is on, preserve the subfolder path
        let dest_dir = if opts.keep_structure {
            let relative = file_path.strip_prefix(base).unwrap_or(file_path);
            match relative.parent() {
                Some(p) if p.components().next().is_some() => base.join(category).join(p),
                _ => base.join(category),
            }
        } else {
            base.join(category)
        };

        // If a file with the same name already exists, add a date or version number
        let dest_file = resolve_collision(&dest_dir, &file_name, &ext);

        // Show the user what's happening (source -> destination)
        let src_display = file_path.strip_prefix(base).unwrap_or(file_path).display();
        let dst_display = dest_file
            .strip_prefix(base)
            .unwrap_or(&dest_file)
            .display();

        if opts.dry_run {
            // In preview mode, just print what would happen
            println!(
                "  {} {} {} {}",
                "→".cyan(),
                src_display,
                "→".dimmed(),
                dst_display.to_string().green()
            );
            stats.moved += 1;
        } else {
            // In real mode, actually create the folder and move the file
            if !dest_dir.exists() {
                fs::create_dir_all(&dest_dir)?;
            }

            match move_file(file_path, &dest_file) {
                Ok(()) => {
                    // File moved successfully
                    println!(
                        "  {} {} {} {}",
                        "✓".green(),
                        src_display,
                        "→".dimmed(),
                        dst_display.to_string().cyan()
                    );
                    // Write to the log file
                    if let Some(ref mut f) = log {
                        writeln!(f, "{src_display} -> {dst_display}").ok();
                    }
                    stats.moved += 1;
                }
                Err(e) => {
                    // Something went wrong moving this file
                    eprintln!("  {} {} — {}", "✗".red(), src_display, e);
                    stats.errors += 1;
                }
            }
        }
    }

    Ok(stats)
}

// ──────────────────────────────────────────────
//  Helper functions
// ──────────────────────────────────────────────

// Go through a folder and all its subfolders to find every file
// Skip hidden folders and folders that are already category names
pub fn collect_files(dir: &Path, skip: &[&str]) -> std::io::Result<Vec<PathBuf>> {
    let mut out = Vec::new();

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        // Skip hidden files/folders (start with .) and category folders
        if let Some(name) = path.file_name() {
            let name = name.to_string_lossy();
            if name.starts_with('.') || (path.is_dir() && skip.contains(&name.as_ref())) {
                continue;
            }
        }

        if path.is_dir() {
            // If it's a folder, go inside it and find more files (recursion)
            out.append(&mut collect_files(&path, skip)?);
        } else {
            // If it's a file, add it to our list
            out.push(path);
        }
    }

    Ok(out)
}

// Check if a file is a hidden file or system junk we should ignore
pub fn is_hidden_or_junk(path: &Path) -> bool {
    let name = match path.file_name() {
        Some(n) => n.to_string_lossy(),
        None => return true,
    };
    name.starts_with('.')
        || name == "Thumbs.db"
        || name == "desktop.ini"
        || name == "organizer_log.txt"
}

// If a file with the same name already exists in the destination folder,
// add today's date to the filename. If that also exists, add a version number.
// Example: photo.jpg -> photo_2026-02-12.jpg -> photo_2026-02-12_v2.jpg
pub fn resolve_collision(dir: &Path, original_name: &str, ext: &str) -> PathBuf {
    let candidate = dir.join(original_name);
    if !candidate.exists() {
        return candidate;
    }

    // Get the filename without the extension
    let stem = Path::new(original_name)
        .file_stem()
        .unwrap_or_default()
        .to_string_lossy();
    let today = Local::now().format("%Y-%m-%d");

    // Try adding today's date
    let dated = dir.join(format!("{stem}_{today}.{ext}"));
    if !dated.exists() {
        return dated;
    }

    // If that also exists, keep adding version numbers until we find one that works
    let mut n = 2;
    loop {
        let versioned = dir.join(format!("{stem}_{today}_v{n}.{ext}"));
        if !versioned.exists() {
            return versioned;
        }
        n += 1;
    }
}

// Move a file from one place to another
// First try renaming (fast), if that fails, copy it and delete the original
pub fn move_file(from: &Path, to: &Path) -> std::io::Result<()> {
    match fs::rename(from, to) {
        Ok(()) => Ok(()),
        Err(_) => {
            fs::copy(from, to)?;
            fs::remove_file(from)?;
            Ok(())
        }
    }
}
