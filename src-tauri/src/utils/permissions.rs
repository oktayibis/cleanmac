use dirs;
use std::fs;

pub fn check_full_disk_access() -> bool {
    #[cfg(target_os = "macos")]
    {
        // ~/Library/Safari is consistently protected by FDA on recent macOS versions
        if let Some(home) = dirs::home_dir() {
            let protected_path = home.join("Library/Safari");
            // We just need to check if we can list the directory contents
            return fs::read_dir(protected_path).is_ok();
        }
        false
    }
    #[cfg(not(target_os = "macos"))]
    {
        // For development on non-macOS or testing
        true
    }
}
