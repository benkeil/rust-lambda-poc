use async_trait::async_trait;

#[async_trait]
pub trait StorageRepository {
    async fn list(&self) -> Vec<String>;
}
