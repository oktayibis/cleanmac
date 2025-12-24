# Technical Specification: CleanMac

## Technical Context

### Technology Stack

| Component | Technology | Version | Rationale |
|-----------|------------|---------|-----------|
| Framework | Tauri | 2.x | Native performance, small binary, Rust backend |
| Backend | Rust | 1.75+ | Memory safety, performance, system APIs |
| Frontend | React + TypeScript | React 18, TS 5.x | Modern UI, type safety, rich ecosystem |
| Styling | Tailwind CSS | 3.x | Rapid UI development, consistent design |
| Build Tool | Vite | 5.x | Fast HMR, optimized builds |
| State Management | Zustand | 4.x | Lightweight, TypeScript-friendly |
| Package Manager | pnpm | 8.x | Fast, disk-efficient |

### System Requirements

- **macOS**: 12 Monterey and newer (ARM64 and x86_64)
- **Permissions**: Full Disk Access required for complete scanning
- **Disk Space**: ~50MB for application
- **Memory**: 100MB base, scales with scan size

### Dependencies (Rust)

```toml
[dependencies]
tauri = { version = "2", features = ["macos-private-api"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
tokio = { version = "1", features = ["full"] }
walkdir = "2"              # Directory traversal
sha2 = "0.10"              # Hashing for duplicates
rayon = "1"                # Parallel processing
plist = "1"                # Read .plist files
trash = "4"                # Move to Trash safely
dirs = "5"                 # Standard directories
chrono = "0.4"             # Date/time handling
notify = "6"               # File system events
log = "0.4"
env_logger = "0.10"
thiserror = "1"            # Error handling
glob = "0.3"               # Pattern matching
```

---

## Architecture Overview

