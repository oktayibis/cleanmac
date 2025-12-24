import { describe, it, expect } from "vitest";
import {
  formatBytes,
  formatRelativeTime,
  formatDate,
  formatDateTime,
  formatNumber,
  formatPercentage,
  truncatePath,
  getFileExtension,
  getFileName,
} from "./format";

describe("formatBytes", () => {
  it("should format 0 bytes", () => {
    expect(formatBytes(0)).toBe("0 Bytes");
  });

  it("should format negative bytes as invalid", () => {
    expect(formatBytes(-100)).toBe("Invalid size");
  });

  it("should format bytes correctly", () => {
    expect(formatBytes(500)).toBe("500 Bytes");
  });

  it("should format kilobytes correctly", () => {
    expect(formatBytes(1024)).toBe("1 KB");
    expect(formatBytes(1536)).toBe("1.5 KB");
  });

  it("should format megabytes correctly", () => {
    expect(formatBytes(1048576)).toBe("1 MB");
    expect(formatBytes(5242880)).toBe("5 MB");
  });

  it("should format gigabytes correctly", () => {
    expect(formatBytes(1073741824)).toBe("1 GB");
    expect(formatBytes(10737418240)).toBe("10 GB");
  });

  it("should format terabytes correctly", () => {
    expect(formatBytes(1099511627776)).toBe("1 TB");
  });

  it("should respect decimal places", () => {
    expect(formatBytes(1536, 0)).toBe("2 KB");
    expect(formatBytes(1536, 1)).toBe("1.5 KB");
    expect(formatBytes(1536, 3)).toBe("1.5 KB");
  });
});

describe("formatRelativeTime", () => {
  it("should format just now", () => {
    const now = Date.now();
    expect(formatRelativeTime(now)).toBe("Just now");
    expect(formatRelativeTime(now - 30000)).toBe("Just now");
  });

  it("should format minutes ago", () => {
    const now = Date.now();
    expect(formatRelativeTime(now - 60000)).toBe("1 minute ago");
    expect(formatRelativeTime(now - 300000)).toBe("5 minutes ago");
  });

  it("should format hours ago", () => {
    const now = Date.now();
    expect(formatRelativeTime(now - 3600000)).toBe("1 hour ago");
    expect(formatRelativeTime(now - 7200000)).toBe("2 hours ago");
  });

  it("should format days ago", () => {
    const now = Date.now();
    expect(formatRelativeTime(now - 86400000)).toBe("1 day ago");
    expect(formatRelativeTime(now - 172800000)).toBe("2 days ago");
  });

  it("should format weeks ago", () => {
    const now = Date.now();
    expect(formatRelativeTime(now - 604800000)).toBe("1 week ago");
    expect(formatRelativeTime(now - 1209600000)).toBe("2 weeks ago");
  });

  it("should format months ago", () => {
    const now = Date.now();
    expect(formatRelativeTime(now - 2592000000)).toBe("1 month ago");
    expect(formatRelativeTime(now - 5184000000)).toBe("2 months ago");
  });

  it("should format years ago", () => {
    const now = Date.now();
    expect(formatRelativeTime(now - 31536000000)).toBe("1 year ago");
    expect(formatRelativeTime(now - 63072000000)).toBe("2 years ago");
  });
});

describe("formatDate", () => {
  it("should format date correctly", () => {
    // Test with a known date
    const date = new Date("2024-01-15T12:00:00Z").getTime();
    const result = formatDate(date);
    expect(result).toContain("Jan");
    expect(result).toContain("15");
    expect(result).toContain("2024");
  });
});

describe("formatDateTime", () => {
  it("should format date and time correctly", () => {
    const date = new Date("2024-01-15T14:30:00Z").getTime();
    const result = formatDateTime(date);
    expect(result).toContain("Jan");
    expect(result).toContain("15");
    expect(result).toContain("2024");
  });
});

describe("formatNumber", () => {
  it("should format numbers with thousand separators", () => {
    expect(formatNumber(1000)).toBe("1,000");
    expect(formatNumber(1000000)).toBe("1,000,000");
    expect(formatNumber(123456789)).toBe("123,456,789");
  });

  it("should handle small numbers", () => {
    expect(formatNumber(0)).toBe("0");
    expect(formatNumber(999)).toBe("999");
  });
});

describe("formatPercentage", () => {
  it("should format percentage correctly", () => {
    expect(formatPercentage(50, 100)).toBe("50.0%");
    expect(formatPercentage(1, 3)).toBe("33.3%");
    expect(formatPercentage(0, 100)).toBe("0.0%");
  });

  it("should handle zero total", () => {
    expect(formatPercentage(50, 0)).toBe("0%");
  });

  it("should handle 100%", () => {
    expect(formatPercentage(100, 100)).toBe("100.0%");
  });
});

describe("truncatePath", () => {
  it("should not truncate short paths", () => {
    expect(truncatePath("/Users/test/file.txt")).toBe("/Users/test/file.txt");
  });

  it("should truncate long paths", () => {
    const longPath =
      "/Users/username/Documents/Projects/very/long/nested/path/to/some/file.txt";
    const result = truncatePath(longPath, 40);
    expect(result.length).toBeLessThanOrEqual(40);
    expect(result).toContain("...");
  });

  it("should preserve filename", () => {
    const longPath =
      "/Users/username/Documents/Projects/myfile.txt";
    const result = truncatePath(longPath, 30);
    expect(result).toContain("myfile.txt");
  });

  it("should handle custom max length", () => {
    const path = "/Users/test/Documents/file.txt";
    expect(truncatePath(path, 100)).toBe(path);
  });
});

describe("getFileExtension", () => {
  it("should get file extension", () => {
    expect(getFileExtension("file.txt")).toBe("txt");
    expect(getFileExtension("image.PNG")).toBe("png");
    expect(getFileExtension("archive.tar.gz")).toBe("gz");
  });

  it("should handle files without extension", () => {
    expect(getFileExtension("Makefile")).toBe("");
    expect(getFileExtension("README")).toBe("");
  });

  it("should handle paths", () => {
    expect(getFileExtension("/path/to/file.js")).toBe("js");
  });
});

describe("getFileName", () => {
  it("should get file name from path", () => {
    expect(getFileName("/path/to/file.txt")).toBe("file.txt");
    expect(getFileName("/Users/test/Documents/report.pdf")).toBe("report.pdf");
  });

  it("should handle file name only", () => {
    expect(getFileName("file.txt")).toBe("file.txt");
  });

  it("should handle empty string", () => {
    expect(getFileName("")).toBe("");
  });

  it("should handle trailing slash", () => {
    expect(getFileName("/path/to/")).toBe("");
  });
});
