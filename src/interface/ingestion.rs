use std::error::Error;
use std::fs;
use std::io;
use std::path::Path;
use std::time::{SystemTime, Duration};
use chrono::{DateTime, Utc, NaiveDateTime};
use crate::domain::recording::Recording;

pub fn ingest_local_mp3_file(file_path: &str) -> Result<(), Box<dyn Error>> {

    // Use the file basename as the recording ID by default, but prompt user for a different ID if desired
    let file_name = Path::new(file_path).file_name().unwrap().to_str().unwrap();
    let recording_id = file_name.split('.').next().unwrap().to_string();
    println!("Recording ID: {}", recording_id);
    println!("Press Enter to use this ID, or type a new one and press Enter");
    let mut new_id = String::new();
    io::stdin().read_line(&mut new_id)?;
    let recording_id = if new_id.trim().is_empty() {
        recording_id
    } else {
        new_id.trim().to_string()
    };

    // Use the file creation timedate as the recording time
    let metadata = fs::metadata(file_path)?;
    let created_at = DateTime::from(metadata.created().unwrap());
    println!("Recording created at: {:?}", created_at);

    // Prompt user for the number of speakers in the recording
    println!("How many speakers are in the recording?");
    let mut number_of_speakers = String::new();
    io::stdin().read_line(&mut number_of_speakers)?;
    let number_of_speakers = number_of_speakers.trim().parse::<i32>().unwrap();

    // Prompt user for the language of the recording
    println!("What language is the recording in?");
    let mut language = String::new();
    io::stdin().read_line(&mut language)?;
    let language = language.trim().to_string();

    let recording = Recording {
        id: recording_id,
        source: Some(file_path.to_string()),
        date: Some(created_at),
        number_of_speakers: Some(number_of_speakers),
        language: Some(language),
        ..Default::default()
    };
    println!("Recording: {:?}", recording);
    return Ok(());
}