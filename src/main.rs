use std::env;

mod domain;
use domain::recording_store::RecordingStore;

mod infrastructure;
use infrastructure::s3_recording_store::S3RecordingStore;
use infrastructure::claude_summarizer::{self, summarize};

mod interface;
use interface::ingestion;

fn main() {
    // let recording = ingestion::parse_recording_information_from_local_mp3_file("/tmp/foo.mp3");
    let s3_bucket = env::var("TMADL_S3_BUCKET_NAME").unwrap();
    let store = S3RecordingStore::new(&s3_bucket);
    let recordings = store.get_all();
    for recording in recordings {
        println!("Recording: {:?}", recording);
    }

    // store.send_local_recording(&recording.unwrap(), "/tmp/foo.mp3").unwrap();

    let input = summarize();
    let claude_summarizer = claude_summarizer::ClaudeSummarizer::new();
    claude_summarizer.summarize(input.as_str());

}
