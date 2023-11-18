#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri_plugin_theme::ThemePlugin;

fn main() {
    let mut ctx = tauri::generate_context!();
    tauri::Builder::default()
        .plugin(ThemePlugin::init(ctx.config_mut()))
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .run(ctx)
        .expect("error while running tauri application");
}
