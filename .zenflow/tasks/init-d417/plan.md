# CleanMac Implementation Plan

## Configuration
- **Artifacts Path**: `.zenflow/tasks/init-d417`
- **Reference Documents**:
  - `requirements.md` - Product Requirements Document
  - `spec.md` - Technical Specification
- **Rules**:
  1. Test each step before moving to the next.
  2. Create a different branch for each step.
  3. Add enough testing to pass quality gate.
  4. Run quality gate and make sure everything is right.

---

## Workflow Steps

### [x] Step: Requirements
<!-- chat-id: c1ba14b2-45fb-4a78-a321-cccd7dde3235 -->

Create a Product Requirements Document (PRD) based on the feature description.

### [x] Step: Technical Specification
<!-- chat-id: d1e9f050-588c-44fa-a6b4-5d8546d48fcd -->

Create a technical specification based on the PRD in `requirements.md`.

### [x] Step: Planning
<!-- chat-id: 6256b734-999b-4a16-b3e4-e1e05ed5ea93 -->

Create a detailed implementation plan based on `spec.md`.

---

## Phase 1: Foundation (MVP)

### [x] Step: Project Setup
<!-- chat-id: 60d6fc6a-5ca9-4aca-9b8f-49429e32206a -->
<!-- phase: 1 -->

Initialize the Tauri + React project structure.

**Tasks**:
1. Create `.gitignore` with standard patterns (node_modules, dist, target, etc.)
2. Initialize Tauri project with `pnpm create tauri-app`
3. Configure project for TypeScript, React, and Tailwind CSS
4. Set up Vite configuration for development
5. Configure Tauri v2 with proper capabilities and permissions
6. Add Rust dependencies to `Cargo.toml` as specified in spec.md
7. Create initial directory structure for both frontend and backend

**Verification**:
- `pnpm install` completes successfully
- `pnpm tauri dev` launches the application window
- Hot reload works for frontend changes

**Files to create**:
- `.gitignore`
- `package.json`, `pnpm-lock.yaml`
- `tsconfig.json`, `vite.config.ts`
- `tailwind.config.js`, `postcss.config.js`
- `index.html`
- `src-tauri/Cargo.toml`, `src-tauri/tauri.conf.json`
- `src-tauri/capabilities/default.json`

---

### [x] Step: Rust Data Models
<!-- chat-id: c40132f0-29cc-42da-95df-b5406c251877 -->
<!-- phase: 1 -->

Implement core data structures in Rust backend.

**Tasks**:
1. Create `src-tauri/src/models/mod.rs` with module exports
2. Implement `file_entry.rs` with FileEntry, FileType structs
3. Implement `scan_result.rs` with CacheScanResult, CacheCategory, CacheItem, SafetyLevel
4. Implement `config.rs` with AppConfig, UserProfile, AutoCleanConfig, Theme
5. Implement `history.rs` with CleaningHistory, CleaningEntry, CleanedItem

**Verification**:
- `cargo build` compiles without errors
- `cargo test` passes for serialization/deserialization tests

**Files to create**:
- `src-tauri/src/models/mod.rs`
- `src-tauri/src/models/file_entry.rs`
- `src-tauri/src/models/scan_result.rs`
- `src-tauri/src/models/config.rs`
- `src-tauri/src/models/history.rs`

---

### [x] Step: TypeScript Types
<!-- phase: 1 -->

Define TypeScript interfaces mirroring Rust models.

**Tasks**:
1. Create `src/types/index.ts` with all interface definitions
2. Define FileEntry, CacheScanResult, CacheCategory, CacheItem types
3. Define AppConfig, UserProfile, AutoCleanConfig types
4. Define ScanProgress, CleaningResult, DiskInfo types

**Verification**:
- `pnpm typecheck` passes
- Types correctly mirror Rust struct naming conventions (camelCase)

**Files to create**:
- `src/types/index.ts`

---

### [x] Step: Rust Utilities
<!-- phase: 1 -->

Implement utility functions for file system operations.

**Tasks**:
1. Create `src-tauri/src/utils/mod.rs` with module exports
2. Implement `fs.rs` with directory size calculation, path expansion
3. Implement `hash.rs` with SHA-256 hashing functions
4. Implement `permissions.rs` with Full Disk Access checking

