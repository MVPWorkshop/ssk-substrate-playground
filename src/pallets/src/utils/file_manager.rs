use std::fs;
use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;
use log::{error, info};

/// Reads the contents of a file and returns it as a `String`.
///
/// # Arguments
///
/// * `file_path` - A string slice that holds the path to the file to be read.
///
/// # Returns
///
/// * `Ok(String)` containing the file content if the file was read successfully.
/// * `Err(io::Error)` if an error occurs during file opening or reading.
///
/// # Example
///
/// ```
/// let content = read_file_to_string("example.txt")?;
/// println!("{}", content);
/// ```
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

/// Recursively copies a directory and all its contents from the source path to the destination path.
///
/// # Arguments
///
/// * `src` - A reference to a `Path` representing the source directory.
/// * `dest` - A reference to a `Path` representing the destination directory.
///
/// # Returns
///
/// * `Ok(())` if the directory and its contents were copied successfully.
/// * `Err(io::Error)` if an error occurs during directory creation, reading, or copying.
///
/// # Example
///
/// ```
/// copy_dir_recursive(Path::new("source_dir"), Path::new("destination_dir"))?;
/// ```
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

/// Creates a new folder at the specified base path with the given folder name.
///
/// # Arguments
///
/// * `base_path` - A reference to a `Path` representing the base directory where the new folder will be created.
/// * `folder_name` - A string slice that holds the name of the folder to be created.
///
/// # Returns
///
/// * `Ok(())` if the folder was created successfully or if it already exists.
/// * `Err(io::Error)` if an error occurs during folder creation.
///
/// # Example
///
/// ```
/// create_new_folder(Path::new("base_dir"), "new_folder")?;
/// ```
pub fn create_new_folder(base_path: &Path, folder_name: &str) -> io::Result<()> {
    let new_folder_path = base_path.join(folder_name);
    if new_folder_path.exists() {
        info!("Folder already exists at: {:?}", new_folder_path);
    } else {
        fs::create_dir_all(&new_folder_path)?;
        info!("Folder created successfully at: {:?}", new_folder_path);
    }
    Ok(())
}

/// Replaces the content of a file with new content.
///
/// # Arguments
///
/// * `file_path` - A reference to a `Path` representing the file whose content will be replaced.
/// * `new_content` - A string slice that holds the new content to be written to the file.
///
/// # Returns
///
/// * `Ok(())` if the file content was replaced successfully.
/// * `Err(io::Error)` if an error occurs during file opening or writing.
///
/// # Example
///
/// ```
/// replace_file_content(Path::new("example.txt"), "New content")?;
/// ```
pub fn replace_file_content(file_path: &Path, new_content: &str) -> io::Result<()> {
    let mut file = match fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(file_path) {
        Ok(f) => f,
        Err(e) => {
            error!("Failed to open file '{}': {}", file_path.display(), e);
            return Err(e);
        }
    };

    if let Err(e) = file.write_all(new_content.as_bytes()) {
        error!("Failed to write new content to file '{}': {}", file_path.display(), e);
        return Err(e);
    }

    info!("File content replaced successfully in '{}'", file_path.display());
    Ok(())
}
