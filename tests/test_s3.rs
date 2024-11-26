use substrate_runtime_builder::services::object_store::{s3::*, ObjectStoreService};

fn env_setup() {
    dotenv::from_filename(".env.local").ok();
}

#[tokio::test]
async fn test_s3_object_store_new() {
    env_setup();
    let s3 = S3ObjectStoreService::new().await;
    assert!(s3.is_ok());
}

#[tokio::test]
async fn test_list_files() {
    env_setup();
    let service = S3ObjectStoreService::new().await;
    assert!(service.is_ok());
    let service = service.unwrap();
    let files = service.list_files().await;
    assert!(files.is_ok());
}

#[tokio::test]
#[serial_test::serial]
async fn test_list_files_and_upload_file() {
    env_setup();
    let service = S3ObjectStoreService::new().await;
    assert!(service.is_ok());
    let service = service.unwrap();
    let files = service.list_files().await;
    assert!(files.is_ok());
    let files = files.unwrap();
    let old_files_count = files.len();
    let file = b"Hello, World!".to_vec();
    let uuid = uuid::Uuid::new_v4();
    let file_name = format!("hello-{}.txt", uuid);
    let result = service.upload_content(file, file_name.as_str()).await;
    assert!(result.is_ok());

    let files = service.list_files().await;
    assert!(files.is_ok());
    let files = files.unwrap();
    println!("{:?}", files);
    assert_eq!(files.len(), old_files_count + 1);
    assert!(files.contains(&file_name));
}
#[tokio::test]
#[serial_test::serial]
async fn test_upload_content_and_get_presigned_url() {
    env_setup();
    let service = S3ObjectStoreService::new().await;
    assert!(service.is_ok());
    let service = service.unwrap();
    let file = b"Hello, World!".to_vec();
    let file_name = "hello.txt";
    let result = service.upload_content(file, file_name).await;
    assert!(result.is_ok());
    let url = service.get_presigned_url(file_name, 3600).await;
    assert!(url.is_ok());
    let url = url.unwrap();
    let response = reqwest::get(url).await;
    assert!(response.is_ok());
    let response = response.unwrap();
    assert!(response.status().is_success());
    let content = response.text().await;
    assert!(content.is_ok());
    let content = content.unwrap();
    assert_eq!(content, "Hello, World!");
}
