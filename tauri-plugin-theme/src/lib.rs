use serde::Deserialize;
use tauri::generate_handler;
use tauri::plugin::{Builder, TauriPlugin};
use tauri::Runtime;

mod platform;
use platform::set_theme;

#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Theme {
    Auto,
    Light,
    Dark,
}

pub struct ThemePlugin;

impl ThemePlugin {
    pub fn init<R: Runtime>() -> TauriPlugin<R, Option<()>> {
        Builder::new("theme")
            .invoke_handler(generate_handler![set_theme])
            .build()
    }
}
