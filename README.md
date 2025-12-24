# CleanMac

A native macOS disk cleanup and optimization utility built with Tauri and React.

![macOS](https://img.shields.io/badge/macOS-12%2B-blue)
![License](https://img.shields.io/badge/license-MIT-green)
![Rust](https://img.shields.io/badge/rust-1.75%2B-orange)
![Tauri](https://img.shields.io/badge/tauri-2.x-purple)

## Overview

CleanMac helps you reclaim disk space on your Mac by intelligently identifying and removing unnecessary files while protecting important data. Unlike other cleanup tools, CleanMac is **developer-aware** and won't accidentally delete your development caches and build artifacts.

## Features

- **System Cache Cleaning** - Safely remove system, browser, and application caches
- **Developer Mode** - Automatically detects developer tools and protects important caches (npm, cargo, Xcode DerivedData, etc.)
- **Application Leftovers** - Find and remove files left behind by uninstalled applications
- **Large File Finder** - Discover large files taking up space with preview support
- **Duplicate Detection** - Find duplicate files using smart hash-based comparison
- **Safe Deletion** - Files are moved to Trash by default for easy recovery

## Screenshots

*Coming soon*

## Requirements

- macOS 12 Monterey or later (ARM64 and Intel)
- Full Disk Access permission (for complete scanning)

## Installation

### Download

Download the latest release from the [Releases](https://github.com/yourusername/cleanmac/releases) page.

### Build from Source

See [DEVELOPMENT.md](DEVELOPMENT.md) for detailed build instructions.

```bash
# Quick start
git clone https://github.com/yourusername/cleanmac.git
cd cleanmac
pnpm install
pnpm tauri build
```

## Usage

1. **Grant Full Disk Access** - On first launch, CleanMac will guide you to grant Full Disk Access in System Preferences for complete scanning capabilities.

2. **Choose Your Profile**
   - **Regular** - Standard cleanup suitable for most users
   - **Developer** - Protects development caches and build artifacts

3. **Scan & Clean** - Use the dashboard to scan for cleanable files, review what will be removed, and clean with confidence.

## Tech Stack

| Component | Technology |
|-----------|------------|
| Framework | [Tauri 2.x](https://tauri.app/) |
| Backend | Rust 1.75+ |
| Frontend | React 19, TypeScript 5.x |
| Styling | Tailwind CSS 3.x |
| State | Zustand |
| Build | Vite 7.x, pnpm |

## Project Structure

```
cleanmac/
├── src/                    # React frontend
│   ├── components/         # UI components
│   ├── stores/             # Zustand state stores
│   ├── hooks/              # Custom React hooks
│   ├── lib/                # Utilities and API wrappers
│   ├── types/              # TypeScript type definitions
│   └── pages/              # Page components
├── src-tauri/              # Rust backend
│   ├── src/
│   │   ├── commands/       # Tauri command handlers
│   │   ├── scanner/        # File scanning logic
│   │   ├── analyzer/       # Analysis (developer detection, etc.)
│   │   ├── cleaner/        # Safe deletion operations
│   │   ├── models/         # Data structures
│   │   └── utils/          # Utilities
│   └── Cargo.toml
└── package.json
```

## Privacy & Security

- **100% Local** - All operations happen on your device. No data is sent anywhere.
- **No Telemetry** - CleanMac does not collect any usage data or analytics.
- **Open Source** - Full source code available for audit.
- **Safe by Default** - Files go to Trash first, allowing easy recovery.

## Contributing

Contributions are welcome! Please read the [Contributing Guidelines](CONTRIBUTING.md) before submitting a PR.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## Development

See [DEVELOPMENT.md](DEVELOPMENT.md) for detailed development setup and guidelines.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Built with [Tauri](https://tauri.app/)
- Icons and design inspired by macOS
- Thanks to all contributors

---

**Note:** CleanMac is not affiliated with Apple Inc. macOS is a trademark of Apple Inc.
