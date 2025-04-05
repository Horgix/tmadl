use crate::domain::recording::Recording;

pub struct SummaryRequest {
    pub recording: Recording,
    pub additional_context: Option<Vec<String>>, // E.g. e.g. "the recording speakers are inconsistent"
}