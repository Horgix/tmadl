use std::collections::HashMap;

use aws_config::BehaviorVersion;
use aws_sdk_s3::Client as S3Client;
use tokio;

use crate::domain::recording_store::RecordingStore;
use crate::domain::recording::Recording;

static S3_RAW_RECORDINGS_PATH: &str = "raw-recordings/";

pub struct S3RecordingStore {
    bucket: String,
    client: S3Client,
}

impl S3RecordingStore {
    #[tokio::main(flavor = "current_thread")]
    pub async fn new(bucket: &str) -> S3RecordingStore {
        let config = aws_config::defaults(BehaviorVersion::latest())
                .load()
                .await;
        let client = S3Client::new(&config);

        S3RecordingStore {
            bucket: bucket.to_owned(),
            client: client,
        }
    }

    fn recording_metadata_to_s3_tags(&self, recording: &Recording) -> Result<String, aws_sdk_s3::error::BuildError> {
        let mut tags = HashMap::new();
        tags.insert("recording_id".to_string(), recording.id.to_string());
        if let Some(source) = &recording.source {
            tags.insert("source".to_string(), source.to_string());
        }
        if let Some(date) = &recording.date_time { // As ISO 8601 to the second
            tags.insert("date".to_string(), date.to_rfc3339_opts(chrono::SecondsFormat::Secs, true));
        }
        if let Some(duration) = &recording.duration {
            tags.insert("duration".to_string(), duration.to_string());
        }
        if let Some(number_of_speakers) = &recording.number_of_speakers {
            tags.insert("number_of_speakers".to_string(), number_of_speakers.to_string());
        }
        if let Some(description) = &recording.description {
            tags.insert("description".to_string(), description.to_string());
        }
        if let Some(language) = &recording.language {
            tags.insert("language".to_string(), language.to_string());
        }

        let mut s3_tags = aws_sdk_s3::types::Tagging::builder();
        for (key, value) in tags {
            s3_tags = s3_tags.tag_set(
                aws_sdk_s3::types::Tag::builder()
                    .key(key)
                    .value(value)
                    .build()
                    .unwrap()
            );
        }

        Ok(s3_tags.build()?
            .tag_set()
            .iter()
            .map(|tag| format!("{}={}", tag.key(), tag.value()))
            .collect::<Vec<String>>()
            .join("&"))
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
            .max_keys(10)
            .into_paginator()
            .send();

        let mut recordings : Vec<Recording> = Vec::new();
        while let Some(result) = response.next().await {
            match result {
                Ok(output) => {
                    let current_recordings = output.contents().iter().filter_map(|object| {
                        if object.key().unwrap_or_default() != S3_RAW_RECORDINGS_PATH { // Skip the directory itself
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
        // Get the metadata for each recording from S3 tags
        for recording in &mut recordings {
            let object_tags = self.client
                .get_object_tagging()
                .bucket(self.bucket.to_owned())
                .key(recording.id.to_owned())
                .send()
                .await;
            match object_tags {
                Ok(tags) => {
                    for tag in tags.tag_set() {
                        match tag.key() {
                            "source" => recording.source = Some(tag.value().to_string()),
                            "date" => recording.date_time = Some(tag.value().parse::<chrono::DateTime<chrono::Utc>>().unwrap()),
                            "duration" => recording.duration = Some(tag.value().parse::<i32>().unwrap()),
                            "number_of_speakers" => recording.number_of_speakers = Some(tag.value().parse::<i32>().unwrap()),
                            "language" => recording.language = Some(tag.value().to_string()),
                            _ => {}
                        }
                    }
                }
                Err(err) => {
                    eprintln!("Failed to fetch tags for recording {}: {err:?}", recording.id);
                }
            }
        }
        recordings
    }

    #[tokio::main(flavor = "current_thread")]
    async fn send_local_recording(&self, recording: &Recording, file_path: &str) -> Result<(), String> {
        let content = aws_sdk_s3::primitives::ByteStream::from_path(std::path::Path::new(file_path)).await;
        let tags = self.recording_metadata_to_s3_tags(recording);
        println!("Tags: {:?}", tags);

        let file_name = format!("{}{}.mp3", S3_RAW_RECORDINGS_PATH, recording.id);
        let result = self.client
            .put_object()
            .bucket(self.bucket.to_owned())
            .key(file_name)
            .tagging(tags.unwrap())
            .body(content.unwrap())
            .send()
            .await;

        match result {
            Ok(_) => Ok(()),
            Err(err) => Err(format!("Failed to upload recording to S3: {err}")),
        } 
    }
}