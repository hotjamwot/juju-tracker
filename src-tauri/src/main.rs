// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod tray;

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            tray::create_tray(app.handle().clone());
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running Tauri app");
}
