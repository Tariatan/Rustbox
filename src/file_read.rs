use std::fs;
use std::fs::File;
use std::io;
use std::io::Read;

pub fn list_current_directory() -> io::Result<()> {
    // Get the current working directory
    let current_dir = fs::canonicalize(".")?;
    println!("Current directory: {}", current_dir.display());

    // Read directory entries
    println!("Files in directory:");
    let entries = fs::read_dir(".")?;

    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        let file_type = entry.file_type()?;

        // Display the file name and whether it's a directory or file
        let type_label = if file_type.is_dir() { "Directory" } else if file_type.is_file() { "File" } else { "Other" };
        println!("  {} - {}", path.display(), type_label);
    }

    Ok(())
}

pub fn read_file() -> Result<String, std::io::Error> {
    let file : Result<File, std::io::Error> = File::open("README.md");
    let mut contents = String::new();
    match file {
        Ok(mut file) => {
            if let Ok(bytes) = file.read_to_string(&mut contents) {
                println!("Content: {contents} ({bytes} bytes)");
            } else {
                println!("Could not read file content");
            }
        }
        Err(err) => {
            println!("The file could not be opened: {err}");
        }
    }

    Ok(contents)
}