**Verification**:
- `cargo test` passes for utility functions
- Can correctly expand `~` to home directory
- Hash functions produce correct output for test inputs

**Files to create**:
- `src-tauri/src/utils/mod.rs`
- `src-tauri/src/utils/fs.rs`
- `src-tauri/src/utils/hash.rs`
- `src-tauri/src/utils/permissions.rs`

---

### [x] Step: Cache Scanner Implementation
<!-- phase: 1 -->

Implement the core cache scanning logic.

**Tasks**:
1. Create `src-tauri/src/scanner/mod.rs` with module exports
2. Implement `cache_scanner.rs` with CACHE_LOCATIONS constant
3. Implement directory walking with walkdir crate
4. Categorize caches by path patterns (Browser, System, Application, etc.)
5. Calculate sizes and last access times
6. Emit progress events via Tauri window

**Verification**:
- Scanner finds caches in `~/Library/Caches`
- Categories are correctly assigned
- Sizes are accurate
- Progress events fire during scan

**Files to create**:
- `src-tauri/src/scanner/mod.rs`
- `src-tauri/src/scanner/cache_scanner.rs`

---

### [x] Step: Tauri Commands - Scan
<!-- phase: 1 -->

Implement Tauri command handlers for scanning operations.

**Tasks**:
1. Create `src-tauri/src/commands/mod.rs` with module exports
2. Implement `scan.rs` with `scan_caches` command
3. Implement `cancel_scan` command with atomic cancellation flag
4. Wire up progress event emission
5. Register commands in `main.rs`

**Verification**:
- `scan_caches` command can be invoked from frontend
- Progress events are received by frontend
- Cancel stops ongoing scan

**Files to create**:
- `src-tauri/src/commands/mod.rs`
- `src-tauri/src/commands/scan.rs`
- Update `src-tauri/src/main.rs`

---

### [x] Step: Cleaner Implementation
<!-- phase: 1 -->

Implement safe file deletion operations.

**Tasks**:
1. Create `src-tauri/src/cleaner/mod.rs` with module exports
2. Implement `safe_delete.rs` using `trash` crate for recoverable deletion
3. Implement permanent deletion option
4. Track cleaning history
5. Handle permission errors gracefully

**Verification**:
- Files moved to Trash can be recovered
- Permanent deletion removes files completely
- Errors don't crash the application

**Files to create**:
- `src-tauri/src/cleaner/mod.rs`
- `src-tauri/src/cleaner/safe_delete.rs`
- `src-tauri/src/cleaner/history.rs`

---

### [x] Step: Tauri Commands - Clean & Config
<!-- phase: 1 -->

Implement Tauri commands for cleaning and configuration.

**Tasks**:
1. Implement `clean.rs` with `clean_items`, `clean_category` commands
2. Implement `get_cleaning_history` command
3. Implement `config.rs` with `get_config`, `save_config` commands
4. Implement `add_exclusion`, `remove_exclusion` commands
5. Persist config to app data directory

**Verification**:
- Config persists across app restarts
- Clean operations return accurate results
- History is recorded

**Files to create**:
- `src-tauri/src/commands/clean.rs`
- `src-tauri/src/commands/config.rs`

---

### [x] Step: Tauri Commands - System
<!-- phase: 1 -->

Implement system information commands.

**Tasks**:
1. Implement `system.rs` with `get_disk_info` command
2. Implement `check_full_disk_access` command
3. Implement `open_full_disk_access_settings` command
4. Implement `reveal_in_finder`, `open_file` commands

**Verification**:
- Disk info returns accurate values
- Full Disk Access check works correctly
- Reveal in Finder opens correct location

**Files to create**:
- `src-tauri/src/commands/system.rs`

---

### [ ] Step: Frontend Tauri API Wrapper
<!-- phase: 1 -->

Create TypeScript wrapper for Tauri commands.

**Tasks**:
1. Create `src/lib/tauri.ts` with all command wrappers
2. Create `src/lib/format.ts` with size/date formatting utilities
3. Create `src/lib/constants.ts` with application constants
4. Set up event listeners for scan progress

**Verification**:
- All commands have TypeScript wrappers
- Event listeners receive progress updates
- Format functions produce readable output

**Files to create**:
- `src/lib/tauri.ts`
- `src/lib/format.ts`
- `src/lib/constants.ts`

---

