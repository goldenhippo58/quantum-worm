use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct FileMetadata {
    pub filename: String,
    pub filesize: u64,
}

pub fn serialize_metadata(metadata: &FileMetadata) -> Vec<u8> {
    bincode::serialize(metadata).unwrap()
}

pub fn deserialize_metadata(data: &[u8]) -> FileMetadata {
    bincode::deserialize(data).unwrap()
}
