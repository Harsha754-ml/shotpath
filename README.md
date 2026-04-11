# ShotPath

ShotPath is a lightweight utility for Windows that implements a feature commonly found on macOS: automatic clipboard synchronization for screenshot folders.

## The Problem
On Windows, native screenshot tools are fragmented. You either capture to the clipboard (Win + Shift + S) without saving a file, or you save a file (Win + PrtSc) without it being added to the clipboard. Users of third-party capture software, game-specific screenshotters, or professional tools often face the same issue: images are saved to disk, but sharing them requires manually opening a folder and copying the file.

## The Solution
ShotPath bridges this gap by monitoring a designated folder in real-time. When a new image is detected, it is immediately copied to your clipboard, making it available for instant pasting into applications like Discord, Slack, or document editors.

## Core Features
- **Real-time Folder Monitoring:** Utilizes a high-performance Rust-based watcher to detect new files instantly.
- **Dual Copy Modes:** Choose between copying the actual image data (for sharing) or the absolute file path (for development/CLI workflows).
- **Native System Integration:** Runs as a background process with a system tray icon for quick access.
- **File Integrity Handling:** Intelligent retry logic ensures the OS has finished writing the file before the clipboard is updated.
- **Desktop Notifications:** Provides immediate confirmation when a synchronization occurs.

## Technical Architecture
- **Backend:** Rust (Tauri) for minimal resource overhead and native performance.
- **Frontend:** React and Vite for a clean, responsive configuration interface.
- **APIs:** Leverages Tauri's native dialogs, notifications, and filesystem watchers.

## Getting Started
To run or build this project locally, ensure you have the Rust toolchain and Node.js installed.

```bash
# Install dependencies
npm install

# Run in development mode
npm run tauri dev

# Build for production
npm run tauri build
```

## How It Works
1. A background thread monitors your selected directory for image creation or modification events.
2. Upon detection, ShotPath confirms the file is ready for reading (handling OS-level file locks).
3. Based on your configuration, the app copies the image data or its path to the system clipboard.
4. A system notification confirms the action is complete.

---
*Bringing macOS-style screenshot workflows to the Windows desktop.*
