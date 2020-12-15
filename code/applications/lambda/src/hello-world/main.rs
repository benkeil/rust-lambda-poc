use lambda::{lambda, Context};
use aws_lambda_events::event::sqs::SqsEvent;
use rusoto_core::Region;
use rusoto_s3::{S3, S3Client};
use core::use_cases::hello_world_use_case::{HelloWorldUseCase, Duck, UseCase};
use adapters::repositories::s3_storage_repository::S3StorageRepository;

type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

#[lambda]
#[tokio::main]
async fn main(event: SqsEvent, c: Context) -> Result<(), Error> {
    println!("Hello, world! - {}", c.request_id);
    println!("Record length: - {}", event.records.len());
    for r in event.records {
        println!("Hello, {}", r.body.unwrap_or(String::from("World")));
    }
    println!("Env: - {:?}", c.env_config);

    let client = S3Client::new(Region::EuCentral1);
    let storage_repository = S3StorageRepository { client };
    let use_case = HelloWorldUseCase { storage_repository };
    use_case.execute();

    match client.list_buckets().await {
        Ok(output) => {
            match output.buckets {
                Some(buckets) => {
                    for bucket in buckets {
                        println!("{}", bucket.name.unwrap());
                    }
                }
                None => println!("no buckets found"),
            }
        }
        Err(error) => panic!(error),
    }

    Ok(())
}
