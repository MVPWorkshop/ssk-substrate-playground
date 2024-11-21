pub mod s3;

pub trait ObjectStoreService {
    fn upload_content_with_ttl(
        &self,
        file: Vec<u8>,
        file_name: &str,
        ttl: i64,
    ) -> Result<(), String>;
    fn get_presigned_url(&self, file_name: &str) -> Result<String, String>;
    fn list_files(&self) -> Result<Vec<String>, String>;
}
