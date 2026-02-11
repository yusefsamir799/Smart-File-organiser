use std::fs;
use std::path::{Path, PathBuf};
use std::io::{self, Write};

fn main() {
    println!("=== File Organizer ===\n");
    
    // Get the directory to organize
    print!("Enter the path to organize (or press Enter for current directory): ");
    io::stdout().flush().unwrap();
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let target_dir = input.trim();
    
    // Use current directory if no input
    let directory = if target_dir.is_empty() {
        "."
    } else {
        target_dir
    };
    
    println!("\nOrganizing files in: {}", directory);
    
    // Ask for confirmation
    print!("Continue? (y/n): ");
    io::stdout().flush().unwrap();
    
    let mut confirm = String::new();
    io::stdin().read_line(&mut confirm).unwrap();
    
    if confirm.trim().to_lowercase() != "y" {
        println!("Cancelled.");
        return;
    }
    
    // Organize the files
    match organize_files(directory) {
        Ok(count) => println!("\n✓ Successfully organized {} files!", count),
        Err(e) => println!("\n✗ Error: {}", e),
    }
}

fn organize_files(directory: &str) -> io::Result<usize> {
    let path = Path::new(directory);
    let mut file_count = 0;
    
    // Read all entries in the directory
    let entries = fs::read_dir(path)?;
    
    for entry in entries {
        let entry = entry?;
        let file_path = entry.path();
        
        // Skip if it's a directory
        if file_path.is_dir() {
            continue;
        }
        
        // Get file extension
        if let Some(extension) = file_path.extension() {
            let ext = extension.to_string_lossy().to_lowercase();
            
            // Determine category based on extension
            let category = categorize_file(&ext);
            
            // Create category folder if it doesn't exist
            let category_path = path.join(category);
            if !category_path.exists() {
                fs::create_dir(&category_path)?;
            }
            
            // Move file to category folder
            let file_name = file_path.file_name().unwrap();
            let new_path = category_path.join(file_name);
            
            // Check if file already exists in destination
            if new_path.exists() {
                println!("  Skipping {} (already exists in {})", file_name.to_string_lossy(), category);
                continue;
            }
            
            fs::rename(&file_path, &new_path)?;
            println!("  Moved {} → {}/", file_name.to_string_lossy(), category);
            file_count += 1;
        }
    }
    
    Ok(file_count)
}

fn categorize_file(extension: &str) -> &str {
    match extension {
        // Images
        "jpg" | "jpeg" | "png" | "gif" | "bmp" | "svg" | "webp" | "ico" => "Images",
        
        // Documents
        "pdf" | "doc" | "docx" | "txt" | "odt" | "rtf" => "Documents",
        
        // Spreadsheets
        "xls" | "xlsx" | "csv" | "ods" => "Spreadsheets",
        
        // Presentations
        "ppt" | "pptx" | "odp" => "Presentations",
        
        // Videos
        "mp4" | "avi" | "mkv" | "mov" | "wmv" | "flv" | "webm" => "Videos",
        
        // Audio
        "mp3" | "wav" | "flac" | "aac" | "ogg" | "wma" | "m4a" => "Audio",
        
        // Archives
        "zip" | "rar" | "7z" | "tar" | "gz" | "bz2" => "Archives",
        
        // Code
        "py" | "rs" | "js" | "java" | "cpp" | "c" | "h" | "html" | "css" | "json" | "xml" => "Code",
        
        // Executables
        "exe" | "msi" | "app" | "deb" | "rpm" => "Programs",
        
        // Default category for unknown types
        _ => "Other",
    }
}
