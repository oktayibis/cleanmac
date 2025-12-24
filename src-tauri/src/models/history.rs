use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CleaningHistory {
    pub entries: Vec<CleaningEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CleaningEntry {
    pub timestamp: i64,
    pub space_reclaimed: u64,
    pub items_cleaned: u32,
    pub categories: Vec<String>,
    pub items: Vec<CleanedItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CleanedItem {
    pub path: PathBuf,
    pub size: u64,
    pub category: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_history_serialization() {
        let entry = CleaningEntry {
            timestamp: 1625247600,
            space_reclaimed: 1024,
            items_cleaned: 1,
            categories: vec!["Cache".to_string()],
            items: vec![CleanedItem {
                path: PathBuf::from("/tmp/file"),
                size: 1024,
                category: "Cache".to_string(),
            }],
        };

        let history = CleaningHistory {
            entries: vec![entry],
        };

        let json = serde_json::to_string(&history).unwrap();
        let deserialized: CleaningHistory = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.entries.len(), 1);
        assert_eq!(deserialized.entries[0].space_reclaimed, 1024);
    }
}
