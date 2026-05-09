use tauri::{
    Manager,
    menu::{Menu, MenuItem},
    tray::{TrayIcon, TrayIconBuilder, TrayIconEvent, MouseButton, MouseButtonState},
};

pub fn create_tray(app: &tauri::App) -> Result<TrayIcon, tauri::Error> {
    let open_item = MenuItem::with_id(app, "open", "Open UI", true, None::<&str>)?;
    let quit_item = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
    let menu = Menu::with_items(app, &[&open_item, &quit_item])?;

    TrayIconBuilder::new()
        .menu(&menu)
        .tooltip("ShotPath")
        .on_menu_event(|app, event| {
            match event.id.as_ref() {
                "open" => {
                    if let Some(window) = app.get_webview_window("main") {
                        let _ = window.show();
                        let _ = window.set_focus();
                    }
                }
                "quit" => {
                    std::process::exit(0);
                }
                _ => {}
            }
        })
        .on_tray_icon_event(|tray, event| {
            if let TrayIconEvent::Click { button: MouseButton::Left, button_state: MouseButtonState::Up, .. } = event {
                let app = tray.app_handle();
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.show();
                    let _ = window.set_focus();
                }
            }
        })
        .build(app)
}
