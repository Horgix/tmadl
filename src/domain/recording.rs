use chrono::{DateTime, Utc};

use crate::domain::transcription::Transcription;

#[derive(Default, Debug)]
pub struct Recording {
    pub id: String,
    pub source: Option<String>,
    pub date: Option<DateTime<Utc>>,
    pub duration: Option<i32>, // Duration in seconds
    pub number_of_speakers: Option<i32>,
    pub language: Option<String>,
    pub transcription: Option<Transcription>,
}