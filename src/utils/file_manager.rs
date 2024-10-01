use actix_files::NamedFile;
use actix_web::{web, Error, HttpRequest, HttpResponse, Responder, Result};
use log::{error, info};
use reqwest::Client;
use serde_json::json;
use std::fs;
use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;
use zip::{write::FileOptions, ZipWriter};
/// Reads the contents of a file and returns it as a `String`.s
///
/// # Arguments
///
/// * `file_path` - A string slice that holds the path to the file to be read.
///
/// # Returns
///
/// * `Ok(String)` containing the file content if the file was read successfully.
/// * `Err(io::Error)` if an error occurs during file opening or reading.
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
pub fn replace_file_content(file_path: &Path, new_content: &str) -> io::Result<()> {
    let mut file = match fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(file_path)
    {
        Ok(f) => f,
        Err(e) => {
            println!("Failed to open file '{}': {}", file_path.display(), e);
            return Err(e);
        }
    };

    if let Err(e) = file.write_all(new_content.as_bytes()) {
        println!(
            "Failed to write new content to file '{}': {}",
            file_path.display(),
            e
        );
        return Err(e);
    }

    println!(
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

/// Pushes the generated project to a GitHub repository.
///
/// This function initializes a Git repository, adds files, commits them, and pushes them to a GitHub repository.
/// It requires the project name, GitHub username, and GitHub token for authentication.
///
/// # Arguments
///
/// * `project_name` - The name of the project to push.
/// * `github_username` - The GitHub username used for authentication.
/// * `github_token` - The GitHub personal access token used for authentication.
///
/// # Returns
///
/// Returns a `Result<HttpResponse, Error>`, indicating success or failure of the operation.

pub fn push_to_github(
    project_name: &str,
    github_username: &str,
    github_email: &str,
    github_token: &str,
) -> Result<HttpResponse, Error> {
    // Construct the project directory path
    let project_dir = format!("generated_code/{}", project_name);

    // Check if the project directory exists
    if !Path::new(&project_dir).exists() {
        return Ok(HttpResponse::NotFound().body(format!("Project {} not found", project_dir)));
    }




    // Set Git user email and name
    let config_email_output = Command::new("git")
        .arg("config")
        .arg("user.email")
        .arg(github_email)
        .current_dir(&project_dir)
        .output()
        .map_err(|e| {
            println!(
                "Failed to set Git user email in '{}': {:?}",
                project_dir, e
            );
            actix_web::error::ErrorInternalServerError(format!(
                "Failed to set Git user email: {:?}",
                e
            ))
        })?;

    if !config_email_output.status.success() {
        println!(
            "Git 'config user.email' failed in directory '{}'. Output: {:?}",
            project_dir,
            String::from_utf8_lossy(&config_email_output.stderr)
        );
        return Ok(HttpResponse::InternalServerError().body("Git config user.email failed"));
    }

    let config_name_output = Command::new("git")
        .arg("config")
        .arg("user.name")
        .arg(github_username)
        .current_dir(&project_dir)
        .output()
        .map_err(|e| {
            println!(
                "Failed to set Git user name in '{}': {:?}",
                project_dir, e
            );
            actix_web::error::ErrorInternalServerError(format!(
                "Failed to set Git user name: {:?}",
                e
            ))
        })?;

    if !config_name_output.status.success() {
        println!(
            "Git 'config user.name' failed in directory '{}'. Output: {:?}",
            project_dir,
            String::from_utf8_lossy(&config_name_output.stderr)
        );
        return Ok(HttpResponse::InternalServerError().body("Git config user.name failed"));
    }





    // Initialize Git repository in the project directory
    let init_output = Command::new("git")
        .arg("init")
        .current_dir(&project_dir)
        .output()
        .map_err(|e| {
            println!(
                "Failed to initialize Git repository in '{}': {:?}",
                project_dir, e
            );
            actix_web::error::ErrorInternalServerError(format!(
                "Failed to initialize Git repository: {:?}",
                e
            ))
        })?;

    // Check if Git initialization was successful
    if !init_output.status.success() {
        println!("Git initialization failed for directory '{}'.", project_dir);
        return Ok(HttpResponse::InternalServerError().body("Git initialization failed"));
    }

    println!(
        "Git repository successfully initialized in '{}'.",
        project_dir
    );

    // Add all files in the project directory to the Git repository
    let add_output = Command::new("git")
        .arg("add")
        .arg(".")
        .current_dir(&project_dir)
        .output()
        .map_err(|e| {
            println!(
                "Error adding files to Git repository in directory '{}': {:?}",
                project_dir, e
            );
            actix_web::error::ErrorInternalServerError(format!(
                "Failed to add files to Git: {:?}",
                e
            ))
        })?;

    if !add_output.status.success() {
        println!(
            "Git 'add' command failed in directory '{}'. Output: {:?}",
            project_dir,
            String::from_utf8_lossy(&add_output.stderr)
        );
        return Ok(HttpResponse::InternalServerError().body("Git add failed"));
    }

    println!(
        "Files successfully added to the Git repository in directory '{}'.",
        project_dir
    );

    // Debug: check the contents of the directory before the `git add` command
    println!("Checking directory contents before 'git add':");
    match std::fs::read_dir(&project_dir) {
        Ok(entries) => {
            for entry in entries {
                match entry {
                    Ok(e) => println!("File/folder: {:?}", e.path()),
                    Err(e) => println!("Error reading directory entry: {:?}", e),
                }
            }
        }
        Err(e) => println!(
            "Error reading contents of directory '{}': {:?}",
            project_dir, e
        ),
    }

    // Commit the changes in the repository with a message
    let commit_output = Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg("Initial commit of generated project")
        .current_dir(&project_dir)
        .output()
        .map_err(|e| {
            println!(
                "Error committing files in directory '{}': {:?}",
                project_dir, e
            );
            actix_web::error::ErrorInternalServerError(format!("Failed to commit files: {:?}", e))
        })?;

    // Check if the `git commit` command was successful
    if !commit_output.status.success() {
        println!(
            "Git 'commit' command failed in directory '{}'. Output: {:?}",
            project_dir,
            String::from_utf8_lossy(&commit_output.stderr)
        );
        return Ok(HttpResponse::InternalServerError().body(format!(
            "Git commit failed: {:?}",
            String::from_utf8_lossy(&commit_output.stderr)
        )));
    }

    println!(
        "Changes successfully committed in directory '{}'.",
        project_dir
    );

    // Prepare the remote URL for pushing to GitHub

    let remote_url = format!(
        "https://{}:{}@github.com/{}/{}.git",
        github_username, github_token, github_username, project_name
    );

    // Add the remote origin for the GitHub repository
    let remote_output = Command::new("git")
        .arg("remote")
        .arg("add")
        .arg("origin")
        .arg(&remote_url)
        .current_dir(&project_dir)
        .output()
        .map_err(|e| {
            println!(
                "Failed to add remote origin in directory '{}': {:?}",
                project_dir, e
            );
            actix_web::error::ErrorInternalServerError(format!(
                "Failed to add remote origin: {:?}",
                e
            ))
        })?;

    if !remote_output.status.success() {
        println!(
            "Git 'remote add' command failed in directory '{}'. Output: {:?}",
            project_dir,
            String::from_utf8_lossy(&remote_output.stderr)
        );
        return Ok(HttpResponse::InternalServerError().body("Failed to add remote to GitHub"));
    }

    println!(
        "Remote origin successfully added in directory '{}'.",
        project_dir
    );

    // Set the branch to 'main'
    let branch_output = Command::new("git")
        .arg("branch")
        .arg("-M")
        .arg("main")
        .current_dir(&project_dir)
        .output()
        .map_err(|e| {
            println!(
                "Failed to set branch to 'main' in directory '{}': {:?}",
                project_dir, e
            );
            actix_web::error::ErrorInternalServerError(format!(
                "Failed to set branch to 'main': {:?}",
                e
            ))
        })?;

    if !branch_output.status.success() {
        error!(
            "Git 'branch -M main' command failed in directory '{}'. Output: {:?}",
            project_dir,
            String::from_utf8_lossy(&branch_output.stderr)
        );
        return Ok(HttpResponse::InternalServerError()
            .body("Git branch creation or move to 'main' failed"));
    }

    println!(
        "Branch successfully set to 'main' in directory '{}'.",
        project_dir
    );

    // Push the changes to the GitHub repository using the provided credentials
    let push_output = Command::new("git")
        .arg("push")
        .arg("-u")
        .arg("origin")
        .arg("main")
        .current_dir(&project_dir)
        .output()
        .map_err(|e| {
            println!(
                "Failed to push changes to GitHub from directory '{}': {:?}",
                project_dir, e
            );
            actix_web::error::ErrorInternalServerError(format!("Failed to push to GitHub: {:?}", e))
        })?;

    if !push_output.status.success() {
        println!(
            "Git 'push' command failed for directory '{}'. Output: {:?}",
            project_dir,
            String::from_utf8_lossy(&push_output.stderr)
        );
        return Ok(HttpResponse::InternalServerError().body(format!(
            "Git push failed: {:?}",
            String::from_utf8_lossy(&push_output.stderr)
        )));
    }

    println!(
        "Project '{}' successfully pushed to GitHub from directory '{}'.",
        project_name, project_dir
    );

    // Return a success HTTP response
    Ok(HttpResponse::Ok().body(format!(
        "Project {} successfully pushed to GitHub",
        project_name
    )))
}

/// Creates a new GitHub repository using the GitHub API.
///
/// This function sends an HTTP POST request to the GitHub API to create a new repository
/// under the user's account. It uses basic authentication with the provided username and token.
///
/// # Arguments
///
/// * `github_username` - The GitHub username of the user creating the repository.
/// * `github_token` - The GitHub personal access token for authentication.
/// * `repo_name` - The name of the repository to create.
///
/// # Returns
///
/// Returns a `Result<HttpResponse, Error>`, indicating success or failure of the repository creation.

pub async fn create_github_repo(
    github_username: &str,
    github_token: &str,
    repo_name: &str,
) -> Result<HttpResponse, Error> {
    let client = Client::new();
    let url = "https://api.github.com/user/repos".to_string();

    let body = json!({
        "name": repo_name,
        "private": false,  // Set to true if you want the repository to be private
    });

    // Send the POST request to the GitHub API to create the repository
    let response = client
        .post(&url)
        .basic_auth(github_username, Some(github_token))
        .header("User-Agent", github_username)
        .json(&body)
        .send()
        .await
        .map_err(|e| {
            println!(
                "Failed to send request to create GitHub repository '{}': {}",
                repo_name, e
            );
            actix_web::error::ErrorInternalServerError(format!("Request failed: {}", e))
        })?;

    if response.status().is_success() {
        println!("Successfully created GitHub repository: {}", repo_name);
        Ok(HttpResponse::Ok().body(format!(
            "GitHub repository '{}' created successfully",
            repo_name
        )))
    } else {
        let error_message = response.text().await.map_err(|e| {
            println!(
                "Failed to read response when creating GitHub repository '{}': {}",
                repo_name, e
            );
            actix_web::error::ErrorInternalServerError(format!("Failed to read response: {}", e))
        })?;

        println!(
            "Failed to create GitHub repository '{}': {}",
            repo_name, error_message
        );
        Ok(HttpResponse::InternalServerError().body(format!(
            "Failed to create GitHub repository: {}",
            error_message
        )))
    }
}
