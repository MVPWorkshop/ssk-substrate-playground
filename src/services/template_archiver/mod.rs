pub mod async_zip;

use std::path::Path;

use async_trait::async_trait;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TemplateArchiverError {
    #[error("Failed to archive template: {0}")]
    ArchiveError(String),
    #[error("Failed to close archive: {0}")]
    CloseError(String),
}

#[async_trait]
pub trait TemplateArchiverService {
    type ZippedBuffer;
    async fn archive_template<'a>(
        &self,
        template_path: &'a Path,
        template_extension: &str,
    ) -> Result<Self::ZippedBuffer, TemplateArchiverError>;
    async fn close_archive(
        &self,
        zipper_buffer: Self::ZippedBuffer,
    ) -> Result<Vec<u8>, TemplateArchiverError>;
    async fn add_content_to_archive(
        &self,
        zipper_buffer: Self::ZippedBuffer,
        content: &[u8],
        dest_path: &Path,
    ) -> Result<Self::ZippedBuffer, TemplateArchiverError>;
}
