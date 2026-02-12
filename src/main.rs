use clap::Parser;
use colored::*;
use smart_organizer::{organize, Config, OrganizeOpts};
use std::path::PathBuf;

// Command-line arguments the user can type
#[derive(Parser, Debug)]
#[command(name = "smart-organizer", version, about)]
struct Args {
    // Which folder to organize
    #[arg(short, long, default_value = ".")]
    path: PathBuf,

    // Preview mode — don't actually move files
    #[arg(short, long)]
    dry_run: bool,

    // Skip duplicate files
    #[arg(long)]
    find_duplicates: bool,

    // Keep subfolder layout inside categories
    #[arg(long)]
    keep_structure: bool,
}

fn main() {
    // Print header
    println!("{}", "═══════════════════════════════════════".cyan());
    println!("{}", "      Smart File Organizer  v1.1".cyan().bold());
    println!("{}", "═══════════════════════════════════════\n".cyan());

    let args = Args::parse();
    let config = Config::load();

    // Make sure the path is a real folder
    if !args.path.is_dir() {
        eprintln!("{} \"{}\" is not a directory", "✗".red().bold(), args.path.display());
        std::process::exit(1);
    }

    if args.dry_run {
        println!("{}", "📋 PREVIEW MODE — no files will be moved\n".yellow().bold());
    }

    println!("📁 Target: {}\n", args.path.display());

    // Run the organizer
    let opts = OrganizeOpts {
        path: args.path,
        dry_run: args.dry_run,
        find_duplicates: args.find_duplicates,
        keep_structure: args.keep_structure,
    };

    match organize(&opts, &config) {
        Ok(stats) => {
            println!();
            let label = if opts.dry_run { "would be moved" } else { "organized" };
            println!("{} {} file(s) {}", "✓".green().bold(), stats.moved, label);

            if stats.duplicates > 0 { println!("   {} duplicate(s) found", stats.duplicates); }
            if stats.skipped > 0    { println!("   {} file(s) skipped", stats.skipped); }
            if stats.errors > 0     { println!("   {} error(s)", stats.errors.to_string().red()); }

            if opts.dry_run {
                println!("{}", "   Run without --dry-run to apply.".yellow());
            } else {
                println!("{}", "   See organizer_log.txt for details.".dimmed());
            }
        }
        Err(e) => {
            eprintln!("\n{} {}", "✗".red().bold(), e);
            std::process::exit(1);
        }
    }
}
