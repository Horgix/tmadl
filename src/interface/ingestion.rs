use std::error::Error;
use std::fs;
use std::io;
use std::path::Path;
use chrono::DateTime;
use crate::domain::recording::Recording;

pub fn parse_recording_information_from_local_mp3_file(file_path: &str) -> Result<Recording, Box<dyn Error>> {
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
    // Only keep the date and time part to the minute in ISO 8601 format.
    let metadata = fs::metadata(file_path)?;
    let created_at = DateTime::from(metadata.created().unwrap());
    let shortened_created_at = created_at.format("%Y-%m-%dT%H:%M:%S").to_string();
    println!("Recording created at: {}", shortened_created_at);
    // Prompt user for a different date and time if desired
    println!("Press Enter to use this date and time, or type a new one and press Enter");
    let mut new_date_time = String::new();
    io::stdin().read_line(&mut new_date_time)?;
    let created_at = if new_date_time.trim().is_empty() {
        created_at
    } else {
        let new_date_time = new_date_time.trim();
        // Interpret it as UTC
        let new_date_time = DateTime::parse_from_rfc3339(new_date_time)?;
        new_date_time.with_timezone(&chrono::Utc)
    };
    println!("Recording date and time: {}", created_at);


    // Prompt user for recording source. Default to "Google Meet"
    println!("Recording source (default: gmeet):");
    let mut source = String::new();
    io::stdin().read_line(&mut source)?;
    let source = if source.trim().is_empty() {
        "gmeet".to_string()
    } else {
        source.trim().to_string()
    };
    println!("Recording source: {}", source);

    // Prompt user for the number of speakers in the recording
    println!("How many speakers are in the recording?");
    let mut number_of_speakers = String::new();
    io::stdin().read_line(&mut number_of_speakers)?;
    let number_of_speakers = number_of_speakers.trim().parse::<i32>().unwrap();

    // Prompt user for the description of the recording. Default to None
    println!("What is the description of the recording?");
    let mut description = String::new();
    io::stdin().read_line(&mut description)?;
    let description = if description.trim().is_empty() {
        None
    } else {
        Some(description.trim().to_string())
    };

    // Prompt user for the language of the recording. Default to "English"
    println!("What language is the recording in? (default: English)");
    let mut language = String::new();
    io::stdin().read_line(&mut language)?;
    let language = if language.trim().is_empty() {
        "English".to_string()
    } else {
        language.trim().to_string()
    };
    println!("Recording language: {}", language);

    let recording = Recording {
        id: recording_id,
        source: Some(source),
        date_time: Some(created_at),
        duration: Some(0), // TODO (Placeholder for duration)
        number_of_speakers: Some(number_of_speakers),
        language: Some(language),
        description,
        transcription: None, // TODO (Placeholder for transcription)
    };
    println!("Recording: {:?}", recording);

    Ok(recording)
}