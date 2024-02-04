#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    let mut ctx = tauri::generate_context!();
    tauri::Builder::default()
        .plugin(tauri_plugin_theme::init(ctx.config_mut()))
        .plugin(tauri_plugin_dialog::init())
        .run(ctx)
        .expect("error while running tauri application");
}
