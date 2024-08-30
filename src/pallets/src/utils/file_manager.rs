use std::fs::File;
use std::io::{self, Read, Write};
use std::fs;
use std::path::{Path, PathBuf};

pub fn read_file_to_string(file_path: &str) -> io::Result<String> {
    // Open the file in read-only mode
    let mut file = File::open(file_path)?;

    // Create a String to hold the file content
    let mut content = String::new();

    // Read the file content into the String
    file.read_to_string(&mut content)?;

    // Return the file content as a Result<String>
    Ok(content)
}

pub fn copy_dir_recursive(src: &Path, dest: &Path) -> io::Result<()> {
    if !dest.exists() {
        fs::create_dir_all(dest)?;
    }

    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let entry_path = entry.path();
        let dest_path = dest.join(entry.file_name());

        if entry_path.is_dir() {
            copy_dir_recursive(&entry_path, &dest_path)?;
        } else {
            fs::copy(&entry_path, &dest_path)?;
        }
    }

    Ok(())
}


pub fn create_new_folder(base_path: &Path, folder_name: &str) -> io::Result<()> {
    let new_folder_path = base_path.join(folder_name);
    if new_folder_path.exists() {
        println!("Folder already exists at: {:?}", new_folder_path);
    } else {
        fs::create_dir_all(&new_folder_path)?;
        println!("Folder created successfully at: {:?}", new_folder_path);
    }
    Ok(())
}

pub fn replace_file_content(file_path: &Path, new_content: &str) -> io::Result<()> {
    let mut file = fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(file_path)?;

    file.write_all(new_content.as_bytes())?;
    println!("File content replaced successfully.");

    Ok(())
}