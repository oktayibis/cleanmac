#[cfg(target_os = "macos")]
use std::process::Command;

/// Check if the application has Full Disk Access (FDA)
///
/// On macOS, we can check this by trying to read a directory that requires FDA,
/// such as /Library/Application Support/com.apple.TCC
pub fn check_full_disk_access() -> bool {
    #[cfg(target_os = "macos")]
    {
        // Method 1: Try to read TCC database directory
        // This is the most reliable way to check for FDA
        let tcc_path = "/Library/Application Support/com.apple.TCC";
        if std::fs::read_dir(tcc_path).is_ok() {
            return true;
        }

        // Method 2: Try to read user's Safari history
        // This typically requires FDA or specific Safari permissions
        if let Some(home) = dirs::home_dir() {
            let safari_path = home.join("Library/Safari/CloudTabs.db");
            if safari_path.exists() && std::fs::File::open(safari_path).is_ok() {
                return true;
            }
        }

        false
    }

    #[cfg(not(target_os = "macos"))]
    {
        // Non-macOS systems don't have FDA concepts in the same way
        true
    }
}

/// Open the System Settings to the Full Disk Access page
pub fn open_full_disk_access_settings() {
    #[cfg(target_os = "macos")]
    {
        let _ = Command::new("open")
            .arg("x-apple.systempreferences:com.apple.preference.security?Privacy_AllFiles")
            .output();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_full_disk_access_runs() {
        // We can't easily assert true/false as it depends on the test environment
        // But we can ensure it doesn't panic
        let _ = check_full_disk_access();
    }
}
