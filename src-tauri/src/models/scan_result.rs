use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheScanResult {
    pub total_size: u64,
    pub categories: Vec<CacheCategory>,
    pub scanned_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheCategory {
    pub name: String,
    pub category_type: CacheCategoryType,
    pub total_size: u64,
    pub items: Vec<CacheItem>,
    pub is_protected: bool,
    pub protection_reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CacheCategoryType {
    Browser,
    System,
    Application,
    Developer,
    Temporary,
    Logs,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheItem {
    pub path: PathBuf,
    pub name: String,
    pub size: u64,
    pub age_days: Option<u32>,
    pub app_name: Option<String>,
    pub bundle_id: Option<String>,
    pub safe_to_delete: SafetyLevel,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SafetyLevel {
    Safe,
    Caution,
    Protected,
    Unknown,
}

// Phase 3: Orphans
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrphanScanResult {
    pub total_size: u64,
    pub orphaned_apps: Vec<OrphanedApp>,
    pub scanned_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrphanedApp {
    pub presumed_name: String,
    pub bundle_id: Option<String>,
    pub total_size: u64,
    pub files: Vec<OrphanedFile>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrphanedFile {
    pub path: PathBuf,
    pub size: u64,
    pub file_type: OrphanFileType,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum OrphanFileType {
    Preferences,
    ApplicationSupport,
    Cache,
    SavedState,
    Container,
    Other,
}

// Phase 4: Large Files
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LargeFileScanResult {
    pub total_size: u64,
    pub files: Vec<LargeFile>,
    pub scanned_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LargeFile {
    pub path: PathBuf,
    pub name: String,
    pub size: u64,
    pub modified: i64,
    pub accessed: Option<i64>,
    pub media_type: MediaType,
    pub thumbnail_path: Option<PathBuf>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MediaType {
    Video,
    Image,
    Archive,
    Document,
    Application,
    Other,
}

// Phase 5: Duplicates
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DuplicateScanResult {
    pub total_wasted_space: u64,
    pub groups: Vec<DuplicateGroup>,
    pub scanned_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DuplicateGroup {
    pub hash: String,
    pub size: u64,
    pub wasted_space: u64,
    pub files: Vec<DuplicateFile>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DuplicateFile {
    pub path: PathBuf,
    pub modified: i64,
    pub is_original: bool,
    pub is_protected: bool,
    pub is_selected: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cache_category_serialization() {
        let category = CacheCategory {
            name: "Test Browser".to_string(),
            category_type: CacheCategoryType::Browser,
            total_size: 500,
            items: vec![],
            is_protected: false,
            protection_reason: None,
        };

        let json = serde_json::to_string(&category).unwrap();
        let deserialized: CacheCategory = serde_json::from_str(&json).unwrap();

        assert_eq!(category.name, deserialized.name);
        assert_eq!(category.category_type, deserialized.category_type);
    }

    #[test]
    fn test_large_file_serialization() {
        let file = LargeFile {
            path: PathBuf::from("movie.mp4"),
            name: "movie.mp4".to_string(),
            size: 1024 * 1024 * 500,
            modified: 123456789,
            accessed: None,
            media_type: MediaType::Video,
            thumbnail_path: None,
        };

        let json = serde_json::to_string(&file).unwrap();
        let deserialized: LargeFile = serde_json::from_str(&json).unwrap();
        assert_eq!(file.media_type, deserialized.media_type);
    }
}
