use crate::domain::recording::Recording;

pub trait RecordingStore {
    fn get_all(&self) -> Vec<Recording>;
    fn send_local_recording(&self, recording: &Recording, file_path: &str) -> Result<(), String>;
}