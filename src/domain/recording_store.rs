pub trait RecordingStore {
    fn get_all(&self) -> Vec<String>;
}