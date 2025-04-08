use std::env;
use clap::{Parser, Subcommand};
use tabled::{Tabled, Table};

mod domain;
use domain::recording::Recording;
use domain::recording_store::RecordingStore;

mod infrastructure;
use infrastructure::s3_recording_store::S3RecordingStore;
use infrastructure::claude_summarizer::{self, get_prompt};

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
    /// List all recordings (and their transcription statuses TODO)
    List {},

    /// Upload a local recording to S3
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

    /// Interrogate a transcription
    Ask {
        /// The transcription to interrogate
        #[arg(long, value_name = "TRANSCRIPTION", help = "The transcription to interrogate")]
        transcription: String,
    }
}

fn main() {

    let cli = TmadlCli::parse();

    // When using upload, call ingestion parse recording etc
    match &cli.command {
        Some(TmadlSubcommands::List {}) => {
            println!("Listing all recordings...");
            let s3_bucket = env::var("TMADL_S3_BUCKET_NAME").unwrap();
            let store = S3RecordingStore::new(&s3_bucket);
            let mut recordings = store.get_all();

            // Sort recordings by date (descending)
            recordings.sort_unstable_by_key(|r| r.date_time);
            recordings.reverse();
            let table = Table::new(recordings).to_string();
            println!("{}", table);

        }
        Some(TmadlSubcommands::Upload { path }) => {
            println!("Uploading recording from path: {}", path);
            let recording = ingestion::parse_recording_information_from_local_mp3_file(path);
            let s3_bucket = env::var("TMADL_S3_BUCKET_NAME").unwrap();
            let store = S3RecordingStore::new(&s3_bucket);
            store.send_local_recording(&recording.unwrap(), path).unwrap();
        }
        Some(TmadlSubcommands::Ask { transcription }) => {
            println!("Interrogating transcription: {}", transcription);
            let mock_recording = Recording{
                id: "FIXME_THIS_IS_A_PLACEHOLDER".to_string(),
                source: Some("PLACEHOLDER".to_string()),
                date_time: Some(chrono::Utc::now()),
                duration: Some(3600),
                number_of_speakers: Some(2),
                language: Some("French".to_string()),
                description: Some("FIXME_THIS_IS_A_PLACEHOLDER".to_string()),
                transcription: None,
            };

            let input = get_prompt(
                domain::summary::SummaryRequest {
                    recording: mock_recording,
                    additional_context: Some(vec![
                        "FIXME_THIS_IS_A_PLACEHOLDER".to_string(),
                    ]),
                }
            );
            let claude_summarizer = claude_summarizer::ClaudeSummarizer::new();
            claude_summarizer.summarize(input.as_str());
        }
        _ => {
            // Not implemented
            println!("No subcommand provided or not implemented yet.");
        }
    }
}