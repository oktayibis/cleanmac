use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum FileType {
    File,
    Directory,
    Symlink,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileEntry {
    pub path: PathBuf,
    pub name: String,
    pub size: u64,
    pub file_type: FileType,
    pub created: Option<DateTime<Utc>>,
    pub modified: Option<DateTime<Utc>>,
    pub accessed: Option<DateTime<Utc>>,
    pub extension: Option<String>,
}

impl FileEntry {
    pub fn new(
        path: PathBuf,
        size: u64,
        file_type: FileType,
        modified: Option<DateTime<Utc>>,
    ) -> Self {
        let name = path
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_default();

        let extension = path.extension().map(|e| e.to_string_lossy().to_string());

        Self {
            path,
            name,
            size,
            file_type,
            created: None,
            modified,
            accessed: None,
            extension,
        }
    }
}
