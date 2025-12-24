use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileEntry {
    pub path: PathBuf,
    pub size: u64,
    pub modified: i64,          // Unix timestamp
    pub accessed: Option<i64>,  // Last access time
    pub file_type: FileType,
    pub category: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum FileType {
    File,
    Directory,
    Symlink,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_entry_serialization() {
        let entry = FileEntry {
            path: PathBuf::from("/tmp/test.txt"),
            size: 1024,
            modified: 1600000000,
            accessed: Some(1600000100),
            file_type: FileType::File,
            category: Some("cache".to_string()),
        };

        let json = serde_json::to_string(&entry).unwrap();
        let deserialized: FileEntry = serde_json::from_str(&json).unwrap();

        assert_eq!(entry.path, deserialized.path);
        assert_eq!(entry.size, deserialized.size);
        assert_eq!(entry.file_type, deserialized.file_type);
        assert_eq!(entry.category, deserialized.category);
    }
}
