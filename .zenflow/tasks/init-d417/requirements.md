# Product Requirements Document: CleanMac

## Overview

**Product Name**: CleanMac (working title)
**Platform**: macOS 12 Monterey and newer
**Distribution**: Direct download (notarized app, not App Store)
**Technology Stack**: Tauri (Rust backend + Web UI frontend)

CleanMac is a macOS disk cleanup and optimization application that intelligently identifies and removes unnecessary files while respecting developer workflows and providing a clean, modern user interface.

---

## Goals

1. **Reclaim disk space** by identifying and removing cache files, temporary files, and unused data
2. **Respect developer workflows** by detecting developer environments and protecting important caches
3. **Clean up application leftovers** from uninstalled applications
4. **Find large and duplicate files** to help users make informed cleanup decisions
5. **Provide a safe, transparent experience** with clear explanations of what will be deleted

---

## Target Users

### Regular Users
- Want to free up disk space
- Don't understand technical file system details
- Need clear guidance on what's safe to delete

### Developer Users
- Have development tools installed (Xcode, Homebrew, Node.js, etc.)
- Need to preserve important caches (npm, cargo, pip, CocoaPods, etc.)
- Want control over what gets cleaned

---

## Functional Requirements

### FR-1: User Profile Detection & Selection

**FR-1.1**: Auto-detect developer environment by checking for:
- Xcode installation and developer tools
- Homebrew (`/opt/homebrew`, `/usr/local/Homebrew`)
- Node.js/npm (`~/.npm`, `~/.nvm`)
- Python/pip (`~/.pyenv`, `~/Library/Caches/pip`)
- Rust/Cargo (`~/.cargo`, `~/.rustup`)
- Ruby/Gems (`~/.rbenv`, `~/.gem`)
- Docker (`~/.docker`)
- Git repositories presence
- IDE configurations (VS Code, JetBrains, etc.)

**FR-1.2**: Allow manual profile selection:
- "Regular User" - aggressive cleanup mode
- "Developer" - preserves development caches
- "Custom" - granular control over what to preserve

**FR-1.3**: Remember user profile selection across sessions

---

### FR-2: System Cache Analysis & Cleaning

**FR-2.1**: Analyze system caches in:
- `~/Library/Caches/` - User application caches
- `/Library/Caches/` - System-wide caches (requires elevated permissions)
- `~/Library/Logs/` - Application logs
- `/var/log/` - System logs (with care)
- `~/Library/Application Support/` - App data (selective)

**FR-2.2**: Categorize caches by:
- Browser caches (Safari, Chrome, Firefox, Edge, Arc)
- Application caches (grouped by app)
- System caches
- Temporary files (`/tmp`, `/var/folders`)

**FR-2.3**: For developer mode, protect:
- Package manager caches (npm, yarn, pnpm, cargo, pip, gem, pod, etc.)
- Build caches (Xcode DerivedData - with option to clean)
- Container images and volumes
- Virtual environment caches

**FR-2.4**: Display cache age and last access time when available

---

### FR-3: Developer Cache Management

**FR-3.1**: Identify developer-specific caches:
- `~/Library/Developer/Xcode/DerivedData/` - Xcode build cache
- `~/Library/Developer/Xcode/Archives/` - Old archives
- `~/Library/Developer/Xcode/iOS DeviceSupport/` - Device support files
- `~/.npm/_cacache/` - npm cache
- `~/.cargo/registry/` - Cargo packages
- `~/.gradle/caches/` - Gradle cache
- `~/.m2/repository/` - Maven cache
- `~/Library/Caches/CocoaPods/` - CocoaPods cache
- `~/.cache/` - XDG cache directory

**FR-3.2**: Provide intelligent recommendations:
- "Safe to clean" - caches that will be regenerated
- "Clean with caution" - may require re-downloading
- "Keep" - actively used or critical

**FR-3.3**: Show space usage per developer tool

---

### FR-4: Application Leftover Detection

