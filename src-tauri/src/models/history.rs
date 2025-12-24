use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CleanedItem {
    pub path: PathBuf,
    pub size: u64,
    pub category_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CleaningEntry {
    pub id: String,
    pub timestamp: DateTime<Utc>,
    pub total_size_reclaimed: u64,
    pub items_count: usize,
    pub items: Vec<CleanedItem>,
    pub duration_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CleaningHistory {
    pub entries: Vec<CleaningEntry>,
    pub total_lifetime_reclaimed: u64,
}
