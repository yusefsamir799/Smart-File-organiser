use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

use chrono::Local;
use colored::*;
use serde::Deserialize;

// ──────────────────────────────────────────────
//  Configuration
// ──────────────────────────────────────────────

#[derive(Deserialize, Debug)]
pub struct Config {
    #[serde(default = "default_categories")]
    pub categories: HashMap<String, Vec<String>>,
}

impl Config {
    /// Try to read `config.toml` from disk; fall back to built-in defaults.
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

    /// Return the category name that owns `extension`, or `None`.
    pub fn categorize(&self, extension: &str) -> Option<&str> {
        for (category, extensions) in &self.categories {
            if extensions.iter().any(|e| e.eq_ignore_ascii_case(extension)) {
                return Some(category);
            }
        }
        None
    }
}

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
//  Organize options (decoupled from clap)
// ──────────────────────────────────────────────

/// Settings that control how `organize()` behaves.
/// Separate from the CLI parser so tests can construct it directly.
pub struct OrganizeOpts {
    pub path: PathBuf,
    pub dry_run: bool,
    pub find_duplicates: bool,
    pub keep_structure: bool,
}

// ──────────────────────────────────────────────
//  Statistics
// ──────────────────────────────────────────────

pub struct Stats {
    pub moved: usize,
    pub duplicates: usize,
    pub skipped: usize,
    pub errors: usize,
}

// ──────────────────────────────────────────────
//  Duplicate tracking
// ──────────────────────────────────────────────

#[derive(Debug)]
struct FilePrint {
    first_seen: PathBuf,
    #[allow(dead_code)]
    size: u64,
}

// ──────────────────────────────────────────────
//  Core organizer
// ──────────────────────────────────────────────

pub fn organize(opts: &OrganizeOpts, config: &Config) -> std::io::Result<Stats> {
    let base = &opts.path;

    // Collect the names of category folders so we don't recurse into them.
    let category_names: Vec<&str> = config.categories.keys().map(String::as_str).collect();

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

    // Open the log file only when we're doing a real run.
    let mut log: Option<fs::File> = if !opts.dry_run {
        let f = fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open("organizer_log.txt")?;
        Some(f)
    } else {
        None
    };

    if let Some(ref mut f) = log {
        let ts = Local::now().format("%Y-%m-%d %H:%M:%S");
        writeln!(f, "\n{}", "=".repeat(40))?;
        writeln!(f, "Run started:  {ts}")?;
        writeln!(f, "Directory:    {}", base.display())?;
        writeln!(f, "Dry-run:      {}", opts.dry_run)?;
        writeln!(f, "{}\n", "=".repeat(40))?;
    }

    let mut stats = Stats {
        moved: 0,
        duplicates: 0,
        skipped: 0,
        errors: 0,
    };

    let mut seen: HashMap<String, FilePrint> = HashMap::new();

    for file_path in &files {
        // ── Skip hidden / OS junk files ──────────────────────────────
        if is_hidden_or_junk(file_path) {
            continue;
        }

        // ── Extension check ──────────────────────────────────────────
        let ext = match file_path.extension() {
            Some(e) => e.to_string_lossy().to_lowercase(),
            None => {
                stats.skipped += 1;
                continue;
            }
        };

        // ── Duplicate detection ──────────────────────────────────────
        let meta = fs::metadata(file_path)?;
        let file_size = meta.len();
        let modified_date = chrono::DateTime::<Local>::from(meta.modified()?)
            .format("%Y-%m-%d")
            .to_string();

        let file_name = file_path
            .file_name()
            .unwrap_or_default()
            .to_string_lossy();

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
            seen.insert(
                key,
                FilePrint {
                    first_seen: file_path.clone(),
                    size: file_size,
                },
            );
        }

        // ── Categorize ───────────────────────────────────────────────
        let category = match config.categorize(&ext) {
            Some(c) => c,
            None => {
                stats.skipped += 1;
                continue;
            }
        };

        // ── Build destination path ───────────────────────────────────
        let dest_dir = if opts.keep_structure {
            let relative = file_path.strip_prefix(base).unwrap_or(file_path);
            match relative.parent() {
                Some(p) if p.components().next().is_some() => base.join(category).join(p),
                _ => base.join(category),
            }
        } else {
            base.join(category)
        };

        let dest_file = resolve_collision(&dest_dir, &file_name, &ext);

        // ── Pretty-print source → destination ────────────────────────
        let src_display = file_path.strip_prefix(base).unwrap_or(file_path).display();
        let dst_display = dest_file
            .strip_prefix(base)
            .unwrap_or(&dest_file)
            .display();

        if opts.dry_run {
            println!(
                "  {} {} {} {}",
                "→".cyan(),
                src_display,
                "→".dimmed(),
                dst_display.to_string().green()
            );
            stats.moved += 1;
        } else {
            if !dest_dir.exists() {
                fs::create_dir_all(&dest_dir)?;
            }

            match move_file(file_path, &dest_file) {
                Ok(()) => {
                    println!(
                        "  {} {} {} {}",
                        "✓".green(),
                        src_display,
                        "→".dimmed(),
                        dst_display.to_string().cyan()
                    );
                    if let Some(ref mut f) = log {
                        writeln!(f, "{src_display} -> {dst_display}").ok();
                    }
                    stats.moved += 1;
                }
                Err(e) => {
                    eprintln!("  {} {} — {}", "✗".red(), src_display, e);
                    stats.errors += 1;
                }
            }
        }
    }

    Ok(stats)
}

// ──────────────────────────────────────────────
//  Helpers (public so integration tests can reach them)
// ──────────────────────────────────────────────

/// Recursively collect files, skipping category-named and hidden directories.
pub fn collect_files(dir: &Path, skip: &[&str]) -> std::io::Result<Vec<PathBuf>> {
    let mut out = Vec::new();

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if let Some(name) = path.file_name() {
            let name = name.to_string_lossy();
            if name.starts_with('.') || (path.is_dir() && skip.contains(&name.as_ref())) {
                continue;
            }
        }

        if path.is_dir() {
            out.append(&mut collect_files(&path, skip)?);
        } else {
            out.push(path);
        }
    }

    Ok(out)
}

/// Return true for dotfiles, `.DS_Store`, `Thumbs.db`, desktop.ini, etc.
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

/// Pick a destination path that doesn't collide with existing files.
///
/// Strategy: `photo.jpg` → `photo_2026-02-11.jpg` → `photo_2026-02-11_v2.jpg`
pub fn resolve_collision(dir: &Path, original_name: &str, ext: &str) -> PathBuf {
    let candidate = dir.join(original_name);
    if !candidate.exists() {
        return candidate;
    }

    let stem = Path::new(original_name)
        .file_stem()
        .unwrap_or_default()
        .to_string_lossy();
    let today = Local::now().format("%Y-%m-%d");

    let dated = dir.join(format!("{stem}_{today}.{ext}"));
    if !dated.exists() {
        return dated;
    }

    let mut n = 2;
    loop {
        let versioned = dir.join(format!("{stem}_{today}_v{n}.{ext}"));
        if !versioned.exists() {
            return versioned;
        }
        n += 1;
    }
}

/// Move a file, falling back to copy-then-delete when a rename fails
/// (e.g. across filesystem boundaries).
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
