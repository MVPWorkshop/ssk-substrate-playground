use std::time::Duration;

use async_trait::async_trait;
use aws_sdk_s3::{
    config::{Builder as S3ConfigBuilder, Config as S3Config, Region as S3Region},
    presigning::PresigningConfig,
    primitives::ByteStream,
    Client as S3Client,
};
use log::info;

use super::ObjectStoreService;

pub struct S3ObjectStoreService {
    config: S3Config,
    bucket: String,
}

impl S3ObjectStoreService {
    pub async fn new() -> Result<Self, String> {
        let aws_region = std::env::var("AWS_DEFAULT_REGION").map_err(|e| format!("{:?}", e))?;
        let aws_endpoint = std::env::var("AWS_ENDPOINT_URL").map_err(|e| format!("{:?}", e))?;
        let bucket = std::env::var("AWS_ARCHIVE_BUCKET").map_err(|e| format!("{:?}", e))?;
        info!("Creating S3 Object Store Service...");
        info!("AWS Region: {}", aws_region);
        info!("AWS Endpoint: {}", aws_endpoint);
        info!("AWS Archive Bucket: {}", bucket);
        // Create an AWS Config
        let config = aws_config::from_env()
            .region(S3Region::new(aws_region))
            .endpoint_url(aws_endpoint)
            .load()
            .await;
        // Build the S3 client configuration with force_path_style set to true
        let config = S3ConfigBuilder::from(&config)
            .force_path_style(true)
            .build();
        // make sure bucket exists if it doesnt create it.
        info!("AWS Config: {:?}", config);
        Ok(Self { config, bucket })
    }
    #[allow(dead_code)]
    const fn config(&self) -> &S3Config {
        &self.config
    }
    #[allow(dead_code)]
    fn bucket(&self) -> &str {
        self.bucket.as_str()
    }
}

#[async_trait]
impl ObjectStoreService for S3ObjectStoreService {
    async fn upload_content(&self, file: Vec<u8>, file_name: &str) -> Result<(), String> {
        // Calculate the Expires header (1 hour from now)
        let client = S3Client::from_conf(self.config.clone());
        let response = client
            .put_object()
            .bucket(&self.bucket)
            .key(file_name)
            .body(ByteStream::from(file))
            .send()
            .await
            .map_err(|e| format!("{:?}", e))?;
        println!("Response: {:?}", response);
        Ok(())
    }

    async fn get_presigned_url(&self, file_name: &str, ttl: u64) -> Result<String, String> {
        let client = S3Client::from_conf(self.config.clone());
        let presigning_config = PresigningConfig::expires_in(Duration::from_secs(ttl))
            .map_err(|e| format!("{:?}", e))?;
        let req = client
            .get_object()
            .bucket(&self.bucket)
            .key(file_name)
            .presigned(presigning_config)
            .await
            .map_err(|e| format!("{:?}", e))?;
        Ok(req.uri().to_string())
    }
    async fn list_files(&self) -> Result<Vec<String>, String> {
        let client = S3Client::from_conf(self.config.clone());
        let response = client
            .list_objects_v2()
            .bucket(&self.bucket)
            .send()
            .await
            .map_err(|e| format!("{:?}", e))?;
        let mut files = vec![];
        for object in response.contents.unwrap_or_default() {
            files.push(object.key.unwrap_or_default());
        }
        Ok(files)
    }
}
