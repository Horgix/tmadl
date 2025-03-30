use crate::domain::recording::Recording;

pub trait RecordingStore {
    fn get_all(&self) -> Vec<Recording>;
}