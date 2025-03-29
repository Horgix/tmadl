use std::env;

mod domain;
use domain::recording_store::RecordingStore;
use domain::recording_store::S3RecordingStore;

fn main() {
    println!("Hello, world!");
    // Use a recordingstore
    let s3_bucket = env::var("MNDRP_S3_BUCKET_NAME").unwrap();
    let store = S3RecordingStore::new(&s3_bucket);
    let recordings = store.get_all();
    for recording in recordings {
        println!("Recording: {}", recording);
    }
}