### [ ] Step: Zustand State Management
<!-- phase: 1 -->

Implement frontend state management with Zustand.

**Tasks**:
1. Create `src/stores/scanStore.ts` for scan results and progress
2. Create `src/stores/configStore.ts` for application settings
3. Create `src/stores/uiStore.ts` for UI state (sidebar, modals)
4. Implement actions for scanning, cleaning, selection

**Verification**:
- State updates propagate to components
- Actions correctly call Tauri commands
- Selection state tracks properly

**Files to create**:
- `src/stores/scanStore.ts`
- `src/stores/configStore.ts`
- `src/stores/uiStore.ts`

---

### [ ] Step: Common UI Components
<!-- phase: 1 -->

Build reusable UI components.

**Tasks**:
1. Create `src/components/common/Button.tsx` with variants
2. Create `src/components/common/ProgressBar.tsx`
3. Create `src/components/common/Modal.tsx`
4. Create `src/components/common/FileIcon.tsx`
5. Create `src/components/common/SizeDisplay.tsx`
6. Set up Tailwind with macOS-like design tokens

**Verification**:
- Components render correctly
- Styling matches macOS aesthetics
- Dark mode support works

**Files to create**:
- `src/components/common/Button.tsx`
- `src/components/common/ProgressBar.tsx`
- `src/components/common/Modal.tsx`
- `src/components/common/FileIcon.tsx`
- `src/components/common/SizeDisplay.tsx`
- `src/styles/globals.css`

---

### [ ] Step: Layout Components
<!-- phase: 1 -->

Build application shell and navigation.

**Tasks**:
1. Create `src/components/layout/Sidebar.tsx` with navigation
2. Create `src/components/layout/Header.tsx` with app title
3. Create `src/components/layout/StatusBar.tsx` with disk space indicator
4. Create `src/App.tsx` with layout structure
5. Set up routing between views

**Verification**:
- Sidebar navigation works
- Layout is responsive
- Status bar shows disk info

**Files to create**:
- `src/components/layout/Sidebar.tsx`
- `src/components/layout/Header.tsx`
- `src/components/layout/StatusBar.tsx`
- `src/App.tsx`
- `src/main.tsx`

---

### [ ] Step: Dashboard View
<!-- phase: 1 -->

Implement the main dashboard interface.

**Tasks**:
1. Create `src/components/dashboard/DiskUsageChart.tsx`
2. Create `src/components/dashboard/QuickStats.tsx`
3. Create `src/components/dashboard/QuickActions.tsx`
4. Assemble dashboard page with components
5. Connect to disk info and scan results

**Verification**:
- Dashboard displays disk usage accurately
- Quick stats show reclaimable space
- Quick actions trigger scans

**Files to create**:
- `src/components/dashboard/DiskUsageChart.tsx`
- `src/components/dashboard/QuickStats.tsx`
- `src/components/dashboard/QuickActions.tsx`
- `src/pages/Dashboard.tsx`

---

### [ ] Step: Cache View
<!-- phase: 1 -->

Implement the cache management interface.

**Tasks**:
1. Create `src/components/cache/CacheList.tsx`
2. Create `src/components/cache/CacheCategory.tsx`
3. Implement selection UI with checkboxes
4. Add category-level select all
5. Show cleaning confirmation modal

**Verification**:
- Cache list displays categories and items
- Selection works at item and category level
- Clean action removes selected items

**Files to create**:
- `src/components/cache/CacheList.tsx`
- `src/components/cache/CacheCategory.tsx`
- `src/pages/Cache.tsx`

---

### [ ] Step: Settings View
<!-- phase: 1 -->

Implement the settings interface.

**Tasks**:
1. Create `src/components/settings/SettingsPage.tsx`
2. Create `src/components/settings/ProfileSelector.tsx`
3. Create `src/components/settings/ExclusionList.tsx`
4. Implement theme toggle (System/Light/Dark)
5. Save settings on change

**Verification**:
- Profile selection persists
- Exclusion list can be edited
- Theme changes apply immediately

**Files to create**:
- `src/components/settings/SettingsPage.tsx`
- `src/components/settings/ProfileSelector.tsx`
- `src/components/settings/ExclusionList.tsx`
- `src/pages/Settings.tsx`

---

### [ ] Step: Phase 1 Integration Testing
<!-- phase: 1 -->

