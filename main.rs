use std::path::PathBuf;

use clap::Parser;
use colored::*;

use smart_organizer::{organize, Config, OrganizeOpts};

// ──────────────────────────────────────────────
//  CLI Arguments
// ──────────────────────────────────────────────

/// Smart File Organizer — sort files into folders by extension.
#[derive(Parser, Debug)]
#[command(name = "smart-organizer")]
#[command(version, about, long_about = None)]
struct Args {
    /// Directory to organize (defaults to the current directory).
    #[arg(short, long, default_value = ".")]
    path: PathBuf,

    /// Preview what *would* happen without moving anything.
    #[arg(short, long)]
    dry_run: bool,

    /// Flag files that share the same name, size, and modification date.
    #[arg(long)]
    find_duplicates: bool,

    /// Preserve the original sub-folder layout inside each category folder.
    #[arg(long)]
    keep_structure: bool,
}

// ──────────────────────────────────────────────
//  Entry point
// ──────────────────────────────────────────────

fn main() {
    println!("{}", "═══════════════════════════════════════".cyan());
    println!("{}", "      Smart File Organizer  v1.1".cyan().bold());
    println!("{}", "═══════════════════════════════════════".cyan());
    println!();

    let args = Args::parse();
    let config = Config::load();

    if !args.path.is_dir() {
        eprintln!(
            "{} \"{}\" is not a directory",
            "✗".red().bold(),
            args.path.display()
        );
        std::process::exit(1);
    }

    if args.dry_run {
        println!(
            "{}",
            "📋 PREVIEW MODE — no files will be moved".yellow().bold()
        );
        println!(
            "{}",
            "   Remove --dry-run to organize for real.".yellow()
        );
        println!();
    }

    println!("📁 Target: {}", args.path.display());
    println!();

    let opts = OrganizeOpts {
        path: args.path,
        dry_run: args.dry_run,
        find_duplicates: args.find_duplicates,
        keep_structure: args.keep_structure,
    };

    match organize(&opts, &config) {
        Ok(stats) => {
            println!();
            if opts.dry_run {
                println!(
                    "{} Preview complete: {} file(s) would be moved",
                    "✓".green().bold(),
                    stats.moved
                );
                if stats.duplicates > 0 {
                    println!(
                        "   {} duplicate(s) detected",
                        stats.duplicates.to_string().yellow()
                    );
                }
                if stats.skipped > 0 {
                    println!(
                        "   {} file(s) skipped (no matching category)",
                        stats.skipped
                    );
                }
                println!(
                    "{}",
                    "   Run again without --dry-run to apply changes.".yellow()
                );
            } else {
                println!(
                    "{} Organized {} file(s)",
                    "✓".green().bold(),
                    stats.moved
                );
                if stats.duplicates > 0 {
                    println!("   {} duplicate(s) skipped", stats.duplicates);
                }
                if stats.skipped > 0 {
                    println!("   {} file(s) had no matching category", stats.skipped);
                }
                if stats.errors > 0 {
                    println!(
                        "   {} file(s) could not be moved",
                        stats.errors.to_string().red()
                    );
                }
                println!("{}", "   See organizer_log.txt for details.".dimmed());
            }
        }
        Err(e) => {
            eprintln!("\n{} {}", "✗".red().bold(), e);
            std::process::exit(1);
        }
    }
}
