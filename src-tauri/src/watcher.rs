use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::Path;
use std::sync::mpsc::channel;
use crate::utils;
use crate::clipboard;
use tauri::AppHandle;
use tauri_plugin_notification::NotificationExt;

pub fn start_watching(app: AppHandle, folder: String) {
    if folder.is_empty() { return; }

    std::thread::spawn(move || {
        let (tx, rx) = channel();
        let mut watcher = RecommendedWatcher::new(tx, Config::default()).expect("Failed to create watcher");

        if let Err(e) = watcher.watch(Path::new(&folder), RecursiveMode::NonRecursive) {
            println!("Watcher error: could not watch folder {}: {:?}", folder, e);
            return;
        }

        println!("Watcher started on: {}", folder);

        for res in rx {
            match res {
                Ok(event) => {
                    if event.kind.is_create() || event.kind.is_modify() {
                        for path in event.paths {
                            if utils::is_image(&path) {
                                println!("New image detected: {:?}", path);
                                utils::wait_for_file_ready(&path);

                                clipboard::copy_to_clipboard(&path);
                                println!("Image & Path copied to clipboard!");

                                let _ = app.notification()
                                    .builder()
                                    .title("ShotPath")
                                    .body(format!("Ready to paste: {:?}", path.file_name().unwrap_or_default()))
                                    .show();
                            }
                        }
                    }
                }
                Err(e) => println!("watch error: {:?}", e),
            }
        }
    });
}