### High-Level Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                      CleanMac App                           │
├─────────────────────────────────────────────────────────────┤
│  Frontend (React + TypeScript)                              │
│  ┌──────────┐ ┌──────────┐ ┌──────────┐ ┌──────────┐       │
│  │Dashboard │ │ Cache    │ │ Large    │ │Duplicate │       │
│  │   View   │ │  View    │ │  Files   │ │  Finder  │       │
│  └────┬─────┘ └────┬─────┘ └────┬─────┘ └────┬─────┘       │
│       │            │            │            │              │
│  ┌────┴────────────┴────────────┴────────────┴─────┐       │
│  │              State Management (Zustand)          │       │
│  └────────────────────────┬────────────────────────┘       │
│                           │ Tauri Commands (IPC)            │
├───────────────────────────┼─────────────────────────────────┤
│  Backend (Rust)           ▼                                 │
│  ┌─────────────────────────────────────────────────┐       │
│  │              Command Handlers (Tauri)            │       │
│  └───────────────────────┬─────────────────────────┘       │
│           ┌──────────────┼──────────────┐                  │
│           ▼              ▼              ▼                  │
│  ┌────────────┐ ┌────────────┐ ┌────────────┐              │
│  │  Scanner   │ │  Analyzer  │ │  Cleaner   │              │
│  │   Module   │ │   Module   │ │   Module   │              │
│  └────────────┘ └────────────┘ └────────────┘              │
│           │              │              │                  │
│           ▼              ▼              ▼                  │
│  ┌─────────────────────────────────────────────────┐       │
│  │              Core Services                       │       │
│  │  • FileSystem  • DeveloperDetection  • Config   │       │
│  │  • Hashing     • AppRegistry         • History  │       │
│  └─────────────────────────────────────────────────┘       │
└─────────────────────────────────────────────────────────────┘
```

### Directory Structure

```
cleanmac/
├── src-tauri/                    # Rust backend
│   ├── Cargo.toml
│   ├── tauri.conf.json
│   ├── capabilities/             # Tauri v2 permissions
│   ├── icons/
│   └── src/
│       ├── main.rs               # Entry point
│       ├── lib.rs                # Library exports
│       ├── commands/             # Tauri command handlers
│       │   ├── mod.rs
│       │   ├── scan.rs           # Scan commands
│       │   ├── clean.rs          # Cleaning commands
│       │   ├── config.rs         # Settings commands
│       │   └── system.rs         # System info commands
│       ├── scanner/              # File scanning logic
│       │   ├── mod.rs
│       │   ├── cache_scanner.rs
│       │   ├── large_file_scanner.rs
│       │   ├── duplicate_scanner.rs
│       │   └── orphan_scanner.rs
│       ├── analyzer/             # Analysis logic
│       │   ├── mod.rs
│       │   ├── developer_detector.rs
│       │   ├── app_registry.rs
│       │   └── cache_categorizer.rs
│       ├── cleaner/              # Cleaning operations
│       │   ├── mod.rs
│       │   ├── safe_delete.rs
│       │   └── history.rs
│       ├── models/               # Data structures
│       │   ├── mod.rs
│       │   ├── scan_result.rs
│       │   ├── file_entry.rs
│       │   ├── config.rs
│       │   └── user_profile.rs
│       └── utils/                # Utilities
│           ├── mod.rs
│           ├── fs.rs
│           ├── hash.rs
│           └── permissions.rs
├── src/                          # Frontend (React)
│   ├── main.tsx                  # Entry point
│   ├── App.tsx                   # Root component
│   ├── components/               # UI components
│   │   ├── layout/
│   │   │   ├── Sidebar.tsx
│   │   │   ├── Header.tsx
│   │   │   └── StatusBar.tsx
│   │   ├── dashboard/
│   │   │   ├── DiskUsageChart.tsx
│   │   │   ├── QuickStats.tsx
│   │   │   └── QuickActions.tsx
│   │   ├── cache/
│   │   │   ├── CacheList.tsx
│   │   │   ├── CacheCategory.tsx
│   │   │   └── DeveloperCacheSection.tsx
│   │   ├── leftovers/
│   │   │   ├── OrphanedAppList.tsx
│   │   │   └── LeftoverItem.tsx
│   │   ├── large-files/
│   │   │   ├── LargeFileList.tsx
│   │   │   ├── FilePreview.tsx
│   │   │   └── FilterControls.tsx
│   │   ├── duplicates/
│   │   │   ├── DuplicateGroups.tsx
│   │   │   └── DuplicateItem.tsx
│   │   ├── settings/
│   │   │   ├── SettingsPage.tsx
│   │   │   ├── ProfileSelector.tsx
│   │   │   └── ExclusionList.tsx
│   │   └── common/
│   │       ├── Button.tsx
│   │       ├── ProgressBar.tsx
│   │       ├── Modal.tsx
│   │       ├── FileIcon.tsx
│   │       └── SizeDisplay.tsx
│   ├── stores/                   # Zustand stores
│   │   ├── scanStore.ts
│   │   ├── configStore.ts
│   │   └── uiStore.ts
│   ├── hooks/                    # Custom hooks
│   │   ├── useTauriCommand.ts
│   │   ├── useScan.ts
│   │   └── useQuickLook.ts
│   ├── lib/                      # Utilities
│   │   ├── tauri.ts              # Tauri API wrappers
│   │   ├── format.ts             # Size/date formatting
│   │   └── constants.ts
│   ├── types/                    # TypeScript types
│   │   └── index.ts
│   └── styles/
│       └── globals.css
├── index.html
├── package.json
├── pnpm-lock.yaml
├── tsconfig.json
├── vite.config.ts
├── tailwind.config.js
├── postcss.config.js
└── .gitignore
```

---

## Data Models

### Rust Models (`src-tauri/src/models/`)

```rust
// file_entry.rs
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileEntry {
    pub path: PathBuf,
    pub size: u64,
    pub modified: i64,          // Unix timestamp
    pub accessed: Option<i64>,  // Last access time
    pub file_type: FileType,
    pub category: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum FileType {
    File,
    Directory,
    Symlink,
}

// scan_result.rs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheScanResult {
    pub total_size: u64,
    pub categories: Vec<CacheCategory>,
    pub scanned_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheCategory {
    pub name: String,
    pub category_type: CacheCategoryType,
    pub total_size: u64,
    pub items: Vec<CacheItem>,
    pub is_protected: bool,     // For developer mode
    pub protection_reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CacheCategoryType {
    Browser,
    System,
    Application,
    Developer,
    Temporary,
    Logs,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheItem {
    pub path: PathBuf,
    pub name: String,
    pub size: u64,
    pub age_days: Option<u32>,
    pub app_name: Option<String>,
    pub bundle_id: Option<String>,
    pub safe_to_delete: SafetyLevel,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SafetyLevel {
    Safe,           // Can delete without concern
    Caution,        // May need re-download
    Protected,      // Should not delete (dev caches in dev mode)
    Unknown,        // Unable to determine
}

// orphan.rs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrphanScanResult {
    pub total_size: u64,
    pub orphaned_apps: Vec<OrphanedApp>,
    pub scanned_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrphanedApp {
    pub presumed_name: String,
    pub bundle_id: Option<String>,
    pub total_size: u64,
    pub files: Vec<OrphanedFile>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrphanedFile {
    pub path: PathBuf,
    pub size: u64,
    pub file_type: OrphanFileType,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum OrphanFileType {
    Preferences,        // .plist files
    ApplicationSupport,
    Cache,
    SavedState,
    Container,
    Other,
}

// large_file.rs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LargeFileScanResult {
    pub total_size: u64,
    pub files: Vec<LargeFile>,
    pub scanned_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LargeFile {
    pub path: PathBuf,
    pub name: String,
    pub size: u64,
    pub modified: i64,
    pub accessed: Option<i64>,
    pub media_type: MediaType,
    pub thumbnail_path: Option<PathBuf>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MediaType {
    Video,
    Image,
    Archive,
    Document,
    Application,
    Other,
}

// duplicate.rs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DuplicateScanResult {
    pub total_wasted_space: u64,
    pub groups: Vec<DuplicateGroup>,
    pub scanned_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DuplicateGroup {
    pub hash: String,
    pub size: u64,
    pub wasted_space: u64,      // size * (count - 1)
    pub files: Vec<DuplicateFile>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DuplicateFile {
    pub path: PathBuf,
    pub modified: i64,
    pub is_original: bool,      // Oldest file
    pub is_protected: bool,     // In protected location
    pub is_selected: bool,      // Selected for deletion
}

// config.rs
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomProfile {
    pub protect_developer_caches: bool,
    pub protected_paths: Vec<PathBuf>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutoCleanConfig {
    pub enabled: bool,
    pub schedule: AutoCleanSchedule,
    pub categories: Vec<CacheCategoryType>,
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

// user_profile.rs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeveloperEnvironment {
    pub is_developer: bool,
    pub detected_tools: Vec<DeveloperTool>,
    pub confidence: f32,        // 0.0 - 1.0
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

// history.rs
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
```

### TypeScript Types (`src/types/index.ts`)

```typescript
// Mirror of Rust types for frontend
export interface FileEntry {
  path: string;
  size: number;
  modified: number;
  accessed: number | null;
  fileType: 'File' | 'Directory' | 'Symlink';
  category?: string;
}

export interface CacheScanResult {
  totalSize: number;
  categories: CacheCategory[];
  scannedAt: number;
}

export interface CacheCategory {
  name: string;
  categoryType: CacheCategoryType;
  totalSize: number;
  items: CacheItem[];
  isProtected: boolean;
  protectionReason?: string;
}

export type CacheCategoryType =
  | 'Browser'
  | 'System'
  | 'Application'
  | 'Developer'
  | 'Temporary'
  | 'Logs';

export interface CacheItem {
  path: string;
  name: string;
  size: number;
  ageDays?: number;
  appName?: string;
  bundleId?: string;
  safeToDelete: SafetyLevel;
  description?: string;
}

export type SafetyLevel = 'Safe' | 'Caution' | 'Protected' | 'Unknown';

export interface OrphanScanResult {
  totalSize: number;
  orphanedApps: OrphanedApp[];
  scannedAt: number;
}

export interface OrphanedApp {
  presumedName: string;
  bundleId?: string;
  totalSize: number;
  files: OrphanedFile[];
}

export interface OrphanedFile {
  path: string;
  size: number;
  fileType: OrphanFileType;
}

export type OrphanFileType =
  | 'Preferences'
  | 'ApplicationSupport'
  | 'Cache'
  | 'SavedState'
  | 'Container'
  | 'Other';

export interface LargeFileScanResult {
  totalSize: number;
  files: LargeFile[];
  scannedAt: number;
}

export interface LargeFile {
  path: string;
  name: string;
  size: number;
  modified: number;
  accessed?: number;
  mediaType: MediaType;
  thumbnailPath?: string;
}

export type MediaType =
  | 'Video'
  | 'Image'
  | 'Archive'
  | 'Document'
  | 'Application'
  | 'Other';

export interface DuplicateScanResult {
  totalWastedSpace: number;
  groups: DuplicateGroup[];
  scannedAt: number;
}

export interface DuplicateGroup {
  hash: string;
  size: number;
  wastedSpace: number;
  files: DuplicateFile[];
}

export interface DuplicateFile {
  path: string;
  modified: number;
  isOriginal: boolean;
  isProtected: boolean;
  isSelected: boolean;
}

export interface AppConfig {
  userProfile: UserProfile;
  exclusions: string[];
  largeFileThresholdMb: number;
  autoClean: AutoCleanConfig;
  appearance: AppearanceConfig;
  scanLocations: ScanLocations;
}

export type UserProfile =
  | { type: 'Regular' }
  | { type: 'Developer' }
  | { type: 'Custom'; profile: CustomProfile };

export interface CustomProfile {
  protectDeveloperCaches: boolean;
  protectedPaths: string[];
}

export interface AutoCleanConfig {
  enabled: boolean;
  schedule: AutoCleanSchedule;
  categories: CacheCategoryType[];
  minAgeDays: number;
}

export type AutoCleanSchedule =
  | { type: 'OnDemand' }
  | { type: 'Daily' }
  | { type: 'Weekly' }
  | { type: 'Monthly' }
  | { type: 'OnLowDiskSpace'; thresholdGb: number };

export interface AppearanceConfig {
  theme: 'System' | 'Light' | 'Dark';
  showMenuBarIcon: boolean;
}

export interface ScanLocations {
  includeExternalVolumes: boolean;
  customScanPaths: string[];
}

export interface DeveloperEnvironment {
  isDeveloper: boolean;
  detectedTools: DeveloperTool[];
  confidence: number;
}

export interface DeveloperTool {
  name: string;
  toolType: DeveloperToolType;
  cachePaths: string[];
  cacheSize: number;
}

export type DeveloperToolType =
  | 'Xcode'
  | 'Homebrew'
  | 'NodeNpm'
  | 'Python'
  | 'Rust'
  | 'Ruby'
  | 'Java'
  | 'Docker'
  | 'IDE'
  | 'Git'
  | 'Other';

// Scan progress for real-time updates
export interface ScanProgress {
  phase: string;
  current: number;
  total: number;
  currentPath?: string;
  bytesScanned: number;
}

// Cleaning result
export interface CleaningResult {
  success: boolean;
  spaceReclaimed: number;
  itemsCleaned: number;
  errors: CleaningError[];
}

export interface CleaningError {
  path: string;
  error: string;
}

// System info
export interface DiskInfo {
  totalSpace: number;
  freeSpace: number;
  usedSpace: number;
  volumeName: string;
}
```

---

## Tauri Commands (IPC Interface)

### Command Definitions (`src-tauri/src/commands/`)

```rust
// scan.rs
#[tauri::command]
pub async fn scan_caches(
    config: AppConfig,
    window: tauri::Window,
) -> Result<CacheScanResult, String>;

#[tauri::command]
pub async fn scan_large_files(
    threshold_mb: u64,
    scan_paths: Vec<PathBuf>,
    window: tauri::Window,
) -> Result<LargeFileScanResult, String>;

#[tauri::command]
pub async fn scan_duplicates(
    scan_paths: Vec<PathBuf>,
    window: tauri::Window,
) -> Result<DuplicateScanResult, String>;

#[tauri::command]
pub async fn scan_orphaned_apps(
    window: tauri::Window,
) -> Result<OrphanScanResult, String>;

#[tauri::command]
pub async fn cancel_scan() -> Result<(), String>;

// clean.rs
#[tauri::command]
pub async fn clean_items(
    paths: Vec<PathBuf>,
    permanent: bool,
) -> Result<CleaningResult, String>;

#[tauri::command]
pub async fn clean_category(
    category: CacheCategoryType,
    permanent: bool,
) -> Result<CleaningResult, String>;

#[tauri::command]
pub fn get_cleaning_history() -> Result<CleaningHistory, String>;

// config.rs
#[tauri::command]
pub fn get_config() -> Result<AppConfig, String>;

#[tauri::command]
pub fn save_config(config: AppConfig) -> Result<(), String>;

#[tauri::command]
pub fn add_exclusion(path: PathBuf) -> Result<(), String>;

#[tauri::command]
pub fn remove_exclusion(path: PathBuf) -> Result<(), String>;

// system.rs
#[tauri::command]
pub fn get_disk_info() -> Result<DiskInfo, String>;

#[tauri::command]
pub async fn detect_developer_environment() -> Result<DeveloperEnvironment, String>;

#[tauri::command]
pub fn check_full_disk_access() -> Result<bool, String>;

#[tauri::command]
pub fn open_full_disk_access_settings() -> Result<(), String>;

#[tauri::command]
pub fn reveal_in_finder(path: PathBuf) -> Result<(), String>;

#[tauri::command]
pub fn quick_look_file(path: PathBuf) -> Result<(), String>;

#[tauri::command]
pub fn open_file(path: PathBuf) -> Result<(), String>;
```

### Frontend API Wrapper (`src/lib/tauri.ts`)

```typescript
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';

// Scan operations
export const scanCaches = (config: AppConfig) =>
  invoke<CacheScanResult>('scan_caches', { config });

export const scanLargeFiles = (thresholdMb: number, scanPaths: string[]) =>
  invoke<LargeFileScanResult>('scan_large_files', { thresholdMb, scanPaths });

export const scanDuplicates = (scanPaths: string[]) =>
  invoke<DuplicateScanResult>('scan_duplicates', { scanPaths });

export const scanOrphanedApps = () =>
  invoke<OrphanScanResult>('scan_orphaned_apps');

export const cancelScan = () =>
  invoke('cancel_scan');

// Clean operations
export const cleanItems = (paths: string[], permanent = false) =>
  invoke<CleaningResult>('clean_items', { paths, permanent });

export const cleanCategory = (category: CacheCategoryType, permanent = false) =>
  invoke<CleaningResult>('clean_category', { category, permanent });

export const getCleaningHistory = () =>
  invoke<CleaningHistory>('get_cleaning_history');

// Config operations
export const getConfig = () =>
  invoke<AppConfig>('get_config');

export const saveConfig = (config: AppConfig) =>
  invoke('save_config', { config });

export const addExclusion = (path: string) =>
  invoke('add_exclusion', { path });

export const removeExclusion = (path: string) =>
  invoke('remove_exclusion', { path });

// System operations
export const getDiskInfo = () =>
  invoke<DiskInfo>('get_disk_info');

export const detectDeveloperEnvironment = () =>
  invoke<DeveloperEnvironment>('detect_developer_environment');

export const checkFullDiskAccess = () =>
  invoke<boolean>('check_full_disk_access');

export const openFullDiskAccessSettings = () =>
  invoke('open_full_disk_access_settings');

export const revealInFinder = (path: string) =>
  invoke('reveal_in_finder', { path });

export const quickLookFile = (path: string) =>
  invoke('quick_look_file', { path });

export const openFile = (path: string) =>
  invoke('open_file', { path });

// Event listeners for progress
export const onScanProgress = (callback: (progress: ScanProgress) => void) =>
  listen<ScanProgress>('scan-progress', (event) => callback(event.payload));
```

---

## Core Implementation Approach

### 1. Scanner Module

**Cache Scanner (`src-tauri/src/scanner/cache_scanner.rs`)**

```rust
// Approach:
// 1. Walk predefined cache directories
// 2. Categorize each entry by path pattern and .plist metadata
// 3. Calculate size recursively for directories
// 4. Check last access time for age calculation
// 5. Mark developer caches as protected based on user profile
// 6. Emit progress events via Tauri window

const CACHE_LOCATIONS: &[(&str, CacheCategoryType)] = &[
    ("~/Library/Caches", CacheCategoryType::Application),
    ("~/Library/Logs", CacheCategoryType::Logs),
    ("/Library/Caches", CacheCategoryType::System),
    ("/tmp", CacheCategoryType::Temporary),
    // Browser-specific paths detected dynamically
];

const DEVELOPER_CACHE_PATTERNS: &[(&str, DeveloperToolType)] = &[
    ("~/Library/Developer/Xcode/DerivedData", DeveloperToolType::Xcode),
    ("~/.npm", DeveloperToolType::NodeNpm),
    ("~/.cargo", DeveloperToolType::Rust),
    ("~/.gradle", DeveloperToolType::Java),
    // ... etc
];
```

**Orphan Scanner (`src-tauri/src/scanner/orphan_scanner.rs`)**

```rust
// Approach:
// 1. Build registry of installed apps by scanning /Applications
// 2. Extract bundle identifiers from Info.plist
// 3. Scan Library directories for app-related files
// 4. Match files against known bundle IDs
// 5. Files with unmatched bundle IDs = orphaned
// 6. Group by presumed app name

const ORPHAN_SCAN_LOCATIONS: &[&str] = &[
    "~/Library/Application Support",
    "~/Library/Preferences",
    "~/Library/Caches",
    "~/Library/Saved Application State",
    "~/Library/Containers",
    "~/Library/Group Containers",
    "~/Library/HTTPStorages",
    "~/Library/WebKit",
];
```

**Large File Scanner (`src-tauri/src/scanner/large_file_scanner.rs`)**

```rust
// Approach:
// 1. Walk user home directory (respecting exclusions)
// 2. Filter files by size threshold
// 3. Categorize by extension/MIME type
// 4. Sort by size descending
// 5. Generate thumbnails for images (lazy, via macOS APIs)
// 6. Parallel directory traversal using rayon

const MEDIA_EXTENSIONS: &[(&str, MediaType)] = &[
    // Video
    ("mp4", MediaType::Video),
    ("mov", MediaType::Video),
    ("avi", MediaType::Video),
    // Images
    ("jpg", MediaType::Image),
    ("png", MediaType::Image),
    ("heic", MediaType::Image),
    // ... etc
];
```

**Duplicate Scanner (`src-tauri/src/scanner/duplicate_scanner.rs`)**

```rust
// Approach:
// 1. First pass: Group files by size
// 2. Second pass: For same-size files, compute partial hash (first/last 4KB)
// 3. Third pass: For matching partial hashes, compute full SHA-256
// 4. Group files with identical full hashes
// 5. Mark oldest file as "original"
// 6. Parallel hashing using rayon for performance

use rayon::prelude::*;
use sha2::{Sha256, Digest};

fn partial_hash(path: &Path) -> Result<[u8; 32]> {
    // Read first 4KB + last 4KB
    // Hash together for quick comparison
}

fn full_hash(path: &Path) -> Result<String> {
    // Stream file through SHA-256
    // Return hex string
}
```

### 2. Analyzer Module

**Developer Detector (`src-tauri/src/analyzer/developer_detector.rs`)**

```rust
// Check for developer tool presence
const DEVELOPER_INDICATORS: &[(&str, DeveloperToolType, f32)] = &[
    ("/Applications/Xcode.app", DeveloperToolType::Xcode, 0.9),
    ("/opt/homebrew", DeveloperToolType::Homebrew, 0.8),
    ("~/.cargo", DeveloperToolType::Rust, 0.7),
    ("~/.npm", DeveloperToolType::NodeNpm, 0.7),
    ("~/.pyenv", DeveloperToolType::Python, 0.7),
    // VS Code, JetBrains IDEs, etc.
];

pub fn detect_developer_environment() -> DeveloperEnvironment {
    // Score each indicator
    // Confidence = sum of matched scores / max possible score
    // is_developer = confidence > 0.3 (at least a few dev tools)
}
```

**App Registry (`src-tauri/src/analyzer/app_registry.rs`)**

```rust
// Build mapping of bundle IDs to app names
pub struct AppRegistry {
    apps: HashMap<String, InstalledApp>,
}

impl AppRegistry {
    pub fn scan_applications() -> Self {
        // Scan /Applications and ~/Applications
        // Read Info.plist for each .app bundle
        // Extract CFBundleIdentifier, CFBundleName
    }

    pub fn is_installed(&self, bundle_id: &str) -> bool;
    pub fn get_app_name(&self, bundle_id: &str) -> Option<&str>;
}
```

### 3. Cleaner Module

**Safe Delete (`src-tauri/src/cleaner/safe_delete.rs`)**

```rust
use trash;

pub async fn delete_items(
    paths: Vec<PathBuf>,
    permanent: bool,
    history: &mut CleaningHistory,
) -> CleaningResult {
    let mut space_reclaimed = 0;
    let mut items_cleaned = 0;
    let mut errors = vec![];

    for path in paths {
        let size = get_size(&path)?;

        let result = if permanent {
            fs::remove_dir_all(&path).or_else(|_| fs::remove_file(&path))
        } else {
            trash::delete(&path)  // Move to Trash
        };

        match result {
            Ok(_) => {
                space_reclaimed += size;
                items_cleaned += 1;
                history.add_entry(&path, size);
            }
            Err(e) => {
                errors.push(CleaningError {
                    path: path.to_string_lossy().to_string(),
                    error: e.to_string(),
                });
            }
        }
    }

    CleaningResult {
        success: errors.is_empty(),
        space_reclaimed,
        items_cleaned,
        errors,
    }
}
```

### 4. Frontend State Management

**Scan Store (`src/stores/scanStore.ts`)**

```typescript
import { create } from 'zustand';

interface ScanState {
  // Scan results
  cacheResult: CacheScanResult | null;
  orphanResult: OrphanScanResult | null;
  largeFilesResult: LargeFileScanResult | null;
  duplicatesResult: DuplicateScanResult | null;

  // Scan progress
  isScanning: boolean;
  scanProgress: ScanProgress | null;

  // Selection
  selectedItems: Set<string>;

  // Actions
  startCacheScan: () => Promise<void>;
  startOrphanScan: () => Promise<void>;
  startLargeFileScan: (threshold: number) => Promise<void>;
  startDuplicateScan: (paths: string[]) => Promise<void>;
  cancelScan: () => void;
  toggleSelection: (path: string) => void;
  selectAll: (paths: string[]) => void;
  clearSelection: () => void;
  cleanSelected: (permanent?: boolean) => Promise<CleaningResult>;
}

export const useScanStore = create<ScanState>((set, get) => ({
  // Implementation
}));
```

---

## Delivery Phases

### Phase 1: Foundation (MVP)

**Goal**: Core scanning and cache cleanup functionality

**Deliverables**:
1. Tauri project setup with build configuration
2. Basic UI shell with sidebar navigation
3. Disk usage display (Dashboard)
4. System cache scanner
5. Cache list view with selection
6. Basic cleaning to Trash
7. Configuration persistence

**Verification**:
- App launches on macOS 12+
- Can scan ~/Library/Caches
- Displays cache items with sizes
- Can delete selected caches to Trash
- Settings persist across restarts

### Phase 2: Developer Mode

**Goal**: Developer-aware cache management

**Deliverables**:
1. Developer environment detection
2. User profile selection (Regular/Developer)
3. Developer cache protection
4. Developer-specific cache section in UI
5. Protected cache indicators

**Verification**:
- Correctly detects Xcode, npm, cargo installations
- Developer caches marked as protected in dev mode
- Can switch between profiles
- Protected caches not selected by default

### Phase 3: Application Leftovers

**Goal**: Orphaned application file detection

**Deliverables**:
1. Application registry scanner
2. Bundle ID extraction
3. Orphan detection logic
4. Orphaned apps view
5. Grouped leftover display

**Verification**:
- Lists currently installed applications
- Detects files from uninstalled apps
- Groups files by presumed app
- Can clean selected leftovers

### Phase 4: Large Files

**Goal**: Large file discovery and management

**Deliverables**:
1. Large file scanner with threshold
2. Media type categorization
3. Large files view with filters
4. File preview (Quick Look integration)
5. Reveal in Finder action

**Verification**:
- Finds files above threshold
- Correctly categorizes by type
- Preview works for images/videos
- Can filter by type
- Reveal in Finder works

### Phase 5: Duplicate Finder

**Goal**: Duplicate file detection

**Deliverables**:
1. Size-based grouping
2. Hash-based verification
3. Duplicate groups view
4. Original file marking
5. Smart selection

**Verification**:
- Finds actual duplicates (not false positives)
- Performance acceptable for 10K+ files
- Shows original vs copies
- Can select non-originals for deletion

### Phase 6: Polish & Distribution

**Goal**: Production-ready release

**Deliverables**:
1. Full Disk Access permission flow
2. Auto-clean scheduling
3. Menu bar mode
4. Cleaning history view
5. Code signing and notarization
6. DMG installer creation
7. Performance optimization
8. Accessibility audit

**Verification**:
- Permission request works correctly
- Auto-clean runs on schedule
- Menu bar icon functional
- History shows past cleanings
- App passes notarization
- VoiceOver navigable

---

## Verification Approach

### Unit Tests (Rust)

```bash
# Run Rust unit tests
cd src-tauri && cargo test
```

**Test coverage**:
- Scanner logic (mock file system)
- Hash computation correctness
- Bundle ID extraction
- Developer detection scoring
- Config serialization

### Integration Tests

```bash
# Run with test fixtures
cargo test --features integration-tests
```

**Test coverage**:
- Full scan workflow with test directory
- Orphan detection accuracy
- Duplicate detection accuracy

### Frontend Tests

```bash
# Run frontend tests
pnpm test
```

**Test coverage**:
- Component rendering
- Store actions
- Tauri command mocking

### Manual Testing Checklist

- [ ] Fresh install on macOS 12 (VM)
- [ ] Fresh install on macOS 14 (ARM)
- [ ] Full Disk Access permission flow
- [ ] Scan with >100GB of caches
- [ ] Delete operation to Trash
- [ ] Permanent delete with confirmation
- [ ] Developer mode protection
- [ ] Large file preview
- [ ] Duplicate detection accuracy
- [ ] App leftover detection
- [ ] Settings persistence
- [ ] Menu bar mode
- [ ] Dark mode appearance
- [ ] VoiceOver navigation

### Linting

```bash
# Rust
cargo clippy -- -D warnings
cargo fmt --check

# Frontend
pnpm lint
pnpm typecheck
```

---

## Security Considerations

1. **Sandboxing**: Tauri apps are sandboxed by default; we need to request specific entitlements
2. **Full Disk Access**: Required for complete scanning; app works with limited functionality without it
3. **No Telemetry**: All operations local; no network requests
4. **Trash First**: Default to recoverable deletion
5. **Code Signing**: Required for distribution; prevents tampering

### Required Entitlements

```xml
<!-- src-tauri/entitlements.plist -->
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>com.apple.security.app-sandbox</key>
    <false/>
    <key>com.apple.security.files.user-selected.read-write</key>
    <true/>
    <key>com.apple.security.files.downloads.read-write</key>
    <true/>
</dict>
</plist>
```

---

## Performance Targets

| Operation | Target Time | Notes |
|-----------|-------------|-------|
| Initial cache scan | <30s | Typical system with 10GB caches |
| Large file scan (home) | <60s | ~500GB home directory |
| Duplicate scan | <120s | 50,000 files |
| Orphan detection | <10s | ~100 apps installed |
| UI responsiveness | <16ms | 60fps during operations |

**Optimization strategies**:
- Parallel directory traversal with rayon
- Streaming hash computation
- Lazy thumbnail generation
- Incremental result display
- Background threads for all I/O
