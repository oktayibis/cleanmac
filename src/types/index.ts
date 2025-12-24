// Mirror of Rust models

// from file_entry.rs
export type FileType = "File" | "Directory" | "Symlink" | "Unknown";

export interface FileEntry {
  path: string;
  name: string;
  size: number;
  fileType: FileType;
  created?: string; // ISO 8601 string
  modified?: string; // ISO 8601 string
  accessed?: string; // ISO 8601 string
  extension?: string;
}

// from scan_result.rs
export type SafetyLevel = "Safe" | "Warning" | "Protected";

export interface CacheItem {
  id: string;
  path: string;
  name: string;
  size: number;
  fileType: FileType;
  modified?: string; // ISO 8601 string
  safetyLevel: SafetyLevel;
  description?: string;
  selected: boolean;
}

export interface CacheCategory {
  id: string;
  name: string;
  description: string;
  items: CacheItem[];
  totalSize: number;
  selected: boolean;
  icon: string;
}

export interface CacheScanResult {
  categories: CacheCategory[];
  totalWastedSize: number;
  scanDurationMs: number;
  scannedAt: string; // ISO 8601 string
}

// from config.rs
export type Theme = "System" | "Light" | "Dark";

export type UserProfile = "Standard" | "Developer" | "PowerUser";

export interface AutoCleanConfig {
  enabled: boolean;
  frequencyDays: number;
  nextRun?: string; // ISO 8601 string
  notifyOnCompletion: boolean;
  trustedCategories: string[];
}

export interface AppConfig {
  theme: Theme;
  userProfile: UserProfile;
  autoClean: AutoCleanConfig;
  excludedPaths: string[]; // Paths as strings
  scanThresholdMb: number;
  lastScan?: string; // ISO 8601 string
}

// from history.rs
export interface CleanedItem {
  path: string;
  size: number;
  categoryId: string;
}

export interface CleaningEntry {
  id: string;
  timestamp: string; // ISO 8601 string
  totalSizeReclaimed: number;
  itemsCount: number;
  items: CleanedItem[];
  durationMs: number;
}

export interface CleaningHistory {
  entries: CleaningEntry[];
  totalLifetimeReclaimed: number;
}

// Additional UI/Command types
export interface ScanProgress {
  status: "idle" | "scanning" | "cleaning" | "completed" | "error";
  currentPath?: string;
  progress: number; // 0-100
  totalBytes?: number;
  scannedBytes?: number;
  message?: string;
}

export interface CleaningResult {
  success: boolean;
  reclaimedSpace: number;
  itemsRemoved: number;
  errors: string[];
}

export interface DiskInfo {
  totalSpace: number;
  availableSpace: number;
  usedSpace: number;
  mountPoint: string;
  name: string;
}
