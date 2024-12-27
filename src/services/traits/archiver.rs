use std::path::Path;

use async_trait::async_trait;
use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum ArchiverError {
    #[error("Failed to archive template: {0}")]
    ArchiveError(String),
    #[error("Failed to close archive: {0}")]
    CloseError(String),
}

#[async_trait]
pub trait ArchiverService: Send + Sync {
    type ZippedBuffer;
    async fn archive_folder(
        &self,
        template_path: &Path,
        template_extension: &str,
    ) -> Result<Self::ZippedBuffer, ArchiverError>;
    async fn close_archive(
        &self,
        zipper_buffer: Self::ZippedBuffer,
    ) -> Result<Vec<u8>, ArchiverError>;
    async fn add_content_to_archive(
        &self,
        zipper_buffer: Self::ZippedBuffer,
        content: &[u8],
        dest_path: &Path,
    ) -> Result<Self::ZippedBuffer, ArchiverError>;
    async fn unpack_archive_to_folder(
        &self,
        buffer: Vec<u8>,
        output: &Path,
    ) -> Result<(), ArchiverError>;
}