**FR-4.1**: Build application inventory by scanning:
- `/Applications/`
- `~/Applications/`
- `/System/Applications/` (reference only, not for cleanup)

**FR-4.2**: Detect orphaned files by cross-referencing:
- `~/Library/Application Support/<AppName>/`
- `~/Library/Preferences/com.<developer>.<appname>.plist`
- `~/Library/Caches/<BundleIdentifier>/`
- `~/Library/Saved Application State/<BundleIdentifier>.savedState/`
- `~/Library/Containers/<BundleIdentifier>/`
- `~/Library/Group Containers/`
- `~/Library/HTTPStorages/`
- `~/Library/WebKit/`
- `~/Library/Cookies/`

**FR-4.3**: Use bundle identifier matching:
- Extract bundle IDs from installed apps
- Match against files in Library directories
- Files with bundle IDs not matching any installed app = orphaned

**FR-4.4**: Display orphaned files grouped by (presumed) original application

**FR-4.5**: Allow users to:
- Clean all orphaned files
- Select specific applications' leftovers to clean
- Exclude certain leftovers from future scans

---

### FR-5: Large File Finder

**FR-5.1**: Scan user-accessible directories for large files:
- Home directory (`~`)
- External volumes (optional)
- Exclude system directories

**FR-5.2**: Filter by file type:
- Videos (`.mp4`, `.mov`, `.avi`, `.mkv`, `.wmv`, `.flv`)
- Images (`.jpg`, `.png`, `.gif`, `.raw`, `.heic`, `.psd`, `.tiff`)
- Archives (`.zip`, `.rar`, `.7z`, `.tar`, `.gz`, `.dmg`)
- Documents (`.pdf`, large `.docx`, etc.)
- Other large files

**FR-5.3**: Configurable size threshold (default: 100MB)

**FR-5.4**: Sort by:
- Size (largest first)
- Last accessed date
- File type

**FR-5.5**: Preview capability:
- Thumbnails for images
- Quick Look integration for other files
- Show file path and metadata

**FR-5.6**: Actions:
- Move to Trash
- Reveal in Finder
- Open file
- Add to exclusion list

---

### FR-6: Duplicate File Finder

**FR-6.1**: Detect duplicates using:
- File size comparison (first pass)
- Partial hash comparison (second pass - first/last 4KB)
- Full hash comparison (final verification - SHA-256)

**FR-6.2**: Scan locations:
- User-selected directories
- Common locations (Downloads, Documents, Desktop)
- Photo libraries (with care for managed libraries)

**FR-6.3**: Group duplicates by:
- Original + copies (oldest = original)
- File location
- Total reclaimable space

**FR-6.4**: Smart selection:
- Auto-select duplicates (keep originals)
- Prefer files in organized locations over Downloads
- Never auto-select files in protected locations

**FR-6.5**: Actions:
- Delete selected duplicates (to Trash)
- Replace with hard links (advanced option)
- Keep one, move others to review folder

---

### FR-7: Auto-Clean Mode

**FR-7.1**: Define trusted categories for auto-clean:
- Browser caches
- System logs older than 30 days
- Temporary files
- Trash (optional, with age threshold)

**FR-7.2**: Require explicit opt-in for auto-clean

**FR-7.3**: Provide detailed log of auto-cleaned items

**FR-7.4**: Schedule options:
- On-demand only
- Daily/Weekly/Monthly
- On low disk space (threshold configurable)

**FR-7.5**: Notification after auto-clean with summary

---

### FR-8: Safety & Recovery

**FR-8.1**: Default behavior: Move to Trash (recoverable)

**FR-8.2**: Option for permanent deletion (with warning)

**FR-8.3**: Pre-deletion confirmation showing:
- Total items to delete
- Total space to reclaim
- List of items (collapsible by category)

**FR-8.4**: Maintain cleaning history log:
- Date/time of cleanup
- Items cleaned
- Space reclaimed
- Allows audit of past actions

**FR-8.5**: Exclusion list:
- User can mark files/folders to never clean
- Persisted across sessions

---

## Non-Functional Requirements