Test the complete Phase 1 functionality.

**Tasks**:
1. Verify app launches on macOS 12+
2. Test cache scanning finds items
3. Test cleaning moves items to Trash
4. Verify settings persist across restarts
5. Fix any bugs discovered during testing
6. Run `cargo clippy` and `pnpm lint`

**Verification**:
- All Phase 1 requirements verified manually
- No clippy warnings
- No ESLint errors
- `cargo test` passes
- `pnpm typecheck` passes

---

## Phase 2: Developer Mode

### [ ] Step: Developer Detection
<!-- phase: 2 -->

Implement developer environment detection.

**Tasks**:
1. Create `src-tauri/src/analyzer/mod.rs`
2. Implement `developer_detector.rs` with DEVELOPER_INDICATORS
3. Score developer tools presence
4. Calculate confidence level
5. Implement `detect_developer_environment` command

**Verification**:
- Correctly detects Xcode, npm, cargo installations
- Confidence score is reasonable
- Detection completes quickly

**Files to create**:
- `src-tauri/src/analyzer/mod.rs`
- `src-tauri/src/analyzer/developer_detector.rs`

---

### [ ] Step: Developer Cache Protection
<!-- phase: 2 -->

Implement protection logic for developer caches.

**Tasks**:
1. Create `src-tauri/src/analyzer/cache_categorizer.rs`
2. Define DEVELOPER_CACHE_PATTERNS constant
3. Mark developer caches as protected in dev mode
4. Add protection_reason to protected items
5. Update cache scanner to apply protection

**Verification**:
- Developer caches marked as Protected in developer mode
- Regular mode shows caches as Safe
- Protection reason is descriptive

**Files to create**:
- `src-tauri/src/analyzer/cache_categorizer.rs`

---

### [ ] Step: Developer Cache UI
<!-- phase: 2 -->

Add developer-specific UI components.

**Tasks**:
1. Create `src/components/cache/DeveloperCacheSection.tsx`
2. Add protected cache indicators (lock icon, warning)
3. Show developer tools detected
4. Conditionally show Developer Tools in sidebar
5. Update profile selector with auto-detection

**Verification**:
- Developer section appears in developer mode
- Protected caches show indicators
- Profile can be switched

**Files to create**:
- `src/components/cache/DeveloperCacheSection.tsx`
- Update `src/components/layout/Sidebar.tsx`
- Update `src/components/settings/ProfileSelector.tsx`

---

### [ ] Step: Phase 2 Integration Testing
<!-- phase: 2 -->

Test developer mode functionality.

**Tasks**:
1. Test developer detection accuracy
2. Verify protected caches not selected by default
3. Test profile switching behavior
4. Run linting and tests

**Verification**:
- Developer mode protects appropriate caches
- Switching profiles updates UI
- All tests pass

---

## Phase 3: Application Leftovers

### [ ] Step: App Registry
<!-- phase: 3 -->

Implement installed application registry.

**Tasks**:
1. Create `src-tauri/src/analyzer/app_registry.rs`
2. Scan /Applications and ~/Applications
3. Parse Info.plist files for bundle IDs
4. Build HashMap of bundle ID to app info
5. Implement is_installed() and get_app_name()

**Verification**:
- All installed apps are discovered
- Bundle IDs extracted correctly
- Lookup is fast

**Files to create**:
- `src-tauri/src/analyzer/app_registry.rs`

---

### [ ] Step: Orphan Data Models
<!-- phase: 3 -->

Add data models for orphan detection.

**Tasks**:
1. Add OrphanScanResult to `scan_result.rs`
2. Add OrphanedApp, OrphanedFile, OrphanFileType structs
3. Add corresponding TypeScript types
4. Define ORPHAN_SCAN_LOCATIONS constant

**Verification**:
- Models serialize/deserialize correctly
- TypeScript types match Rust

**Files to update**:
- `src-tauri/src/models/scan_result.rs` (add orphan types)
- `src/types/index.ts` (add orphan types)

---

### [ ] Step: Orphan Scanner
<!-- phase: 3 -->

Implement orphaned application file scanner.

**Tasks**:
1. Create `src-tauri/src/scanner/orphan_scanner.rs`
2. Scan Library directories for app-related files
3. Extract bundle IDs from file paths
4. Match against app registry
5. Group orphaned files by presumed app
6. Implement `scan_orphaned_apps` command

