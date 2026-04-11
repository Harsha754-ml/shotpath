# ShotPath 📸

ShotPath is a lightweight Windows utility that brings a "missing" macOS feature to Windows: **automatic clipboard synchronization for folder-based screenshots.**

## 🚀 The Backstory: Mac vs. Windows

On macOS, you can easily configure screenshots to go straight to your clipboard or a folder—and often both. Windows users, however, are usually stuck: either you use `Win + Shift + S` (clipboard only, no file) or `Win + PrtSc` (file only, no clipboard). 

If you use professional capture tools, game-specific screenshotters, or specialized software, they almost always save to a folder. ShotPath bridges this gap. It watches your folders and instantly pushes those new files to your clipboard.

**It's the "Best of Both Worlds" feature Windows was missing.**

## ✨ Features

- **macOS Style Workflow:** Save the file *and* have it ready to paste immediately.
- **Native Folder Monitoring:** Real-time watching of any directory using Rust's `notify` engine.
- **Dual Modes:**
    - **Image Mode:** Copies the actual image data (perfect for Discord, Slack, or Word).
    - **Path Mode:** Copies the file path (perfect for developers and CLI workflows).
- **Smart Detection:** Automatically filters for images and waits for the OS to finish writing the file before copying.
- **System Tray Integration:** Runs quietly in the background, just like a native system utility.

## 🛠️ Built With

- **Backend:** Rust (Tauri) - for extreme performance and safety.
- **Frontend:** React + Vite - for a clean, modern configuration UI.
- **API:** Tauri Native Dialogs & Notifications.

## 📦 Installation

```bash
# Install dependencies
npm install

# Run in development mode
npm run tauri dev

# Build for production
npm run tauri build
```

## 📂 How it Works

1. **Watch:** The Rust backend starts a background thread monitoring your chosen folder.
2. **Detect:** When a new `.png`, `.jpg`, or `.webp` is created, ShotPath triggers.
3. **Sync:** The app handles the file lock, processes the image, and updates your system clipboard.
4. **Notify:** A native Windows notification confirms the sync is complete.

---
*Windows didn't have it. Mac did. Now Windows has it too.*
