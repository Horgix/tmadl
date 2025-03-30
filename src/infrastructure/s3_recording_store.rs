use aws_config::BehaviorVersion;
use aws_sdk_s3::Client;
use tokio;

use crate::domain::recording_store::RecordingStore;
use crate::domain::recording::Recording;

static S3_RAW_RECORDINGS_PATH: &str = "raw-recordings/";

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
    async fn get_all(&self) -> Vec<Recording> {
        let mut response = self.client
            .list_objects_v2()
            .bucket(self.bucket.to_owned())
            .prefix(S3_RAW_RECORDINGS_PATH)
            .max_keys(10) // In this example, go 10 at a time.
            .into_paginator()
            .send();

        let mut recordings : Vec<Recording> = Vec::new();
        while let Some(result) = response.next().await {
            match result {
                Ok(output) => {
                    let current_recordings = output.contents().iter().filter_map(|object| {
                        if object.key().unwrap_or_default() != S3_RAW_RECORDINGS_PATH {
                            Some(
                                Recording{
                                    id: object.key().unwrap().to_owned(),
                                    ..Default::default()
                                }
                            )
                        } else {
                            None
                        }
                    }).collect::<Vec<Recording>>();
                    recordings.extend(current_recordings);
                }
                Err(err) => {
                    eprintln!("Failed to fetch recording list from S3 objects: {err:?}")
                }
            }
        }
        recordings
    }
}