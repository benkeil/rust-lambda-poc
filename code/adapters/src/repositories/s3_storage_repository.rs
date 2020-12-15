use core::repositories::storage_repository::StorageRepository;
use rusoto_s3::{S3, S3Client};

pub struct S3StorageRepository {
    client: S3Client
}

#[async_trait]
impl StorageRepository for S3StorageRepository {
    async fn list(&self) -> Vec<String> {
        self.client.list_buckets().await;
        return vec!["hello".into_string(), "world".into_string()];
    }
}
