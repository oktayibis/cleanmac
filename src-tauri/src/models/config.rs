use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub user_profile: UserProfile,
    pub exclusions: Vec<PathBuf>,
    pub large_file_threshold_mb: u64,
    pub auto_clean: AutoCleanConfig,
    pub appearance: AppearanceConfig,
    pub scan_locations: ScanLocations,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum UserProfile {
    Regular,
    Developer,
    Custom(CustomProfile),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CustomProfile {
    pub protect_developer_caches: bool,
    pub protected_paths: Vec<PathBuf>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutoCleanConfig {
    pub enabled: bool,
    pub schedule: AutoCleanSchedule,
    // We use String here to avoid circular deps, but typically it maps to CacheCategoryType
    pub categories: Vec<String>, 
    pub min_age_days: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AutoCleanSchedule {
    OnDemand,
    Daily,
    Weekly,
    Monthly,
    OnLowDiskSpace { threshold_gb: u32 },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppearanceConfig {
    pub theme: Theme,
    pub show_menu_bar_icon: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Theme {
    System,
    Light,
    Dark,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanLocations {
    pub include_external_volumes: bool,
    pub custom_scan_paths: Vec<PathBuf>,
}

// Developer Environment (for profile detection)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeveloperEnvironment {
    pub is_developer: bool,
    pub detected_tools: Vec<DeveloperTool>,
    pub confidence: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeveloperTool {
    pub name: String,
    pub tool_type: DeveloperToolType,
    pub cache_paths: Vec<PathBuf>,
    pub cache_size: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DeveloperToolType {
    Xcode,
    Homebrew,
    NodeNpm,
    Python,
    Rust,
    Ruby,
    Java,
    Docker,
    IDE,
    Git,
    Other,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_serialization() {
        let config = AppConfig {
            user_profile: UserProfile::Developer,
            exclusions: vec![PathBuf::from("/ignore/me")],
            large_file_threshold_mb: 100,
            auto_clean: AutoCleanConfig {
                enabled: false,
                schedule: AutoCleanSchedule::Weekly,
                categories: vec![],
                min_age_days: 14,
            },
            appearance: AppearanceConfig {
                theme: Theme::Dark,
                show_menu_bar_icon: true,
            },
            scan_locations: ScanLocations {
                include_external_volumes: false,
                custom_scan_paths: vec![],
            }
        };

        let json = serde_json::to_string(&config).unwrap();
        let deserialized: AppConfig = serde_json::from_str(&json).unwrap();
        
        assert!(matches!(deserialized.user_profile, UserProfile::Developer));
        assert_eq!(deserialized.appearance.theme, Theme::Dark);
    }
}
