use async_trait::async_trait;

#[async_trait]
pub trait ObjectStoreService: Send + Sync {
    async fn upload_content(&self, file: Vec<u8>, file_name: &str) -> Result<(), String>;
    async fn get_presigned_url(&self, file_name: &str, ttl: u64) -> Result<String, String>;
    async fn list_files(&self) -> Result<Vec<String>, String>;
}
