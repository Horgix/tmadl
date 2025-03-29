use chrono::{DateTime, Utc};

// Recording: a meeting or discussion recording that is stored in S3
pub struct Recording {
    pub id: String,
    pub source: String,
    pub date: DateTime<Utc>,
    pub duration: i32,
    pub number_of_speakers: i32,
    pub language: String,
}