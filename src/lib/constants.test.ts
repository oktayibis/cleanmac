import { describe, it, expect } from "vitest";
import {
  BYTES_PER_KB,
  BYTES_PER_MB,
  BYTES_PER_GB,
  BYTES_PER_TB,
  CACHE_CATEGORIES,
  MEDIA_TYPES,
  DISK_USAGE_WARNING_THRESHOLD,
  DISK_USAGE_CRITICAL_THRESHOLD,
  getMediaType,
  mbToBytes,
  bytesToMb,
  isDiskUsageWarning,
  isDiskUsageCritical,
} from "./constants";

describe("Byte constants", () => {
  it("should have correct byte values", () => {
    expect(BYTES_PER_KB).toBe(1024);
    expect(BYTES_PER_MB).toBe(1024 * 1024);
    expect(BYTES_PER_GB).toBe(1024 * 1024 * 1024);
    expect(BYTES_PER_TB).toBe(1024 * 1024 * 1024 * 1024);
  });
});

describe("CACHE_CATEGORIES", () => {
  it("should contain all expected categories", () => {
    expect(CACHE_CATEGORIES).toContain("Browser");
    expect(CACHE_CATEGORIES).toContain("System");
    expect(CACHE_CATEGORIES).toContain("Application");
    expect(CACHE_CATEGORIES).toContain("Developer");
    expect(CACHE_CATEGORIES).toContain("Temporary");
    expect(CACHE_CATEGORIES).toContain("Logs");
  });

  it("should have 6 categories", () => {
    expect(CACHE_CATEGORIES.length).toBe(6);
  });
});

describe("MEDIA_TYPES", () => {
  it("should contain video extensions", () => {
    expect(MEDIA_TYPES.Video).toContain("mp4");
    expect(MEDIA_TYPES.Video).toContain("mov");
    expect(MEDIA_TYPES.Video).toContain("mkv");
  });

  it("should contain image extensions", () => {
    expect(MEDIA_TYPES.Image).toContain("jpg");
    expect(MEDIA_TYPES.Image).toContain("png");
    expect(MEDIA_TYPES.Image).toContain("heic");
  });

  it("should contain audio extensions", () => {
    expect(MEDIA_TYPES.Audio).toContain("mp3");
    expect(MEDIA_TYPES.Audio).toContain("wav");
  });

  it("should contain archive extensions", () => {
    expect(MEDIA_TYPES.Archive).toContain("zip");
    expect(MEDIA_TYPES.Archive).toContain("dmg");
  });

  it("should contain document extensions", () => {
    expect(MEDIA_TYPES.Document).toContain("pdf");
    expect(MEDIA_TYPES.Document).toContain("docx");
  });
});

describe("getMediaType", () => {
  it("should return Video for video extensions", () => {
    expect(getMediaType("mp4")).toBe("Video");
    expect(getMediaType("MP4")).toBe("Video");
    expect(getMediaType("mov")).toBe("Video");
  });

  it("should return Image for image extensions", () => {
    expect(getMediaType("jpg")).toBe("Image");
    expect(getMediaType("PNG")).toBe("Image");
    expect(getMediaType("heic")).toBe("Image");
  });

  it("should return Audio for audio extensions", () => {
    expect(getMediaType("mp3")).toBe("Audio");
    expect(getMediaType("WAV")).toBe("Audio");
  });

  it("should return Archive for archive extensions", () => {
    expect(getMediaType("zip")).toBe("Archive");
    expect(getMediaType("DMG")).toBe("Archive");
  });

  it("should return Document for document extensions", () => {
    expect(getMediaType("pdf")).toBe("Document");
    expect(getMediaType("DOCX")).toBe("Document");
  });

  it("should return null for unknown extensions", () => {
    expect(getMediaType("xyz")).toBe(null);
    expect(getMediaType("unknown")).toBe(null);
    expect(getMediaType("")).toBe(null);
  });
});

describe("mbToBytes", () => {
  it("should convert MB to bytes", () => {
    expect(mbToBytes(1)).toBe(1048576);
    expect(mbToBytes(10)).toBe(10485760);
    expect(mbToBytes(100)).toBe(104857600);
  });

  it("should handle zero", () => {
    expect(mbToBytes(0)).toBe(0);
  });

  it("should handle decimals", () => {
    expect(mbToBytes(0.5)).toBe(524288);
  });
});

describe("bytesToMb", () => {
  it("should convert bytes to MB", () => {
    expect(bytesToMb(1048576)).toBe(1);
    expect(bytesToMb(10485760)).toBe(10);
    expect(bytesToMb(104857600)).toBe(100);
  });

  it("should handle zero", () => {
    expect(bytesToMb(0)).toBe(0);
  });

  it("should return decimals", () => {
    expect(bytesToMb(524288)).toBe(0.5);
  });
});

describe("Disk usage thresholds", () => {
  it("should have correct threshold values", () => {
    expect(DISK_USAGE_WARNING_THRESHOLD).toBe(0.8);
    expect(DISK_USAGE_CRITICAL_THRESHOLD).toBe(0.95);
  });
});

describe("isDiskUsageWarning", () => {
  it("should return false below warning threshold", () => {
    expect(isDiskUsageWarning(0.5)).toBe(false);
    expect(isDiskUsageWarning(0.79)).toBe(false);
  });

  it("should return true at warning threshold", () => {
    expect(isDiskUsageWarning(0.8)).toBe(true);
    expect(isDiskUsageWarning(0.85)).toBe(true);
    expect(isDiskUsageWarning(0.94)).toBe(true);
  });

  it("should return false at critical threshold", () => {
    expect(isDiskUsageWarning(0.95)).toBe(false);
    expect(isDiskUsageWarning(0.99)).toBe(false);
  });
});

describe("isDiskUsageCritical", () => {
  it("should return false below critical threshold", () => {
    expect(isDiskUsageCritical(0.5)).toBe(false);
    expect(isDiskUsageCritical(0.8)).toBe(false);
    expect(isDiskUsageCritical(0.94)).toBe(false);
  });

  it("should return true at or above critical threshold", () => {
    expect(isDiskUsageCritical(0.95)).toBe(true);
    expect(isDiskUsageCritical(0.99)).toBe(true);
    expect(isDiskUsageCritical(1.0)).toBe(true);
  });
});
