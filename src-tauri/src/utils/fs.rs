use dirs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

pub fn get_dir_size(path: &Path) -> u64 {
    WalkDir::new(path)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter_map(|entry| entry.metadata().ok())
        .filter(|metadata| metadata.is_file())
        .map(|metadata| metadata.len())
        .sum()
}

pub fn expand_path(path_str: &str) -> PathBuf {
    if let Some(stripped) = path_str.strip_prefix("~") {
        if let Some(home_dir) = dirs::home_dir() {
            if stripped.is_empty() {
                return home_dir;
            }
            // Handle ~/.config, ~/Documents etc
            // stripped is either "" or "/Documents" etc
            let path_without_slash = stripped.strip_prefix('/').unwrap_or(stripped);
            return home_dir.join(path_without_slash);
        }
    }
    PathBuf::from(path_str)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn test_get_dir_size() {
        let dir = tempdir().unwrap();
        let file1 = dir.path().join("file1.txt");
        let file2 = dir.path().join("file2.txt");

        let mut f1 = File::create(&file1).unwrap();
        f1.write_all(b"Hello").unwrap(); // 5 bytes

        let mut f2 = File::create(&file2).unwrap();
        f2.write_all(b"World").unwrap(); // 5 bytes

        let size = get_dir_size(dir.path());
        assert_eq!(size, 10);
    }

    #[test]
    fn test_expand_path() {
        let home = dirs::home_dir().expect("Should have home dir");
        let expanded = expand_path("~/Documents");
        assert_eq!(expanded, home.join("Documents"));
    }
}
