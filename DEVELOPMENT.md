# Development Guide

This guide covers everything you need to set up and run CleanMac for development.

## Prerequisites

### Required Tools

1. **Node.js 18+** and **pnpm**
   ```bash
   # Install Node.js via Homebrew
   brew install node

   # Install pnpm
   npm install -g pnpm
   ```

2. **Rust 1.75+**
   ```bash
   # Install Rust via rustup
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

   # Restart your terminal, then verify
   rustc --version
   ```

3. **Xcode Command Line Tools** (required for macOS development)
   ```bash
   xcode-select --install
   ```

### Optional but Recommended

- [VS Code](https://code.visualstudio.com/) with extensions:
  - [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode)
  - [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
  - [Tailwind CSS IntelliSense](https://marketplace.visualstudio.com/items?itemName=bradlc.vscode-tailwindcss)
  - [ESLint](https://marketplace.visualstudio.com/items?itemName=dbaeumer.vscode-eslint)

## Getting Started

### 1. Clone the Repository

```bash
git clone https://github.com/yourusername/cleanmac.git
cd cleanmac
```

### 2. Install Dependencies

```bash
# Install frontend dependencies
pnpm install

# Rust dependencies are installed automatically on first build
```

### 3. Run Development Server

```bash
pnpm tauri dev
```

This will:
- Start the Vite dev server with hot reload
- Compile the Rust backend
- Open the application window

The first build may take a few minutes as Rust compiles all dependencies.

## Available Commands

### Frontend

| Command | Description |
|---------|-------------|
| `pnpm dev` | Start Vite dev server only (no Tauri) |
| `pnpm build` | Build frontend for production |
| `pnpm lint` | Run ESLint |
| `pnpm typecheck` | Run TypeScript type checking |

### Tauri / Full App

| Command | Description |
|---------|-------------|
| `pnpm tauri dev` | Run app in development mode |
| `pnpm tauri build` | Build production app bundle |
| `pnpm tauri build --debug` | Build with debug symbols |

### Rust Backend

```bash
cd src-tauri

# Build
cargo build

# Build release
cargo build --release

# Run tests
cargo test

# Run linter
cargo clippy -- -D warnings

# Format code
cargo fmt

# Check formatting
cargo fmt --check
```

## Project Architecture

### Frontend (React + TypeScript)

```
src/
├── main.tsx              # Application entry point
├── App.tsx               # Root component
├── components/           # Reusable UI components
│   ├── common/           # Buttons, modals, etc.
│   ├── layout/           # Sidebar, header, status bar
│   ├── dashboard/        # Dashboard widgets
│   ├── cache/            # Cache management UI
│   ├── large-files/      # Large file finder UI
│   ├── duplicates/       # Duplicate finder UI
│   ├── leftovers/        # App leftovers UI
│   └── settings/         # Settings page
├── stores/               # Zustand state stores
├── hooks/                # Custom React hooks
├── lib/                  # Utilities
│   ├── tauri.ts          # Tauri API wrappers
│   ├── format.ts         # Formatting utilities
│   └── constants.ts      # App constants
├── types/                # TypeScript type definitions
├── pages/                # Page components
└── styles/               # Global styles
```

### Backend (Rust)

```
src-tauri/src/
├── main.rs               # Application entry point
├── lib.rs                # Library exports and Tauri setup
├── commands/             # Tauri command handlers
│   ├── mod.rs
│   ├── scan.rs           # Scan commands
│   ├── clean.rs          # Clean commands
│   ├── config.rs         # Config commands
│   └── system.rs         # System info commands
├── scanner/              # File scanning logic
│   ├── mod.rs
│   ├── cache_scanner.rs
│   ├── large_file_scanner.rs
│   ├── duplicate_scanner.rs
│   └── orphan_scanner.rs
├── analyzer/             # Analysis logic
│   ├── mod.rs
│   ├── developer_detector.rs
│   ├── app_registry.rs
│   └── cache_categorizer.rs
├── cleaner/              # File deletion
│   ├── mod.rs
│   ├── safe_delete.rs
│   └── history.rs
├── models/               # Data structures
│   ├── mod.rs
│   ├── file_entry.rs
│   ├── scan_result.rs
│   ├── config.rs
│   └── history.rs
└── utils/                # Utilities
    ├── mod.rs
    ├── fs.rs
    ├── hash.rs
    └── permissions.rs
```

## Development Workflow

### Adding a New Tauri Command

1. **Define the command in Rust** (`src-tauri/src/commands/`)
   ```rust
   #[tauri::command]
   pub async fn my_command(param: String) -> Result<String, String> {
       Ok(format!("Hello, {}", param))
   }
   ```

2. **Register in lib.rs**
   ```rust
   .invoke_handler(tauri::generate_handler![
       my_command,
       // ... other commands
   ])
   ```

3. **Create TypeScript wrapper** (`src/lib/tauri.ts`)
   ```typescript
   export const myCommand = (param: string) =>
     invoke<string>('my_command', { param });
   ```

4. **Use in React component**
   ```typescript
   const result = await myCommand("World");
   ```

### Adding a New Component

1. Create component file in appropriate directory
2. Export from index if creating a module
3. Follow existing patterns for styling (Tailwind)
4. Add TypeScript types as needed

### State Management

We use Zustand for state management. Stores are in `src/stores/`:

```typescript
// Example store
import { create } from 'zustand';

interface MyStore {
  value: string;
  setValue: (value: string) => void;
}

export const useMyStore = create<MyStore>((set) => ({
  value: '',
  setValue: (value) => set({ value }),
}));
```

## Testing

### Frontend Tests

```bash
pnpm test
```

### Rust Tests

```bash
cd src-tauri
cargo test
```

### Manual Testing Checklist

- [ ] App launches correctly
- [ ] Sidebar navigation works
- [ ] Cache scan finds items
- [ ] Clean operation moves files to Trash
- [ ] Settings persist across restarts
- [ ] Dark mode works
- [ ] Developer mode protects caches

## Debugging

### Frontend

- Use browser DevTools (opens automatically in dev mode)
- React DevTools extension
- Console logging

### Rust Backend

- Use `println!` or `log` crate for debugging
- Check terminal output where `pnpm tauri dev` is running
- Use `cargo test` for isolated testing

### Common Issues

**Rust compilation errors:**
```bash
# Clean and rebuild
cd src-tauri
cargo clean
cargo build
```

**Frontend not updating:**
```bash
# Clear Vite cache
rm -rf node_modules/.vite
pnpm dev
```

**Permission issues on macOS:**
- Grant Full Disk Access in System Preferences > Security & Privacy > Privacy

## Building for Production

### Build the App

```bash
pnpm tauri build
```

Output will be in `src-tauri/target/release/bundle/`:
- `.app` - macOS application bundle
- `.dmg` - Disk image for distribution

### Code Signing (for distribution)

1. Obtain an Apple Developer certificate
2. Configure signing in `src-tauri/tauri.conf.json`
3. Build with signing:
   ```bash
   pnpm tauri build
   ```

### Notarization

For distribution outside the App Store, notarization is required:

```bash
# After building
xcrun notarytool submit path/to/app.dmg --apple-id YOUR_ID --team-id TEAM_ID --password APP_SPECIFIC_PASSWORD
```

## Code Style

### TypeScript/React

- ESLint configuration in `eslint.config.js`
- Run `pnpm lint` before committing
- Use functional components with hooks
- Prefer TypeScript strict mode

### Rust

- Follow Rust standard style
- Run `cargo fmt` before committing
- Run `cargo clippy` for lints
- Document public APIs

## Resources

- [Tauri Documentation](https://tauri.app/v2/guides/)
- [React Documentation](https://react.dev/)
- [Rust Book](https://doc.rust-lang.org/book/)
- [Tailwind CSS Documentation](https://tailwindcss.com/docs)
- [Zustand Documentation](https://github.com/pmndrs/zustand)