### NFR-1: Performance

- Initial scan should complete within 60 seconds for typical system
- UI must remain responsive during scans (background processing)
- Incremental scans for repeat analysis
- Cancel operation at any time

### NFR-2: Security

- Request Full Disk Access permission on first run (required for complete scan)
- No data collection or telemetry
- All operations local to the machine
- Code signed and notarized for Gatekeeper

### NFR-3: User Experience

- Clean, modern UI following macOS design patterns
- Dark mode support
- Clear progress indicators
- Helpful tooltips and explanations
- Keyboard navigation support
- Accessibility compliance (VoiceOver support)

### NFR-4: System Integration

- Menu bar icon option for quick access
- System notifications for completed operations
- Respect system appearance settings
- Native file dialogs and Quick Look

### NFR-5: Reliability

- Graceful handling of permission errors
- No crashes on inaccessible files
- Atomic operations where possible
- Logging for troubleshooting

---

## User Interface Structure

### Main Window

1. **Dashboard**
   - Disk usage overview (pie/bar chart)
   - Quick stats: reclaimable space, orphaned apps, duplicates found
   - Quick actions: "Smart Clean", "Full Scan"
   - User profile indicator (Regular/Developer)

2. **Sidebar Navigation**
   - Dashboard
   - System Cache
   - Developer Tools (if developer mode)
   - App Leftovers
   - Large Files
   - Duplicates
   - Settings

3. **Content Area**
   - Category-specific interface
   - Selection controls
   - Action buttons

4. **Status Bar**
   - Current operation status
   - Last scan time
   - Disk space indicator

### Settings

- User profile selection
- Auto-clean configuration
- Exclusion list management
- Scan locations
- Appearance (theme)
- Notifications

---

## Out of Scope (v1.0)

- iOS device cleanup
- Cloud storage cleanup (iCloud, Dropbox, etc.)
- RAM optimization/memory cleaning
- Startup item management
- Malware scanning
- File shredding/secure delete
- Network drive scanning
- Time Machine management

---

## Success Metrics

- Successfully identifies >90% of common cache locations
- Accurate orphan detection (minimal false positives)
- Duplicate detection accuracy >99%
- Scan performance within NFR targets
- Zero data loss incidents

---

## Assumptions

1. User will grant Full Disk Access permission for complete functionality
2. macOS file system APIs (FSEvents, Spotlight metadata) are available
3. Users understand that cleaning caches may require re-downloading data
4. Trash functionality works as expected for recovery

---

## Risks & Mitigations

| Risk | Impact | Mitigation |
|------|--------|------------|
| False positive orphan detection | User loses needed data | Conservative matching, Trash-first, exclusion lists |
| Permission denied errors | Incomplete scan | Clear messaging, guide to grant permissions |
| Performance on large disks | Poor UX | Background processing, incremental scans, cancellation |
| Breaking developer environments | Lost productivity | Developer mode, protected paths, clear warnings |

---

## Appendix: Common Cache Locations Reference

### Browser Caches
- Safari: `~/Library/Caches/com.apple.Safari/`
- Chrome: `~/Library/Caches/Google/Chrome/`
- Firefox: `~/Library/Caches/Firefox/`
- Edge: `~/Library/Caches/Microsoft Edge/`
- Arc: `~/Library/Caches/company.thebrowser.Browser/`

### Developer Caches
- Xcode DerivedData: `~/Library/Developer/Xcode/DerivedData/`
- npm: `~/.npm/`
- Yarn: `~/Library/Caches/Yarn/`
- Cargo: `~/.cargo/registry/cache/`
- pip: `~/Library/Caches/pip/`
- CocoaPods: `~/Library/Caches/CocoaPods/`
- Gradle: `~/.gradle/caches/`
- Maven: `~/.m2/repository/`

### System Caches
- User caches: `~/Library/Caches/`
- System caches: `/Library/Caches/`
- Temporary: `/tmp/`, `/var/folders/`
- Logs: `~/Library/Logs/`, `/var/log/`
