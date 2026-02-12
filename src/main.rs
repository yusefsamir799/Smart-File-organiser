use std::path::PathBuf;

use clap::Parser;
use colored::*;

use smart_organizer::{organize, Config, OrganizeOpts};

// This struct holds the command-line arguments the user can type in
// For example: smart-organizer --path ~/Downloads --dry-run
#[derive(Parser, Debug)]
#[command(name = "smart-organizer")]
#[command(version, about, long_about = None)]
struct Args {
    // Which folder to organize (if not given, use the current folder)
    #[arg(short, long, default_value = ".")]
    path: PathBuf,

    // If true, just show what would happen but don't actually move files
    #[arg(short, long)]
    dry_run: bool,

    // If true, look for duplicate files and skip them
    #[arg(long)]
    find_duplicates: bool,

    // If true, keep the original subfolder layout inside each category
    #[arg(long)]
    keep_structure: bool,
}

// This is where the program starts running
fn main() {
    // Print a nice header
    println!("{}", "═══════════════════════════════════════".cyan());
    println!("{}", "      Smart File Organizer  v1.1".cyan().bold());
    println!("{}", "═══════════════════════════════════════".cyan());
    println!();

    // Read what the user typed in the command line
    let args = Args::parse();

    // Load the config file (which file types go in which folders)
    let config = Config::load();

    // Check if the path the user gave is actually a folder
    if !args.path.is_dir() {
        eprintln!(
            "{} \"{}\" is not a directory",
            "✗".red().bold(),
            args.path.display()
        );
        std::process::exit(1);
    }

    // If dry-run mode is on, tell the user nothing will be moved
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

    // Show which folder we are organizing
    println!("📁 Target: {}", args.path.display());
    println!();

    // Put all the user's choices into one struct to pass to the organize function
    let opts = OrganizeOpts {
        path: args.path,
        dry_run: args.dry_run,
        find_duplicates: args.find_duplicates,
        keep_structure: args.keep_structure,
    };

    // Run the organizer and check if it worked or failed
    match organize(&opts, &config) {
        // If it worked, show a summary of what happened
        Ok(stats) => {
            println!();
            if opts.dry_run {
                // In preview mode, show what WOULD happen
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
                // In real mode, show what actually happened
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
        // If something went wrong, show the error
        Err(e) => {
            eprintln!("\n{} {}", "✗".red().bold(), e);
            std::process::exit(1);
        }
    }
}
