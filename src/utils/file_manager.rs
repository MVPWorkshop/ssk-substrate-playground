use actix_files::NamedFile;
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use log::{error, info};
use std::fs;
use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;
use std::path::PathBuf;
use zip::{write::FileOptions, ZipWriter};

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
        .open(file_path)
    {
        Ok(f) => f,
        Err(e) => {
            error!("Failed to open file '{}': {}", file_path.display(), e);
            return Err(e);
        }
    };

    if let Err(e) = file.write_all(new_content.as_bytes()) {
        error!(
            "Failed to write new content to file '{}': {}",
            file_path.display(),
            e
        );
        return Err(e);
    }

    info!(
        "File content replaced successfully in '{}'",
        file_path.display()
    );
    Ok(())
}

/// Asynchronously downloads the generated project as a ZIP file.
///
/// This function checks if the specified project directory exists. If it does,
/// it creates a ZIP file containing all the files and subdirectories within
/// that directory. The ZIP file is then served as a response to the client.
///
/// # Arguments
///
/// * `req` - The HTTP request object.
/// * `path` - The web path containing the project name.
///
/// # Returns
///
/// Returns an HTTP response indicating the success or failure of the download.
pub async fn download_project(req: HttpRequest, path: web::Path<String>) -> impl Responder {
    let project_name = path.into_inner();
    let project_path = PathBuf::from(format!("./generated_code/{}", project_name)); // Adjust the path as needed

    if project_path.is_dir() {
        let zip_path = format!("./generated_code/{}.zip", project_name);
        if let Err(e) = zip_directory(&project_path, &zip_path) {
            eprintln!("Error creating ZIP: {}", e); // Log the error to the console
            return HttpResponse::InternalServerError().body(format!("Error creating ZIP: {}", e));
        }

        match NamedFile::open(zip_path) {
            Ok(file) => file.into_response(&req),
            Err(e) => {
                eprintln!("Error opening ZIP file: {}", e); // Log the error to the console
                HttpResponse::InternalServerError().body("Error opening ZIP file")
            }
        }
    } else {
        eprintln!("Project directory not found: {}", project_path.display()); // Log the missing directory
        HttpResponse::NotFound().body("Project not found")
    }
}

/// Creates a ZIP file from a specified directory.
///
/// This function takes the source directory and the destination path for the
/// ZIP file. It recursively adds all files and subdirectories to the ZIP
/// archive.
///
/// # Arguments
///
/// * `source` - The path to the directory to be zipped.
/// * `destination` - The path where the ZIP file will be created.
///
/// # Returns
///
/// Returns an `io::Result<()>`, indicating success or failure of the operation.
fn zip_directory(source: &Path, destination: &str) -> io::Result<()> {
    let zip_file = fs::File::create(destination)?;
    let mut zip = ZipWriter::new(zip_file);

    let options = FileOptions::default()
        .compression_method(zip::CompressionMethod::Stored)
        .unix_permissions(0o755)
        .large_file(true);

    // Walk through the directory and add files and subdirectories to the ZIP
    add_to_zip(&mut zip, source, source, &options)?;

    zip.finish()?;
    Ok(())
}

/// Recursively adds files and directories to the ZIP archive.
///
/// This function is called by `zip_directory` to traverse the directory tree
/// and add each file and directory to the ZIP archive.
///
/// # Arguments
///
/// * `zip` - A mutable reference to the ZipWriter instance.
/// * `base_path` - The base path used to create relative paths for the ZIP entries.
/// * `path` - The current path being processed.
/// * `options` - The file options used for adding files to the ZIP.
///
/// # Returns
///
/// Returns an `io::Result<()>`, indicating success or failure of the operation.
fn add_to_zip(
    zip: &mut ZipWriter<fs::File>,
    base_path: &Path,
    path: &Path,
    options: &FileOptions,
) -> io::Result<()> {
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let entry_path = entry.path();

        // Create the relative path for the ZIP file
        let relative_path = entry_path.strip_prefix(base_path).unwrap();

        // If the entry is a directory, recursively add its contents
        if entry_path.is_dir() {
            zip.add_directory(relative_path.to_string_lossy(), *options)?;
            add_to_zip(zip, base_path, &entry_path, options)?;
        } else {
            // Add the file to the ZIP
            zip.start_file(relative_path.to_string_lossy(), *options)?;
            let mut f = fs::File::open(&entry_path)?;
            io::copy(&mut f, zip)?;
        }
    }

    Ok(())
}
