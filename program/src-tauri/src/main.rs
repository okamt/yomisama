// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{path::Path, sync::RwLock};

use program::{
    commands::*,
    settings::{SettingsFile, DEFAULT_CONFIG_FILE_PATH},
    state::AppState,
    tray::{handle_system_tray_event, make_system_tray},
    windows::spawn_first_time_setup_window,
};
use tauri::Manager;

fn main() {
    std::panic::set_hook(Box::new(|panic_info| {
        tauri::api::dialog::blocking::message(
            None::<&tauri::Window>,
            "Yomisama - Error",
            format!("{}", panic_info),
        );
    }));

    let app_state = AppState::default();

    if Path::new(DEFAULT_CONFIG_FILE_PATH.as_str()).exists() {
        let settings_file = SettingsFile::read_from_file(DEFAULT_CONFIG_FILE_PATH.as_str())
            .expect("could not read default config file");
        app_state.settings.get_or_init(|| {
            RwLock::new(
                settings_file
                    .into_settings()
                    .expect("could not read config file"),
            )
        });
    }

    let app = tauri::Builder::default()
        .system_tray(make_system_tray())
        .on_system_tray_event(handle_system_tray_event)
        .manage(app_state)
        .invoke_handler(tauri::generate_handler![
            set_config_dir,
            program::windows::window_loaded,
            program::windows::window_unloading
        ])
        .build(tauri::generate_context!())
        .expect("error while building tauri application");

    {
        let state = app.state::<AppState>();

        if state.settings.get().is_none() {
            spawn_first_time_setup_window(&app).expect("could not spawn first time setup window");
        }
    }

    app.run(|_app, event| match event {
        tauri::RunEvent::ExitRequested { api, .. } => {
            api.prevent_exit();
        }
        _ => {}
    });
}
