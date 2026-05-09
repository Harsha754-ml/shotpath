# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

ShotPath is a Windows desktop utility that monitors a screenshot folder and automatically copies new images to the clipboard in multiple formats (image data + file path), enabling smart paste behavior across different applications.

## Commands

```bash
npm install           # Install dependencies
npm run tauri dev     # Run in development mode (hot-reload frontend + Rust)
npm run tauri build   # Build for production
npm run dev           # Run only frontend (Vite dev server)
npm run build         # Build only frontend
```

## Architecture

**Tauri 1.x** hybrid application — Rust backend with React/Vite frontend.

### Frontend (`src/`)
- `App.jsx` — Main UI. Calls Tauri commands `get_config` and `update_config` via `@tauri-apps/api/tauri`
- `components/` — React components for folder selection, toggles, status indicators
- `styles/main.css` — Global styles

### Backend (`src-tauri/src/`)
- `main.rs` — Entry point. Registers Tauri commands and system tray. Initializes watcher on startup if enabled.
- `config.rs` — Loads/saves JSON config to `%APPDATA%/shotpath/config.json`. Exposes `Config` struct with `folder_path`, `enabled`, `copy_mode`.
- `watcher.rs` — Spawns a thread using the `notify` crate to watch the screenshot folder. On new image file creation/modification, waits for file readiness then triggers clipboard copy + shows notification.
- `clipboard.rs` — Uses `clipboard-win` to set three clipboard formats simultaneously: Unicode text (path), FileDrop list, and CF_DIB (raw BMP image data).
- `utils.rs` — Helpers: `is_image()` filters for png/jpg/jpeg/webp. `wait_for_file_ready()` retries file access to handle write-in-progress.
- `tray.rs` — System tray with "Open UI" and "Quit" menu items.

### Tauri Commands
Frontend communicates with Rust via two commands:
- `get_config()` → returns `Config` struct
- `update_config({ config })` → saves config, restarts watcher if folder/enabled changed

### Config Schema
```rust
struct Config {
    folder_path: String,
    enabled: bool,
    copy_mode: String, // "path" or "image" (copy_mode currently unused in Rust backend)
}
```

## Key Implementation Notes

- The watcher runs in a spawned thread (`std::thread::spawn`). It is not stopped when the window is closed — the app is a background process.
- `clipboard.rs` opens the clipboard once and writes three formats in a single block for atomicity.
- `notify` 6.1.1 is used for filesystem events. Supported image extensions: `png`, `jpg`, `jpeg`, `webp`.
- Config is stored at `%APPDATA%/shotpath/config.json` (resolved via Tauri's `api::path::config_dir`).
- `tauri::api::notification` is used for desktop notifications with identifier `com.shotpath.dev`.
