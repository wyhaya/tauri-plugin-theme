#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri_plugin_theme_v1::{ThemePlugin, get_theme, set_theme, Theme};
use tauri::{CustomMenuItem, Manager, Menu, Submenu};

fn main() {
    let mut menu = Menu::new();    
    let theme_auto = CustomMenuItem::new("theme_auto".to_string(), "Auto");
    let theme_light = CustomMenuItem::new("theme_light".to_string(), "Light");
    let theme_dark = CustomMenuItem::new("theme_dark".to_string(), "Dark");
    let theme_get = CustomMenuItem::new("theme_get".to_string(), "Get");
    menu = menu
    .add_submenu(Submenu::new(
        "Theme",
        Menu::new()
            .add_item(theme_auto)
            .add_item(theme_light)
            .add_item(theme_dark)
            .add_item(theme_get),
    ));
    let mut ctx = tauri::generate_context!();
    tauri::Builder::default()
        .menu(menu)
        .on_menu_event(|event| match event.menu_item_id() {
            "theme_auto" => {
                let _ = set_theme(event.window().app_handle(), Theme::Auto);
            }
            "theme_light" => {
                let _ = set_theme(event.window().app_handle(), Theme::Light);
            }
            "theme_dark" => {
                let _ = set_theme(event.window().app_handle(), Theme::Dark);
            }
            "theme_get" => {
                let theme = get_theme(event.window().app_handle());
                println!("{:#?}", theme);
            }
            _ => {}
        })
        .plugin(ThemePlugin::init(ctx.config_mut()))
        .run(ctx)
        .expect("error while running tauri application");
}
