# CleanMac

CleanMac is a native macOS disk cleanup and optimization utility built with Tauri and React. It helps users reclaim disk space by identifying and removing unnecessary files while protecting important data, specifically designed to be developer-aware.

## Tech Stack

**Frontend:**
*   **Framework:** React 19
*   **Language:** TypeScript 5.x
*   **Build Tool:** Vite 7.x
*   **Styling:** Tailwind CSS 3.x
*   **State Management:** Zustand
*   **Routing:** React Router 7
*   **Testing:** Vitest, React Testing Library

**Backend:**
*   **Framework:** Tauri 2.x
*   **Language:** Rust 1.75+
*   **Key Crates:** `tokio` (async), `walkdir` (fs traversal), `rayon` (parallelism), `trash` (safe deletion), `plist` (macOS config)

## Architecture

*   **`src/` (Frontend):** Contains the React application.
    *   `components/`: Reusable UI components organized by feature (dashboard, cache, duplicates, etc.).
    *   `stores/`: Zustand state management stores.
    *   `hooks/`: Custom React hooks.
    *   `lib/`: Utilities and Tauri API wrappers (`tauri.ts`).
    *   `types/`: TypeScript definitions.
*   **`src-tauri/` (Backend):** Contains the Rust application.
    *   `src/commands/`: Tauri command handlers (scan, clean, config).
    *   `src/scanner/`: Logic for scanning files (cache, large files, duplicates).
    *   `src/cleaner/`: Logic for safe file deletion/moving to trash.
    *   `src/analyzer/`: Analysis logic (developer detector, cache categorizer).
    *   `tauri.conf.json`: Tauri configuration.

## Key Commands

### Development
*   **Start App (Dev Mode):** `pnpm tauri dev` (Starts Vite server + compiles Rust + opens app)
*   **Start Frontend Only:** `pnpm dev`
*   **Lint Frontend:** `pnpm lint`
*   **Typecheck Frontend:** `pnpm typecheck`
*   **Lint Backend:** `cd src-tauri && cargo clippy -- -D warnings`
*   **Format Backend:** `cd src-tauri && cargo fmt`

### Testing
*   **Frontend Tests:** `pnpm test` (Vitest)
*   **Backend Tests:** `cd src-tauri && cargo test`
*   **Full Quality Check:** `pnpm quality` (Runs typecheck, lint, and test)

### Build
*   **Build Production App:** `pnpm tauri build` (Outputs to `src-tauri/target/release/bundle/`)
*   **Build Frontend Only:** `pnpm build`

## Development Conventions

*   **State Management:** Use Zustand for global state. Stores are located in `src/stores/`.
*   **Tauri Commands:**
    1.  Define the command in `src-tauri/src/commands/`.
    2.  Register the command in `src-tauri/src/lib.rs`.
    3.  Create a TypeScript wrapper in `src/lib/tauri.ts`.
*   **Styling:** Use Tailwind CSS utility classes.
*   **Security:**
    *   Operations should be 100% local.
    *   Files should be moved to Trash by default, not permanently deleted immediately.
    *   The app requires Full Disk Access.
*   **Code Style:**
    *   Follow `eslint` rules for TypeScript/React.
    *   Follow `cargo fmt` and `clippy` for Rust.

## Current Status

The project is in **Phase 1: Foundation (MVP)**.

*   **Completed Steps:**
    *   Requirements & Spec creation.
    *   Project Setup (Tauri + React structure initialized).
    *   Rust Data Models (`src-tauri/src/models/`).
    *   TypeScript Types (`src/types/index.ts`).
    *   Rust Utilities (`src-tauri/src/utils/`).

*   **Next Steps:**
    *   Implement Cache Scanner (`src-tauri/src/scanner/`).
    *   Implement Tauri Commands (`src-tauri/src/commands/`).
    *   Implement Cleaner (`src-tauri/src/cleaner/`).
    *   Frontend integration (State management, Components, Pages).

**Immediate Task:**
The next unchecked item in `.zenflow/tasks/init-d417/plan.md` is **"Step: Cache Scanner Implementation"**.
