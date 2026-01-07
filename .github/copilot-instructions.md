# OTP Bar - Copilot Instructions

## Project Overview

OTP Bar is a macOS menu bar application for managing one-time passwords (OTP). It's built using Tauri with a React/TypeScript frontend and Rust backend.

## Technology Stack

- **Frontend**: React 19.2.3 with TypeScript
- **Backend**: Rust with Tauri 2.x
- **Build Tool**: Vite 7.3.0
- **Package Manager**: pnpm 10.x
- **Runtime**: Node.js 24.x
- **Target Platform**: macOS (primary), menu bar application

## Project Structure

- `src/` - TypeScript/React frontend source code
- `src-tauri/` - Rust backend source code
- `src-tauri/src/lib.rs` - Main Tauri library setup with plugins
- `src-tauri/src/main.rs` - Entry point
- `.github/workflows/` - CI/CD workflows
- `assets/` - Application assets

## Build and Development Commands

```bash
# Install dependencies
pnpm install

# Run in development mode
pnpm run dev

# Build for production
pnpm run build

# Build Tauri application
pnpm run tauri build
```

## TypeScript Configuration

- **Target**: ES2020
- **Strict mode**: Enabled
- **Module system**: ESNext with bundler resolution
- **Linting**: Strict TypeScript rules enabled
  - `noUnusedLocals: true`
  - `noUnusedParameters: true`
  - `noFallthroughCasesInSwitch: true`

## Coding Conventions

### TypeScript/React

- Use TypeScript strict mode
- Prefer async/await over promise chains
- Use ES2020+ features
- Follow the existing code structure in `src/tray.ts` for UI components
- Use Tauri APIs for native functionality (filesystem, clipboard, dialog, shell)

### Rust

- Follow Rust 2021 edition conventions
- Use serde for JSON serialization
- Configure Tauri plugins in `lib.rs` setup function
- Set activation policy to `Accessory` for menu bar mode (hides dock icon)

## Platform-Specific Notes

### macOS

- Application runs as a menu bar app (not in dock)
- Requires `ActivationPolicy::Accessory` for proper menu bar behavior
- Uses system tray icon from `@tauri-apps/api/tray`
- Configuration stored in `$HOME/.config/otp-bar/`

## Dependencies and Plugins

### Tauri Plugins in Use

- `tauri-plugin-shell` - Shell command execution
- `tauri-plugin-clipboard-manager` - Clipboard operations
- `tauri-plugin-fs` - File system access
- `tauri-plugin-dialog` - Native dialogs
- `tauri-plugin-process` - Process management

### Key Frontend Dependencies

- `jsqr` - QR code parsing
- React and React DOM for UI

## Application Features

- OTP token management via menu bar
- QR code scanning for token import (PNG images)
- Clipboard integration for copying OTP codes
- Configuration file at `$HOME/.config/otp-bar/config.json`
- Token files stored as plain text in `$HOME/.config/otp-bar/`

## Development Guidelines

1. **Minimize Changes**: Make surgical, focused changes
2. **Follow Existing Patterns**: Match the style and structure of existing code
3. **Platform Awareness**: Remember this is primarily a macOS application
4. **Config Handling**: Token and configuration files are stored in user's home directory
5. **No Tests Currently**: The project does not have a test suite, so manual testing is required

## Build Pipeline

- CI/CD configured in `.github/workflows/build.yaml`
- Builds on macOS runners
- Uses Tauri action for creating releases
- Targets macOS latest (Apple Silicon)

## File Naming and Organization

- TypeScript files use `.ts` extension
- React components would use `.tsx` (if applicable)
- Rust files follow standard Rust conventions
- Configuration files at root level (vite.config.ts, tsconfig.json, etc.)

## Common Tasks

### Adding a New Tauri Plugin

1. Add dependency to `src-tauri/Cargo.toml`
2. Initialize plugin in `src-tauri/src/lib.rs` builder chain
3. Add corresponding npm package to `package.json` if needed

### Working with OTP Tokens

- Token files are plain text, one per file
- File names become the display name in the menu
- Tokens are passed to `oathtool` executable for OTP generation

### Modifying the Menu

- See `src/tray.ts` for menu structure
- Menu items are created dynamically based on token files
- Use `MenuItem.new()` for custom menu items
- Use `PredefinedMenuItem` for system items (Quit, etc.)
