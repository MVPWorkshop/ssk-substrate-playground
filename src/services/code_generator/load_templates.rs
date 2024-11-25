use std::collections::HashMap;
use std::path::PathBuf;
use thiserror::Error;
use tokio::fs;

// Define the LoadTemplatesError
#[derive(Error, Debug, Clone)]
pub enum LoadTemplatesError {
    #[error("Failed to read directory: {0}")]
    ReadDirError(String),
}

impl From<std::io::Error> for LoadTemplatesError {
    fn from(e: std::io::Error) -> Self {
        LoadTemplatesError::ReadDirError(e.to_string())
    }
}

// The async function to take a path and return the desired map
pub async fn load_templates(
    path: PathBuf,
) -> Result<HashMap<String, HashMap<String, String>>, LoadTemplatesError> {
    let mut result_map = HashMap::new();

    let mut entries = fs::read_dir(&path).await?;
    while let Some(entry) = entries.next_entry().await? {
        let entry_path = entry.path();
        if entry_path.is_dir() {
            let dir_name = entry_path
                .file_name()
                .and_then(|name| name.to_str())
                .unwrap_or_default()
                .to_string();

            let nested_map = get_nested_directories(&entry_path).await?;
            result_map.insert(dir_name, nested_map);
        }
    }

    Ok(result_map)
}

// Helper function to get nested directories
async fn get_nested_directories(
    path: &PathBuf,
) -> Result<HashMap<String, String>, LoadTemplatesError> {
    let mut nested_map = HashMap::new();

    let mut entries = fs::read_dir(path).await?;
    while let Some(entry) = entries.next_entry().await? {
        let entry_path = entry.path();
        if entry_path.is_dir() {
            let dir_name = entry_path
                .file_name()
                .and_then(|name| name.to_str())
                .unwrap_or_default()
                .to_string();

            nested_map.insert(dir_name.clone(), dir_name);
        }
    }

    Ok(nested_map)
}
