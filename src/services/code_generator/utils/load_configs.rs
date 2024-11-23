use std::{collections::HashMap, path::PathBuf};

use super::super::types::PalletConfig;
use thiserror::Error;

// Define the LoadConfigsError
#[derive(Error, Debug, Clone)]
pub enum LoadConfigsError {
    #[error("{0}")]
    FileIOError(String),
    #[error("{0}")]
    TomlError(#[from] toml::de::Error),
}

impl From<std::io::Error> for LoadConfigsError {
    fn from(e: std::io::Error) -> Self {
        LoadConfigsError::FileIOError(e.to_string())
    }
}

pub async fn load_configs(
    path: PathBuf,
) -> Result<HashMap<String, PalletConfig>, LoadConfigsError> {
    let mut read_dir = tokio::fs::read_dir(&path).await?;

    let mut dir_entries = vec![];
    while let Some(entry) = read_dir.next_entry().await? {
        dir_entries.push(entry);
    }
    // Filter and read .toml files into strings
    let file_paths = dir_entries
        .into_iter()
        // Filter out non-.toml files
        .filter_map(|entry| {
            let path = entry.path();
            if path.extension().and_then(|ext| ext.to_str()) == Some("toml") {
                Some(path)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    let mut toml_strings_no_updated = vec![];
    for file_path in file_paths {
        let string = tokio::fs::read_to_string(&file_path).await?;
        toml_strings_no_updated.push(string);
    }
    toml_strings_no_updated
        .into_iter()
        .map(|x| match toml::from_str::<PalletConfig>(x.as_str()) {
            Ok(pallet_config) => Ok((pallet_config.name.clone(), pallet_config)),
            Err(e) => Err(e.into()),
        })
        .collect::<Result<HashMap<String, PalletConfig>, LoadConfigsError>>()
}
