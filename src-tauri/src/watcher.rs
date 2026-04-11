use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::Path;
use std::sync::mpsc::channel;
use crate::utils;
use crate::clipboard;
use crate::config;

pub fn start_watching(folder: String) {
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
                                
                                // Re-load config to get current mode
                                let current_config = config::load_config();
                                if current_config.copy_mode == "image" {
                                    clipboard::copy_image_to_clipboard(&path);
                                    println!("Image copied to clipboard!");
                                } else {
                                    clipboard::copy_path_to_clipboard(&path);
                                    println!("Path copied to clipboard!");
                                }
                            }
                        }
                    }
                }
                Err(e) => println!("watch error: {:?}", e),
            }
        }
    });
}
