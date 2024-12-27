use async_trait::async_trait;
use async_zip::{
    tokio::{read::seek::ZipFileReader, write::ZipFileWriter},
    ZipEntryBuilder,
};
use std::{io::Cursor, path::Path};
use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

use crate::services::traits::archiver::{ArchiverError, ArchiverService};

pub struct AsyncZipArchiverService;

pub fn archive_dir_recursive<'a>(
    src: &'a Path,
    dest: &'a Path,
    ignore_extension: &'a str,
    zip_writter: &'a mut ZipFileWriter<Cursor<Vec<u8>>>,
) -> futures::future::BoxFuture<'a, std::io::Result<()>> {
    Box::pin(async move {
        // Read the entries in the source directory
        let mut entries = tokio::fs::read_dir(src).await?;

        // Iterate over each entry in the directory
        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();
            let extension = path.extension().unwrap_or_default().to_str().unwrap_or("");
            let file_name = entry.file_name();
            let dest_path = dest.join(file_name);

            if path.is_dir() {
                // Recursively copy the subdirectory
                archive_dir_recursive(&path, &dest_path, ignore_extension, zip_writter).await?;
            } else if path.is_file() && extension != ignore_extension {
                // Copy the file to the destination
                let mut file = File::open(&path).await?;
                let mut content = Vec::new();
                file.read_to_end(&mut content).await?;
                let builder = ZipEntryBuilder::new(
                    dest_path.to_str().unwrap().into(),
                    async_zip::Compression::Deflate,
                );
                zip_writter
                    .write_entry_whole(builder, &content)
                    .await
                    .map_err(|_| std::io::ErrorKind::InvalidData)?;
            }
        }

        Ok(())
    })
}

#[async_trait]
impl ArchiverService for AsyncZipArchiverService {
    type ZippedBuffer = ZipFileWriter<Cursor<Vec<u8>>>;

    async fn archive_folder(
        &self,
        template_path: &Path,
        template_extension: &str,
    ) -> Result<Self::ZippedBuffer, ArchiverError> {
        let mut zip_writter = ZipFileWriter::with_tokio(Cursor::new(Vec::new()));
        if let Err(e) = archive_dir_recursive(
            template_path,
            Path::new(""),
            template_extension,
            &mut zip_writter,
        )
        .await
        {
            return Err(ArchiverError::ArchiveError(format!("{:?}", e)));
        }
        Ok(zip_writter)
    }
    async fn close_archive(
        &self,
        zipper_buffer: Self::ZippedBuffer,
    ) -> Result<Vec<u8>, ArchiverError> {
        Ok(zipper_buffer
            .close()
            .await
            .map_err(|e| ArchiverError::CloseError(format!("{}", e)))?
            .into_inner()
            .into_inner())
    }
    async fn add_content_to_archive(
        &self,
        mut zipper_buffer: Self::ZippedBuffer,
        content: &[u8],
        dest_path: &Path,
    ) -> Result<Self::ZippedBuffer, ArchiverError> {
        let builder = ZipEntryBuilder::new(
            dest_path.to_str().unwrap().into(),
            async_zip::Compression::Deflate,
        );
        zipper_buffer
            .write_entry_whole(builder, content)
            .await
            .map_err(|_| ArchiverError::ArchiveError("Failed to write entry".into()))?;
        Ok(zipper_buffer)
    }
    async fn unpack_archive_to_folder(
        &self,
        buffer: Vec<u8>,
        output: &Path,
    ) -> Result<(), ArchiverError> {
        let mut archive = ZipFileReader::with_tokio(Cursor::new(buffer))
            .await
            .unwrap();

        // Iterate over all entries by enumerating them so we have both index and entry
        for i in 0..archive.file().entries().len() {
            let entry = &archive.file().entries()[i];
            let filename = entry.filename();
            let output_path = output.join(filename.as_str().unwrap());

            if entry.dir().unwrap() {
                // Create the directory
                tokio::fs::create_dir_all(&output_path).await.unwrap();
            } else {
                // Ensure the parent directory exists before creating the file
                if let Some(parent) = output_path.parent() {
                    tokio::fs::create_dir_all(parent).await.unwrap();
                }

                // Create the file
                let mut file = tokio::fs::File::create(&output_path).await.unwrap();

                // Get a reader for this entry's data
                let mut reader = archive.reader_with_entry(i).await.unwrap();

                // create buffer
                let mut file_buffer = Vec::new();

                // copy contents into buffer
                reader.read_to_end_checked(&mut file_buffer).await.unwrap();

                // write buffer into file
                file.write_all(&file_buffer).await.unwrap();
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::services::code_generator::templating::handle_templates::HBS_SUFFIX;
    use tmpdir::TmpDir;
    #[tokio::test]
    async fn test_archive_and_close_archive() {
        let archiver = AsyncZipArchiverService;
        let template_path = Path::new("templates/SoloChain");
        let result = archiver.archive_folder(template_path, HBS_SUFFIX).await;
        assert!(result.is_ok());
        let zipper_buffer = result.unwrap();
        let zipped_data = archiver.close_archive(zipper_buffer).await;
        assert!(zipped_data.is_ok());
    }
    #[ignore]
    #[tokio::test]
    async fn test_archive_and_unpack() {
        let archiver = AsyncZipArchiverService;
        let template_path = Path::new("templates/SoloChain");
        let zipper_buffer = archiver
            .archive_folder(template_path, HBS_SUFFIX)
            .await
            .unwrap();
        let zipped_data = archiver.close_archive(zipper_buffer).await.unwrap();
        let tmp = TmpDir::new("output").await.unwrap();
        let x = archiver
            .unpack_archive_to_folder(zipped_data, tmp.as_ref())
            .await;
        println!("{}", tmp.as_ref().file_name().unwrap().to_str().unwrap());
        println!("{}", x.is_ok());
    }
}
