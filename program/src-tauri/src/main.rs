// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use program::{commands::*, windows::spawn_first_time_setup_window};

fn main() {
    let app = tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            set_config_dir,
            program::windows::window_loaded,
            program::windows::window_unloading
        ])
        .build(tauri::generate_context!())
        .expect("error while building tauri application");

    spawn_first_time_setup_window(&app).expect("could not spawn first time setup window");

    app.run(|_, _| {});
}
