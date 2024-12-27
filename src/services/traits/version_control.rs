use std::path::Path;

use async_trait::async_trait;
use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum VersionControlError {
    #[error("{0}")]
    FileError(String),
    #[error("{0}")]
    RepoError(String),
    #[error("{0}")]
    RemoteError(String),
}

#[async_trait]
pub trait VersionControlService: Send + Sync {
    async fn create_remote_repo(
        &self,
        username: &str,
        token: &str,
        repo_name: &str,
    ) -> Result<(), VersionControlError>;
    async fn push_folder_to_repo(
        &self,
        folder: &Path,
        repo_name: &str,
        username: &str,
        token: &str,
        email: &str,
    ) -> Result<(), VersionControlError>;
}
