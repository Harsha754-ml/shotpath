# ShotPath 📸

ShotPath is a lightweight Windows utility that automatically monitors your folders for new images and copies them to your clipboard.

## 🚀 Why ShotPath?

Windows lacks a native feature to automatically copy new images from a specific folder to the clipboard. Whether you're using a game's built-in screenshot tool or a custom capture software that only saves to disk, ShotPath bridges the gap by making those images instantly available for pasting.

## ✨ Features

- **Folder Monitoring:** Watches any directory you choose for new image files.
- **Instant Clipboard:** Copies the image (or its path) as soon as it's created or modified.
- **Toggled Activity:** Easily enable or disable the watcher from the system tray.
- **Customizable Modes:** Choose between copying the actual image data or just the file path.
- **Lightweight:** Built with Tauri and Rust for minimal system resource usage.

## 🛠️ Built With

- **Backend:** Rust (Tauri)
- **Frontend:** React (Vite)
- **Watcher:** `notify` (cross-platform filesystem notifications)

## 📦 Installation

To run this project locally, you'll need:
- [Rust](https://www.rust-lang.org/tools/install)
- [Node.js](https://nodejs.org/)

```bash
# Install dependencies
npm install

# Run in development mode
npm run tauri dev

# Build for production
npm run tauri build
```

## 📂 Structure

- `src-tauri/`: Rust backend logic (watcher, clipboard, tray)
- `src/`: React frontend UI
- `public/`: Static assets and icons

---
*Created because Windows didn't have this feature, so I made it myself.*
