#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri_plugin_theme::ThemePlugin;

fn main() {
    tauri::Builder::default()
        .plugin(ThemePlugin::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
