#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod config;
mod utils;
mod clipboard;
mod watcher;
mod tray;

#[tauri::command]
fn get_config() -> config::Config {
    config::load_config()
}

#[tauri::command]
fn update_config(config: config::Config) {
    let old_config = config::load_config();
    let new_folder = config.folder_path.clone();
    
    config::save_config(&config);

    // If folder changed or enabled state changed, restart the watcher
    if config.enabled && (new_folder != old_config.folder_path || !old_config.enabled) {
        println!("Restarting watcher for: {}", new_folder);
        watcher::start_watching(new_folder);
    }
}

fn main() {
    let initial_config = config::load_config();
    if initial_config.enabled && !initial_config.folder_path.is_empty() {
        watcher::start_watching(initial_config.folder_path);
    }

    tauri::Builder::default()
        .system_tray(tray::create_tray())
        .on_system_tray_event(|app, event| tray::handle_tray_event(app, event))
        .invoke_handler(tauri::generate_handler![get_config, update_config])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
