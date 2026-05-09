#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod config;
mod utils;
mod clipboard;
mod watcher;
mod tray;

use tauri::Manager;

#[tauri::command]
fn get_config() -> config::Config {
    config::load_config()
}

#[tauri::command]
fn update_config(app: tauri::AppHandle, config: config::Config) {
    let old_config = config::load_config();
    let new_folder = config.folder_path.clone();

    config::save_config(&config);

    if config.enabled && (new_folder != old_config.folder_path || !old_config.enabled) {
        println!("Restarting watcher for: {}", new_folder);
        watcher::start_watching(app.clone(), new_folder);
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let builder = tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_notification::init())
        .setup(|app| {
            let initial_config = config::load_config();
            if initial_config.enabled && !initial_config.folder_path.is_empty() {
                let handle = app.handle().clone();
                std::thread::spawn(move || {
                    watcher::start_watching(handle, initial_config.folder_path);
                });
            }

            let _ = tray::create_tray(app)?;
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![get_config, update_config]);

    builder
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
