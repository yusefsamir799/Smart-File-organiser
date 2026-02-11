use std::fs;
use std::path::{Path, PathBuf};

use chrono::Local;
use smart_organizer::*;

// ══════════════════════════════════════════════
//  Test helpers
// ══════════════════════════════════════════════

/// Create a temporary directory with a unique name.
fn tmp_dir(label: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "organizer_test_{label}_{}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    dir
}

/// Create an empty file (parents included).
fn touch(path: &Path) {
    if let Some(p) = path.parent() {
        fs::create_dir_all(p).unwrap();
    }
    fs::write(path, b"").unwrap();
}

/// Create a file with specific content.
fn write_file(path: &Path, content: &[u8]) {
    if let Some(p) = path.parent() {
        fs::create_dir_all(p).unwrap();
    }
    fs::write(path, content).unwrap();
}

/// Build OrganizeOpts for testing (no clap involved).
fn opts(path: &Path) -> OrganizeOpts {
    OrganizeOpts {
        path: path.to_path_buf(),
        dry_run: false,
        find_duplicates: false,
        keep_structure: false,
    }
}

// ══════════════════════════════════════════════
//  Config::categorize
// ══════════════════════════════════════════════

#[test]
fn categorize_known_extensions() {
    let cfg = Config::default();
    assert_eq!(cfg.categorize("jpg"), Some("Images"));
    assert_eq!(cfg.categorize("pdf"), Some("Documents"));
    assert_eq!(cfg.categorize("mp4"), Some("Videos"));
    assert_eq!(cfg.categorize("mp3"), Some("Music"));
    assert_eq!(cfg.categorize("zip"), Some("Archives"));
}

#[test]
fn categorize_is_case_insensitive() {
    let cfg = Config::default();
    assert_eq!(cfg.categorize("JPG"), Some("Images"));
    assert_eq!(cfg.categorize("Pdf"), Some("Documents"));
    assert_eq!(cfg.categorize("MP4"), Some("Videos"));
}

#[test]
fn categorize_unknown_returns_none() {
    let cfg = Config::default();
    assert_eq!(cfg.categorize("xyz"), None);
    assert_eq!(cfg.categorize("randomext"), None);
    assert_eq!(cfg.categorize(""), None);
}

// ══════════════════════════════════════════════
//  Config defaults & TOML parsing
// ══════════════════════════════════════════════

#[test]
fn default_config_has_all_categories() {
    let cfg = Config::default();
    assert!(cfg.categories.contains_key("Images"));
    assert!(cfg.categories.contains_key("Documents"));
    assert!(cfg.categories.contains_key("Videos"));
    assert!(cfg.categories.contains_key("Music"));
    assert!(cfg.categories.contains_key("Archives"));
}

#[test]
fn parse_valid_toml() {
    let toml_str = r#"
        [categories]
        Photos = ["jpg", "png"]
        Texts  = ["txt", "md"]
    "#;
    let cfg: Config = toml::from_str(toml_str).unwrap();
    assert_eq!(cfg.categorize("jpg"), Some("Photos"));
    assert_eq!(cfg.categorize("md"), Some("Texts"));
    assert_eq!(cfg.categorize("mp4"), None); // not in custom config
}

#[test]
fn empty_toml_falls_back_to_defaults() {
    let cfg: Config = toml::from_str("").unwrap();
    assert!(cfg.categorize("jpg").is_some());
}

// ══════════════════════════════════════════════
//  is_hidden_or_junk
// ══════════════════════════════════════════════

#[test]
fn detects_hidden_files() {
    assert!(is_hidden_or_junk(Path::new("/tmp/.DS_Store")));
    assert!(is_hidden_or_junk(Path::new("/tmp/.gitignore")));
    assert!(is_hidden_or_junk(Path::new("/tmp/.hidden")));
}

#[test]
fn detects_os_junk() {
    assert!(is_hidden_or_junk(Path::new("/tmp/Thumbs.db")));
    assert!(is_hidden_or_junk(Path::new("/tmp/desktop.ini")));
    assert!(is_hidden_or_junk(Path::new("/tmp/organizer_log.txt")));
}

#[test]
fn normal_files_are_not_junk() {
    assert!(!is_hidden_or_junk(Path::new("/tmp/photo.jpg")));
    assert!(!is_hidden_or_junk(Path::new("/tmp/report.pdf")));
    assert!(!is_hidden_or_junk(Path::new("/tmp/song.mp3")));
}

// ══════════════════════════════════════════════
//  resolve_collision
// ══════════════════════════════════════════════

