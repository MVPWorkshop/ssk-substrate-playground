use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

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

pub fn copy_dir_recursive<'a>(
    src: &'a Path,
    dest: &'a Path,
) -> futures::future::BoxFuture<'a, io::Result<()>> {
    Box::pin(async move {
        if !dest.exists() {
            tokio::fs::create_dir_all(dest).await?;
        }

        // Read the entries in the source directory
        let mut entries = tokio::fs::read_dir(src).await?;

        // Iterate over each entry in the directory
        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();
            let file_name = entry.file_name();
            let dest_path = dest.join(file_name);

            if path.is_dir() {
                // Recursively copy the subdirectory
                copy_dir_recursive(&path, &dest_path).await?;
            } else if path.is_file() {
                // Copy the file to the destination
                tokio::fs::copy(&path, &dest_path).await?;
            }
        }

        Ok(())
    })
}

pub async fn create_new_folder(base_path: &Path, folder_name: &str) -> io::Result<()> {
    let new_folder_path = base_path.join(folder_name);

    if tokio::fs::metadata(&new_folder_path).await.is_ok() {
        println!("Folder already exists at: {:?}", new_folder_path);
    } else {
        tokio::fs::create_dir_all(&new_folder_path).await?;
        println!("Folder created successfully at: {:?}", new_folder_path);
    }
    Ok(())
}
