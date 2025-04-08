use chrono::{DateTime, Utc};
use tabled::Tabled;

use crate::domain::transcription::Transcription;

#[derive(Default, Debug, Tabled)]
#[tabled(display(Option, "tabled::derive::display::option", "N/A"))]
pub struct Recording {
    pub id: String,
    pub source: Option<String>,
    pub date_time: Option<DateTime<Utc>>,
    pub duration: Option<i32>, // Duration in seconds
    pub number_of_speakers: Option<i32>,
    pub language: Option<String>,
    #[tabled(skip)]
    pub description: Option<String>,
    #[tabled(skip)]
    pub transcription: Option<Transcription>,
}