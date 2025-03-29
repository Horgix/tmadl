use aws_config::BehaviorVersion;
use aws_sdk_s3::{Client, Error};
use tokio::runtime::Runtime;
use tokio;

use crate::domain::recording_store::RecordingStore;

pub struct S3RecordingStore {
    // region: String,
    bucket: String,
    client: Client,
}

impl S3RecordingStore {
    #[tokio::main(flavor = "current_thread")]
    pub async fn new(bucket: &str) -> S3RecordingStore {
        let config = aws_config::defaults(BehaviorVersion::latest())
                .load()
                .await;
        let client = Client::new(&config);

        S3RecordingStore {
            bucket: bucket.to_owned(),
            client: client,
        }
    }
}

// Implement the RecordingStore trait for S3RecordingStore
impl RecordingStore for S3RecordingStore {

    #[tokio::main(flavor = "current_thread")]
    async fn get_all(&self) -> Vec<String> {
        let mut response = self.client
            .list_objects_v2()
            .bucket(self.bucket.to_owned())
            .max_keys(10) // In this example, go 10 at a time.
            .into_paginator()
            .send();

        while let Some(result) = response.next().await {
            match result {
                Ok(output) => {
                    for object in output.contents() {
                        println!(" - {}", object.key().unwrap_or("Unknown"));
                    }
                }
                Err(err) => {
                    eprintln!("Failed to fetch recording list from S3 objects: {err:?}")
                }
            }
        }
        Vec::new()
    }
}