/**
 * Application constants
 */

// Size thresholds
export const DEFAULT_LARGE_FILE_THRESHOLD_MB = 100;
export const MIN_LARGE_FILE_THRESHOLD_MB = 10;
export const MAX_LARGE_FILE_THRESHOLD_MB = 10000;

// File size units in bytes
export const BYTES_PER_KB = 1024;
export const BYTES_PER_MB = 1024 * 1024;
export const BYTES_PER_GB = 1024 * 1024 * 1024;
export const BYTES_PER_TB = 1024 * 1024 * 1024 * 1024;

// Scanning
export const SCAN_BATCH_SIZE = 100;
export const SCAN_DEBOUNCE_MS = 100;

// Cache categories
export const CACHE_CATEGORIES = [
  "Browser",
  "System",
  "Application",
  "Developer",
  "Temporary",
  "Logs",
] as const;

export type CacheCategory = (typeof CACHE_CATEGORIES)[number];

// Media types for large file categorization
export const MEDIA_TYPES = {
  Video: ["mp4", "mov", "avi", "mkv", "wmv", "flv", "webm", "m4v"],
  Image: ["jpg", "jpeg", "png", "gif", "bmp", "tiff", "webp", "heic", "raw"],
  Audio: ["mp3", "wav", "aac", "flac", "ogg", "m4a", "wma"],
  Archive: ["zip", "rar", "7z", "tar", "gz", "bz2", "xz", "dmg", "iso"],
  Document: ["pdf", "doc", "docx", "xls", "xlsx", "ppt", "pptx", "pages", "numbers", "keynote"],
  Application: ["app", "pkg", "dmg"],
} as const;

export type MediaType = keyof typeof MEDIA_TYPES;

// Developer tool indicators
export const DEVELOPER_TOOLS = {
  Xcode: {
    paths: ["/Applications/Xcode.app", "~/Library/Developer"],
    cachePaths: ["~/Library/Developer/Xcode/DerivedData"],
  },
  Homebrew: {
    paths: ["/opt/homebrew", "/usr/local/Homebrew"],
    cachePaths: ["~/Library/Caches/Homebrew"],
  },
  NodeNpm: {
    paths: ["~/.npm", "~/.nvm"],
    cachePaths: ["~/.npm/_cacache", "~/node_modules"],
  },
  Rust: {
    paths: ["~/.cargo", "~/.rustup"],
    cachePaths: ["~/.cargo/registry"],
  },
  Python: {
    paths: ["~/.pyenv", "~/.virtualenvs"],
    cachePaths: ["~/.cache/pip"],
  },
  Docker: {
    paths: ["/Applications/Docker.app", "~/.docker"],
    cachePaths: ["~/Library/Containers/com.docker.docker"],
  },
} as const;

export type DeveloperToolType = keyof typeof DEVELOPER_TOOLS;

// UI Constants
export const SIDEBAR_WIDTH = 224; // 14rem
export const HEADER_HEIGHT = 48;
export const STATUS_BAR_HEIGHT = 32;

// Animation durations (ms)
export const ANIMATION_DURATION_FAST = 150;
export const ANIMATION_DURATION_NORMAL = 300;
export const ANIMATION_DURATION_SLOW = 500;

// Disk usage thresholds for warnings
export const DISK_USAGE_WARNING_THRESHOLD = 0.8; // 80%
export const DISK_USAGE_CRITICAL_THRESHOLD = 0.95; // 95%

/**
 * Helper to check if a file extension matches a media type
 */
export function getMediaType(extension: string): MediaType | null {
  const ext = extension.toLowerCase();
  for (const [type, extensions] of Object.entries(MEDIA_TYPES)) {
    if ((extensions as readonly string[]).includes(ext)) {
      return type as MediaType;
    }
  }
  return null;
}

/**
 * Convert megabytes to bytes
 */
export function mbToBytes(mb: number): number {
  return mb * BYTES_PER_MB;
}

/**
 * Convert bytes to megabytes
 */
export function bytesToMb(bytes: number): number {
  return bytes / BYTES_PER_MB;
}

/**
 * Check if disk usage is at warning level
 */
export function isDiskUsageWarning(usedRatio: number): boolean {
  return usedRatio >= DISK_USAGE_WARNING_THRESHOLD && usedRatio < DISK_USAGE_CRITICAL_THRESHOLD;
}

/**
 * Check if disk usage is at critical level
 */
export function isDiskUsageCritical(usedRatio: number): boolean {
  return usedRatio >= DISK_USAGE_CRITICAL_THRESHOLD;
}
