use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum Theme {
    System,
    Light,
    Dark,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum UserProfile {
    Standard,
    Developer,
    PowerUser,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AutoCleanConfig {
    pub enabled: bool,
    pub frequency_days: u32,
    pub next_run: Option<chrono::DateTime<chrono::Utc>>,
    pub notify_on_completion: bool,
    pub trusted_categories: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppConfig {
    pub theme: Theme,
    pub user_profile: UserProfile,
    pub auto_clean: AutoCleanConfig,
    pub excluded_paths: Vec<PathBuf>,
    pub scan_threshold_mb: u64, // For large files
    pub last_scan: Option<chrono::DateTime<chrono::Utc>>,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            theme: Theme::System,
            user_profile: UserProfile::Standard,
            auto_clean: AutoCleanConfig {
                enabled: false,
                frequency_days: 7,
                next_run: None,
                notify_on_completion: true,
                trusted_categories: vec![],
            },
            excluded_paths: vec![],
            scan_threshold_mb: 100,
            last_scan: None,
        }
    }
}
