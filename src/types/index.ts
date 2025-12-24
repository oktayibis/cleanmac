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
  // Extra UI fields
  formattedSize?: string;
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
  categories: string[]; 
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

// History
export interface CleaningHistory {
  entries: CleaningEntry[];
}

export interface CleaningEntry {
  timestamp: number;
  spaceReclaimed: number;
  itemsCleaned: number;
  categories: string[];
  items: CleanedItem[];
}

export interface CleanedItem {
  path: string;
  size: number;
  category: string;
}
