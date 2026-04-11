# ShotPath

ShotPath is a lightweight utility for Windows that implements a feature commonly found on macOS: automatic clipboard synchronization for screenshot folders.

## The Problem
On Windows, native screenshot tools are fragmented. You either capture to the clipboard (Win + Shift + S) without saving a file, or you save a file (Win + PrtSc) without it being added to the clipboard. Sharing images saved to disk usually requires manually opening a folder and copying the file.

## The Solution
ShotPath bridges this gap by monitoring a designated folder in real-time. When a new image is detected, it is immediately copied to your clipboard in multiple formats simultaneously. This allows the receiving application to "auto-detect" the best version:
- **Discord, Slack, Word:** Will paste the actual image.
- **Notepad, VS Code, Terminals:** Will paste the file path.
- **File Explorer:** Will allow you to "paste" (copy) the file itself into another folder.

## Core Features
- **Smart Multi-Format Clipboard:** Sets Image data (DIB), Unicode Text (Path), and File Drop lists at once.
- **Real-time Folder Monitoring:** Utilizes a high-performance Rust-based watcher to detect new files instantly.
- **Native System Integration:** Runs as a background process with a system tray icon for quick access.
- **File Integrity Handling:** Intelligent retry logic ensures the OS has finished writing the file before the clipboard is updated.
- **Desktop Notifications:** Provides immediate confirmation when a synchronization occurs.

## Technical Architecture
- **Backend:** Rust (Tauri) for minimal resource overhead and native performance.
- **Frontend:** React and Vite for a clean, responsive configuration interface.
- **APIs:** Leverages Tauri's native dialogs and notifications, with `clipboard-win` for advanced clipboard control.

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

---
*Bringing smarter, macOS-style screenshot workflows to the Windows desktop.*
