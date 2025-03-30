use chrono::{DateTime, Utc};

// Recording: a meeting or discussion recording that is stored in S3
#[derive(Default, Debug)]
pub struct Recording {
    pub id: String,
    pub source: Option<String>,
    pub date: Option<DateTime<Utc>>,
    pub duration: Option<i32>,
    pub number_of_speakers: Option<i32>,
    pub language: Option<String>,
}