#[test]
fn no_collision_keeps_original_name() {
    let dir = tmp_dir("col_none");
    let result = resolve_collision(&dir, "photo.jpg", "jpg");
    assert_eq!(result, dir.join("photo.jpg"));
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn first_collision_appends_date() {
    let dir = tmp_dir("col_date");
    touch(&dir.join("photo.jpg"));

    let result = resolve_collision(&dir, "photo.jpg", "jpg");
    let today = Local::now().format("%Y-%m-%d").to_string();
    assert_eq!(result, dir.join(format!("photo_{today}.jpg")));
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn second_collision_appends_v2() {
    let dir = tmp_dir("col_v2");
    let today = Local::now().format("%Y-%m-%d").to_string();
    touch(&dir.join("photo.jpg"));
    touch(&dir.join(format!("photo_{today}.jpg")));

    let result = resolve_collision(&dir, "photo.jpg", "jpg");
    assert_eq!(result, dir.join(format!("photo_{today}_v2.jpg")));
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn third_collision_appends_v3() {
    let dir = tmp_dir("col_v3");
    let today = Local::now().format("%Y-%m-%d").to_string();
    touch(&dir.join("photo.jpg"));
    touch(&dir.join(format!("photo_{today}.jpg")));
    touch(&dir.join(format!("photo_{today}_v2.jpg")));

    let result = resolve_collision(&dir, "photo.jpg", "jpg");
    assert_eq!(result, dir.join(format!("photo_{today}_v3.jpg")));
    let _ = fs::remove_dir_all(&dir);
}

// ══════════════════════════════════════════════
//  collect_files
// ══════════════════════════════════════════════

#[test]
fn collects_recursively() {
    let dir = tmp_dir("cf_recursive");
    touch(&dir.join("a.jpg"));
    touch(&dir.join("sub/b.png"));
    touch(&dir.join("sub/deep/c.gif"));

    let files = collect_files(&dir, &[]).unwrap();
    assert_eq!(files.len(), 3);
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn skips_category_folders() {
    let dir = tmp_dir("cf_skip_cat");
    touch(&dir.join("a.jpg"));
    touch(&dir.join("Images/sorted.png"));

    let files = collect_files(&dir, &["Images"]).unwrap();
    assert_eq!(files.len(), 1);
    assert!(files[0].ends_with("a.jpg"));
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn skips_hidden_directories() {
    let dir = tmp_dir("cf_skip_hidden");
    touch(&dir.join("a.jpg"));
    touch(&dir.join(".git/config"));

    let files = collect_files(&dir, &[]).unwrap();
    assert_eq!(files.len(), 1);
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn empty_dir_returns_empty_vec() {
    let dir = tmp_dir("cf_empty");
    let files = collect_files(&dir, &[]).unwrap();
    assert!(files.is_empty());
    let _ = fs::remove_dir_all(&dir);
}

// ══════════════════════════════════════════════
//  move_file
// ══════════════════════════════════════════════

#[test]
fn move_renames_and_removes_source() {
    let dir = tmp_dir("mv_rename");
    let src = dir.join("original.txt");
    let dst = dir.join("moved.txt");
    write_file(&src, b"hello");

    move_file(&src, &dst).unwrap();
    assert!(!src.exists());
    assert!(dst.exists());
    assert_eq!(fs::read_to_string(&dst).unwrap(), "hello");
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn move_preserves_binary_content() {
    let dir = tmp_dir("mv_binary");
    let src = dir.join("data.bin");
    let dst = dir.join("sub/data.bin");
    let content = b"binary \x00\x01\x02";
    write_file(&src, content);
    fs::create_dir_all(dir.join("sub")).unwrap();

    move_file(&src, &dst).unwrap();
    assert_eq!(fs::read(&dst).unwrap(), content);
    let _ = fs::remove_dir_all(&dir);
}

// ══════════════════════════════════════════════
//  organize — integration tests
// ══════════════════════════════════════════════

#[test]
fn sorts_by_extension() {
    let dir = tmp_dir("org_basic");
    write_file(&dir.join("photo.jpg"), b"img");
    write_file(&dir.join("report.pdf"), b"doc");
    write_file(&dir.join("song.mp3"), b"snd");

    let stats = organize(&opts(&dir), &Config::default()).unwrap();

    assert_eq!(stats.moved, 3);
    assert_eq!(stats.errors, 0);
    assert!(dir.join("Images/photo.jpg").exists());
    assert!(dir.join("Documents/report.pdf").exists());
    assert!(dir.join("Music/song.mp3").exists());
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn skips_unknown_extensions() {
    let dir = tmp_dir("org_unknown");
    write_file(&dir.join("mystery.xyz"), b"wat");
    write_file(&dir.join("photo.jpg"), b"img");

    let stats = organize(&opts(&dir), &Config::default()).unwrap();

    assert_eq!(stats.moved, 1);
    assert_eq!(stats.skipped, 1);
    assert!(dir.join("mystery.xyz").exists());
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn skips_junk_files() {
    let dir = tmp_dir("org_junk");
    write_file(&dir.join(".DS_Store"), b"junk");
    write_file(&dir.join("Thumbs.db"), b"junk");
    write_file(&dir.join("photo.jpg"), b"img");

    let stats = organize(&opts(&dir), &Config::default()).unwrap();

    assert_eq!(stats.moved, 1);
    assert!(dir.join(".DS_Store").exists());
    assert!(dir.join("Thumbs.db").exists());
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn dry_run_moves_nothing() {
    let dir = tmp_dir("org_dry");
    write_file(&dir.join("photo.jpg"), b"img");
    write_file(&dir.join("report.pdf"), b"doc");

    let mut o = opts(&dir);
    o.dry_run = true;
    let stats = organize(&o, &Config::default()).unwrap();

    assert_eq!(stats.moved, 2);
    assert!(dir.join("photo.jpg").exists());
    assert!(dir.join("report.pdf").exists());
    assert!(!dir.join("Images").exists());
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn handles_name_collision() {
    let dir = tmp_dir("org_collision");
    write_file(&dir.join("photo.jpg"), b"first");
    fs::create_dir_all(dir.join("Images")).unwrap();
    write_file(&dir.join("Images/photo.jpg"), b"already here");

    let stats = organize(&opts(&dir), &Config::default()).unwrap();

    assert_eq!(stats.moved, 1);
    assert_eq!(
        fs::read_to_string(dir.join("Images/photo.jpg")).unwrap(),
        "already here"
    );
    let today = Local::now().format("%Y-%m-%d").to_string();
    assert!(dir.join(format!("Images/photo_{today}.jpg")).exists());
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn keep_structure_preserves_subfolders() {
    let dir = tmp_dir("org_keep");
    write_file(&dir.join("projects/work/report.pdf"), b"doc");
    write_file(&dir.join("vacation/photo.jpg"), b"img");

    let mut o = opts(&dir);
    o.keep_structure = true;
    let stats = organize(&o, &Config::default()).unwrap();

    assert_eq!(stats.moved, 2);
    assert!(dir.join("Documents/projects/work/report.pdf").exists());
    assert!(dir.join("Images/vacation/photo.jpg").exists());
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn creates_log_file() {
    let dir = tmp_dir("org_log");
    write_file(&dir.join("photo.jpg"), b"img");

    let original_cwd = std::env::current_dir().unwrap();
    std::env::set_current_dir(&dir).unwrap();

    let stats = organize(&opts(&dir), &Config::default()).unwrap();
    assert_eq!(stats.moved, 1);

    let log = fs::read_to_string(dir.join("organizer_log.txt")).unwrap();
    assert!(log.contains("Run started:"));
    assert!(log.contains("photo.jpg"));

    std::env::set_current_dir(original_cwd).unwrap();
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn empty_directory_returns_zeros() {
    let dir = tmp_dir("org_empty");
    let stats = organize(&opts(&dir), &Config::default()).unwrap();

    assert_eq!(stats.moved, 0);
    assert_eq!(stats.skipped, 0);
    assert_eq!(stats.duplicates, 0);
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn extensionless_files_counted_as_skipped() {
    let dir = tmp_dir("org_noext");
    write_file(&dir.join("Makefile"), b"all: build");
    write_file(&dir.join("LICENSE"), b"MIT");
    write_file(&dir.join("photo.jpg"), b"img");

    let stats = organize(&opts(&dir), &Config::default()).unwrap();

    assert_eq!(stats.moved, 1);
    assert_eq!(stats.skipped, 2);
    assert!(dir.join("Makefile").exists());
    assert!(dir.join("LICENSE").exists());
    let _ = fs::remove_dir_all(&dir);
}

// ══════════════════════════════════════════════
//  Duplicate detection
// ══════════════════════════════════════════════

#[test]
fn detects_duplicates_same_name_and_size() {
    let dir = tmp_dir("org_dup");
    write_file(&dir.join("a/photo.jpg"), b"identical");
    write_file(&dir.join("b/photo.jpg"), b"identical");

    let mut o = opts(&dir);
    o.find_duplicates = true;
    let stats = organize(&o, &Config::default()).unwrap();

    assert_eq!(stats.moved, 1);
    assert_eq!(stats.duplicates, 1);
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn different_sizes_are_not_duplicates() {
    let dir = tmp_dir("org_nodup");
    write_file(&dir.join("a/photo.jpg"), b"short");
    write_file(&dir.join("b/photo.jpg"), b"much longer content here");

    let mut o = opts(&dir);
    o.find_duplicates = true;
    let stats = organize(&o, &Config::default()).unwrap();

    assert_eq!(stats.duplicates, 0);
    assert_eq!(stats.moved, 2);
    let _ = fs::remove_dir_all(&dir);
}
