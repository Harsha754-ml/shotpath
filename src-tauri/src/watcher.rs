use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::Path;
use std::sync::{Mutex, OnceLock};
use std::sync::mpsc::channel;
use tauri::AppHandle;
use tauri_plugin_notification::NotificationExt;

static WATCHER: OnceLock<Mutex<Option<RecommendedWatcher>>> = OnceLock::new();

fn watcher_slot() -> &'static Mutex<Option<RecommendedWatcher>> {
    WATCHER.get_or_init(|| Mutex::new(None))
}

pub fn start_watching(app: AppHandle, folder: String) {
    if folder.is_empty() {
        stop_watching();
        return;
    }

    stop_watching();

    let (tx, rx) = channel();
    let mut watcher = match RecommendedWatcher::new(tx, Config::default()) {
        Ok(w) => w,
        Err(e) => {
            println!("Failed to create watcher: {:?}", e);
            return;
        }
    };

    if let Err(e) = watcher.watch(Path::new(&folder), RecursiveMode::NonRecursive) {
        println!("Watcher error: could not watch folder {}: {:?}", folder, e);
        return;
    }

    *watcher_slot().lock().unwrap() = Some(watcher);

    std::thread::spawn(move || {
        println!("Watcher started on: {}", folder);
        for res in rx {
            match res {
                Ok(event) => {
                    if event.kind.is_create() || event.kind.is_modify() {
                        for path in event.paths {
                            if crate::utils::is_image(&path) {
                                println!("New image detected: {:?}", path);
                                crate::utils::wait_for_file_ready(&path);

                                crate::clipboard::copy_to_clipboard(&path);
                                println!("Image & Path copied to clipboard!");

                                let _ = app
                                    .notification()
                                    .builder()
                                    .title("ShotPath")
                                    .body(format!(
                                        "Ready to paste: {:?}",
                                        path.file_name().unwrap_or_default()
                                    ))
                                    .show();
                            }
                        }
                    }
                }
                Err(e) => println!("watch error: {:?}", e),
            }
        }
        println!("Watcher thread exited.");
    });
}

pub fn stop_watching() {
    if let Ok(mut guard) = watcher_slot().lock() {
        if guard.take().is_some() {
            println!("Watcher stopped.");
        }
    }
}
