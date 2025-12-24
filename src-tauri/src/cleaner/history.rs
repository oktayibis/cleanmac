use crate::models::history::{CleaningEntry, CleaningHistory};
use std::fs;
use std::io::Write;
use std::path::Path;

pub struct HistoryManager {
    file_path: std::path::PathBuf,
}

impl HistoryManager {
    pub fn new(app_data_dir: &Path) -> Self {
        let file_path = app_data_dir.join("history.json");
        Self { file_path }
    }

    pub fn load(&self) -> CleaningHistory {
        if self.file_path.exists() {
            if let Ok(content) = fs::read_to_string(&self.file_path) {
                if let Ok(history) = serde_json::from_str(&content) {
                    return history;
                }
            }
        }
        CleaningHistory::default()
    }

    pub fn save(&self, history: &CleaningHistory) -> Result<(), String> {
        let content = serde_json::to_string_pretty(history).map_err(|e| e.to_string())?;
        if let Some(parent) = self.file_path.parent() {
            fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }
        let mut file = fs::File::create(&self.file_path).map_err(|e| e.to_string())?;
        file.write_all(content.as_bytes())
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn add_entry(&self, entry: CleaningEntry) -> Result<(), String> {
        let mut history = self.load();
        history.total_lifetime_reclaimed += entry.total_size_reclaimed;
        history.entries.push(entry);
        self.save(&history)
    }
}