**Verification**:
- Detects files from uninstalled apps
- Groups correctly by app
- No false positives for installed apps

**Files to create**:
- `src-tauri/src/scanner/orphan_scanner.rs`

---

### [ ] Step: Leftovers UI
<!-- phase: 3 -->

Implement orphaned apps interface.

**Tasks**:
1. Create `src/components/leftovers/OrphanedAppList.tsx`
2. Create `src/components/leftovers/LeftoverItem.tsx`
3. Group by app with expandable sections
4. Show file types (Preferences, Cache, etc.)
5. Add to sidebar navigation

**Verification**:
- Orphaned apps display grouped
- Can select individual files or entire apps
- Clean action removes selected

**Files to create**:
- `src/components/leftovers/OrphanedAppList.tsx`
- `src/components/leftovers/LeftoverItem.tsx`
- `src/pages/Leftovers.tsx`

---

### [ ] Step: Phase 3 Integration Testing
<!-- phase: 3 -->

Test orphan detection functionality.

**Tasks**:
1. Test with known uninstalled app leftovers
2. Verify no false positives
3. Test cleaning orphaned files
4. Run linting and tests

**Verification**:
- Orphan detection is accurate
- Cleaning works correctly
- All tests pass

---

## Phase 4: Large Files

### [ ] Step: Large File Data Models
<!-- phase: 4 -->

Add data models for large file detection.

**Tasks**:
1. Add LargeFileScanResult, LargeFile, MediaType to models
2. Define MEDIA_EXTENSIONS constant
3. Add corresponding TypeScript types

**Verification**:
- Models serialize/deserialize correctly
- Media types cover common formats

**Files to update**:
- `src-tauri/src/models/scan_result.rs`
- `src/types/index.ts`

---

### [ ] Step: Large File Scanner
<!-- phase: 4 -->

Implement large file scanner.

**Tasks**:
1. Create `src-tauri/src/scanner/large_file_scanner.rs`
2. Walk home directory with rayon for parallelism
3. Filter by size threshold
4. Categorize by extension
5. Sort by size descending
6. Implement `scan_large_files` command

**Verification**:
- Finds files above threshold
- Categorization is accurate
- Performance is acceptable (<60s for 500GB)

**Files to create**:
- `src-tauri/src/scanner/large_file_scanner.rs`

---

### [ ] Step: Large Files UI
<!-- phase: 4 -->

Implement large files interface.

**Tasks**:
1. Create `src/components/large-files/LargeFileList.tsx`
2. Create `src/components/large-files/FilterControls.tsx`
3. Create `src/components/large-files/FilePreview.tsx`
4. Implement Quick Look integration via Tauri command
5. Add threshold configuration

**Verification**:
- Large files display with correct info
- Filters work by type
- Preview shows file content

**Files to create**:
- `src/components/large-files/LargeFileList.tsx`
- `src/components/large-files/FilterControls.tsx`
- `src/components/large-files/FilePreview.tsx`
- `src/pages/LargeFiles.tsx`

---

### [ ] Step: Phase 4 Integration Testing
<!-- phase: 4 -->

Test large file functionality.

**Tasks**:
1. Test scanning with various thresholds
2. Verify type categorization
3. Test preview and reveal in Finder
4. Run linting and tests

**Verification**:
- Large files found accurately
- Filters work correctly
- All tests pass

---

## Phase 5: Duplicate Finder

### [ ] Step: Duplicate Data Models
<!-- phase: 5 -->

Add data models for duplicate detection.

**Tasks**:
1. Add DuplicateScanResult, DuplicateGroup, DuplicateFile to models
2. Add corresponding TypeScript types

**Verification**:
- Models serialize/deserialize correctly

**Files to update**:
- `src-tauri/src/models/scan_result.rs`
- `src/types/index.ts`

---

### [ ] Step: Duplicate Scanner
<!-- phase: 5 -->

Implement duplicate file scanner.

**Tasks**:
1. Create `src-tauri/src/scanner/duplicate_scanner.rs`
2. Implement size-based first pass grouping
3. Implement partial hash (first/last 4KB) second pass
4. Implement full SHA-256 hash third pass
5. Mark oldest file as original
6. Implement `scan_duplicates` command

