use crate::models::file_entry::FileType;
use crate::models::scan_result::{CacheCategory, CacheItem, CacheScanResult, SafetyLevel};
use crate::utils::fs::{expand_path, get_dir_size};
use chrono::Utc;
use serde::Serialize;
use std::collections::HashMap;
use std::path::PathBuf;
use std::time::Instant;
use tauri::{Emitter, Runtime, Window};

const CACHE_LOCATIONS: &[&str] = &["~/Library/Caches", "~/Library/Logs"];

#[derive(Clone, Serialize)]
struct ScanProgress {
    status: String,
    current_path: Option<String>,
    progress: u8, // 0-100 (approximate)
    scanned_bytes: u64,
}

pub struct CacheScanner;

impl Default for CacheScanner {
    fn default() -> Self {
        Self::new()
    }
}

impl CacheScanner {
    pub fn new() -> Self {
        Self
    }

    pub fn scan<R: Runtime>(&self, window: &Window<R>) -> CacheScanResult {
        let start_time = Instant::now();
        let mut items_by_category: HashMap<String, Vec<CacheItem>> = HashMap::new();
        let mut total_scanned_bytes = 0;

        let locations: Vec<PathBuf> = CACHE_LOCATIONS
            .iter()
            .map(|p| expand_path(p))
            .filter(|p| p.exists())
            .collect();

        let total_locations = locations.len();

        for (loc_idx, location) in locations.iter().enumerate() {
            // Emit progress for starting a location
            let _ = window.emit(
                "scan-progress",
                ScanProgress {
                    status: "scanning".to_string(),
                    current_path: Some(location.to_string_lossy().to_string()),
                    progress: ((loc_idx as f64 / total_locations as f64) * 100.0) as u8,
                    scanned_bytes: total_scanned_bytes,
                },
            );

            // Read directory items
            if let Ok(entries) = std::fs::read_dir(location) {
                for entry in entries.filter_map(|e| e.ok()) {
                    let path = entry.path();
                    if !path.is_dir() {
                        continue;
                    }

                    let name = path
                        .file_name()
                        .map(|n| n.to_string_lossy().to_string())
                        .unwrap_or_else(|| "Unknown".to_string());

                    // Calculate size
                    // Update progress with current checking path
                    let _ = window.emit(
                        "scan-progress",
                        ScanProgress {
                            status: "scanning".to_string(),
                            current_path: Some(name.clone()),
                            progress: ((loc_idx as f64 / total_locations as f64) * 100.0) as u8,
                            scanned_bytes: total_scanned_bytes,
                        },
                    );

                    let size = get_dir_size(&path);
                    total_scanned_bytes += size;

                    let (category_id, safety_level) = self.categorize(&name);

                    let item = CacheItem {
                        id: path.to_string_lossy().to_string(),
                        path: path.clone(),
                        name: name.clone(),
                        size,
                        file_type: FileType::Directory,
                        modified: entry
                            .metadata()
                            .ok()
                            .and_then(|m| m.modified().ok())
                            .map(|t| t.into()),
                        safety_level,
                        description: None,
                        selected: true, // Default to selected
                    };

                    items_by_category.entry(category_id).or_default().push(item);
                }
            }
        }

        // Finalize results
        let mut categories = Vec::new();
        let mut total_wasted_size = 0;

        for (cat_id, items) in items_by_category {
            let cat_total_size: u64 = items.iter().map(|i| i.size).sum();
            total_wasted_size += cat_total_size;

            let (cat_name, cat_desc, cat_icon) = self.get_category_info(&cat_id);

            categories.push(CacheCategory {
                id: cat_id,
                name: cat_name,
                description: cat_desc,
                items,
                total_size: cat_total_size,
                selected: true,
                icon: cat_icon,
            });
        }

        let _ = window.emit(
            "scan-progress",
            ScanProgress {
                status: "completed".to_string(),
                current_path: None,
                progress: 100,
                scanned_bytes: total_scanned_bytes,
            },
        );

        CacheScanResult {
            categories,
            total_wasted_size,
            scan_duration_ms: start_time.elapsed().as_millis() as u64,
            scanned_at: Utc::now(),
        }
    }

    fn categorize(&self, name: &str) -> (String, SafetyLevel) {
        let name_lower = name.to_lowercase();

        if name_lower.contains("google") && name_lower.contains("chrome") {
            return ("browser".to_string(), SafetyLevel::Safe);
        }
        if name_lower.contains("safari") {
            return ("browser".to_string(), SafetyLevel::Safe); // Usually safe to clear caches
        }
        if name_lower.contains("firefox") || name_lower.contains("mozilla") {
            return ("browser".to_string(), SafetyLevel::Safe);
        }

        if name_lower.contains("code")
            || name_lower.contains("jetbrains")
            || name_lower.contains("xcode")
        {
            return ("development".to_string(), SafetyLevel::Warning); // Devs might want to keep these
        }

        if name.starts_with("com.apple") {
            return ("system".to_string(), SafetyLevel::Warning);
        }

        ("application".to_string(), SafetyLevel::Safe)
    }

    fn get_category_info(&self, id: &str) -> (String, String, String) {
        match id {
            "browser" => (
                "Browser Cache".to_string(),
                "Temporary files from web browsers".to_string(),
                "globe".to_string(),
            ),
            "development" => (
                "Developer".to_string(),
                "Caches from IDEs and tools".to_string(),
                "code".to_string(),
            ),
            "system" => (
                "System".to_string(),
                "macOS system caches".to_string(),
                "cpu".to_string(),
            ),
            _ => (
                "Application".to_string(),
                "General application caches".to_string(),
                "app".to_string(),
            ),
        }
    }
}
