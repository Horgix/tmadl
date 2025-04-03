use std::env;
use clap::{Parser, Subcommand};

mod domain;
use domain::recording_store::RecordingStore;

mod infrastructure;
use infrastructure::s3_recording_store::S3RecordingStore;
use infrastructure::claude_summarizer::{self, summarize};

mod interface;
use interface::ingestion;


/// Main CLI structure
#[derive(Parser, Debug)]
#[command(
    name = "tmadl",
    version = "0.1",
    author = "Alexis Horgix Chotard <tmadl@foss.horgix.fr>",
    about = "TMA;DL (Too Much Audio, Didn't Listen) — A terminal/CLI meetings & discussions recordings processor using AI",
    arg_required_else_help = true,
)]
struct TmadlCli {
    /// Subcommands
    #[command(subcommand)]
    command: Option<TmadlSubcommands>,
}

/// Top-level subcommands
#[derive(Subcommand, Debug)]
enum TmadlSubcommands {
    Upload {
        /// File path of the local recording
        #[arg(long, value_name = "FILE_PATH", help = "Path to the local recording file")]
        path: String,
    },

    /// Record a new meeting/conversation
    Record {
        /// Path to save the recording
        #[arg(long, value_name = "FILE_PATH", help = "Path to save the recording")]
        path: String,
    },
}

fn main() {

    // let cli = TmadlCli::parse();

    // // When using upload, call ingestion parse recording etc
    // match &cli.command {
    //     Some(TmadlSubcommands::Upload { path }) => {
    //         println!("Uploading recording from path: {}", path);
    //         let recording = ingestion::parse_recording_information_from_local_mp3_file(path);
    //         let s3_bucket = env::var("TMADL_S3_BUCKET_NAME").unwrap();
    //         let store = S3RecordingStore::new(&s3_bucket);
    //         store.send_local_recording(&recording.unwrap(), path).unwrap();
    //     }
    //     _ => {
    //         // Not implemented
    //         println!("No subcommand provided or not implemented yet.");
    //     }
    // }

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
