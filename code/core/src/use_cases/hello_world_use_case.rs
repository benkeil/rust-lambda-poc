use async_trait::async_trait;
use crate::repositories::storage_repository::StorageRepository;

#[async_trait]
pub trait UseCase {
    async fn execute(&self) -> Send;
}

pub struct HelloWorldUseCase {
    storage_repository: dyn StorageRepository
}

#[async_trait]
impl UseCase for HelloWorldUseCase {
    async fn execute(&self) {
        let list = self.storage_repository.list();
        let list = list.await;
        println!("Bucket: {:?}", list);
        // for word in list {
        //     println!("Bucket: {:?}", word);
        // }
    }
}
