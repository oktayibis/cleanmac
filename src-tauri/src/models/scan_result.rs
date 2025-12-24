use super::file_entry::FileType;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum SafetyLevel {
    Safe,
    Warning,
    Protected,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CacheItem {
    pub id: String,
    pub path: PathBuf,
    pub name: String,
    pub size: u64,
    pub file_type: FileType,
    pub modified: Option<DateTime<Utc>>,
    pub safety_level: SafetyLevel,
    pub description: Option<String>,
    pub selected: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CacheCategory {
    pub id: String,
    pub name: String,
    pub description: String,
    pub items: Vec<CacheItem>,
    pub total_size: u64,
    pub selected: bool,
    pub icon: String, // Icon name or identifier
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CacheScanResult {
    pub categories: Vec<CacheCategory>,
    pub total_wasted_size: u64,
    pub scan_duration_ms: u64,
    pub scanned_at: DateTime<Utc>,
}