**Verification**:
- Finds actual duplicates (no false positives)
- Performance acceptable for 10K+ files
- Original correctly identified

**Files to create**:
- `src-tauri/src/scanner/duplicate_scanner.rs`

---

### [ ] Step: Duplicates UI
<!-- phase: 5 -->

Implement duplicates interface.

**Tasks**:
1. Create `src/components/duplicates/DuplicateGroups.tsx`
2. Create `src/components/duplicates/DuplicateItem.tsx`
3. Show original vs copies visually
4. Implement smart selection (keep originals)
5. Show wasted space per group

**Verification**:
- Duplicate groups display clearly
- Original marked distinctly
- Selection respects protected files

**Files to create**:
- `src/components/duplicates/DuplicateGroups.tsx`
- `src/components/duplicates/DuplicateItem.tsx`
- `src/pages/Duplicates.tsx`

---

### [ ] Step: Phase 5 Integration Testing
<!-- phase: 5 -->

Test duplicate detection functionality.

**Tasks**:
1. Create test files with known duplicates
2. Verify detection accuracy
3. Test selection and cleaning
4. Run linting and tests

**Verification**:
- No false positives
- Performance meets targets
- All tests pass

---

## Phase 6: Polish & Distribution

### [ ] Step: Permission Flow
<!-- phase: 6 -->

Implement Full Disk Access permission request.

**Tasks**:
1. Create permission check on app launch
2. Design permission request UI
3. Implement deep link to System Preferences
4. Handle partial functionality without permission
5. Show permission status in UI

**Verification**:
- Permission request is clear
- App functions with limited access
- Status indicator is accurate

---

### [ ] Step: Auto-Clean Feature
<!-- phase: 6 -->

Implement scheduled automatic cleaning.

**Tasks**:
1. Add scheduling configuration to settings
2. Implement background scheduler
3. Define trusted categories for auto-clean
4. Log auto-clean operations
5. Send notification on completion

**Verification**:
- Schedule options work correctly
- Only trusted categories cleaned
- Notifications appear

---

### [ ] Step: Menu Bar Mode
<!-- phase: 6 -->

Implement menu bar icon functionality.

**Tasks**:
1. Add menu bar icon using Tauri system tray
2. Show quick stats on click
3. Add quick clean option
4. Toggle main window visibility
5. Add option to start minimized

**Verification**:
- Menu bar icon appears
- Quick actions work
- Window toggling works

---

### [ ] Step: Cleaning History
<!-- phase: 6 -->

Implement cleaning history view.

**Tasks**:
1. Create history page with past cleanings
2. Show date, space reclaimed, items
3. Allow filtering by date range
4. Store history persistently

**Verification**:
- History displays correctly
- Filtering works
- Data persists

---

### [ ] Step: Accessibility & Polish
<!-- phase: 6 -->

Ensure accessibility and UI polish.

**Tasks**:
1. Add ARIA labels to all interactive elements
2. Test VoiceOver navigation
3. Ensure keyboard navigation works
4. Polish animations and transitions
5. Test dark mode thoroughly

**Verification**:
- VoiceOver navigates entire app
- All controls keyboard accessible
- Visual polish is complete

---

### [ ] Step: Build & Distribution
<!-- phase: 6 -->

Prepare for distribution.

**Tasks**:
1. Configure code signing
2. Set up notarization workflow
3. Create DMG installer
4. Test on macOS 12 (Intel) and 14 (ARM)
5. Create app icon set
6. Write release notes

**Verification**:
- App passes notarization
- Installs correctly from DMG
- Works on both architectures

---

### [ ] Step: Final Testing & Release
<!-- phase: 6 -->

Complete final testing and release.

**Tasks**:
1. Run full manual test checklist
2. Performance testing with large datasets
3. Fix any remaining bugs
4. Final linting pass
5. Tag release version

**Verification**:
- All manual tests pass
- Performance meets targets
- Zero critical bugs
- Code is clean

---

## Verification Commands

```bash
# Rust
cd src-tauri && cargo build
cd src-tauri && cargo test
cd src-tauri && cargo clippy -- -D warnings
cd src-tauri && cargo fmt --check

# Frontend
pnpm install
pnpm lint
pnpm typecheck
pnpm test

# Full app
pnpm tauri dev
pnpm tauri build
```
