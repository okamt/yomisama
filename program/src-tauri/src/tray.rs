use tauri::{
    AppHandle, CustomMenuItem, Runtime, SystemTray, SystemTrayEvent, SystemTrayMenu,
    SystemTrayMenuItem,
};

pub fn make_system_tray() -> SystemTray {
    let settings = CustomMenuItem::new("settings".to_owned(), "Settings").disabled();
    let quit = CustomMenuItem::new("quit".to_owned(), "Quit");

    let tray_menu = SystemTrayMenu::new()
        .add_item(settings)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(quit);

    let tray = SystemTray::new().with_menu(tray_menu);

    tray
}

pub fn set_settings_enabled(app: &AppHandle<impl Runtime>, enabled: bool) -> tauri::Result<()> {
    app.tray_handle().get_item("settings").set_enabled(enabled)
}

pub fn handle_system_tray_event(_app: &AppHandle<impl Runtime>, event: SystemTrayEvent) {
    match event {
        SystemTrayEvent::LeftClick {
            position: _,
            size: _,
            ..
        } => {
            println!("left click event");
        }
        SystemTrayEvent::RightClick {
            position: _,
            size: _,
            ..
        } => {
            println!("right click event");
        }
        SystemTrayEvent::DoubleClick {
            position: _,
            size: _,
            ..
        } => {
            println!("double click event");
        }
        SystemTrayEvent::MenuItemClick { id, .. } => {
            match id.as_str() {
                "settings" => {
                    // open settings menu
                }
                "quit" => {
                    std::process::exit(0);
                }
                _ => {}
            }
        }
        _ => {}
    }
}
