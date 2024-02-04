mod platform;

use platform::set_theme;
use serde::{Deserialize, Serialize};
use std::fs;
use tauri::plugin::{Builder, TauriPlugin};
use tauri::{command, generate_handler, AppHandle, Config, Manager, Runtime};

const PLUGIN_NAME: &str = "theme";
const CONFIG_FILENAME: &str = "tauri-plugin-theme";
const ERROR_MESSAGE: &str = "Get app config dir failed";

#[cfg(target_os = "macos")]
pub fn init<R: Runtime>(_config: &mut Config) -> TauriPlugin<R> {
    Builder::new(PLUGIN_NAME)
        .invoke_handler(generate_handler![get_theme, set_theme])
        .on_event(|app, e| {
            if let tauri::RunEvent::Ready = e {
                let theme = saved_theme_value(&app);
                let _ = set_theme(app.clone(), theme);
            }
        })
        .build()
}

#[cfg(target_os = "linux")]
pub fn init<R: Runtime>(_config: &mut Config) -> TauriPlugin<R> {
    Builder::new(PLUGIN_NAME)
        .invoke_handler(generate_handler![get_theme, set_theme])
        .setup(|app, _| {
            let theme = saved_theme_value(&app);
            let _ = set_theme(app.clone(), theme);
            Ok(())
        })
        .build()
}

#[cfg(target_os = "windows")]
pub fn init<R: Runtime>(config: &mut Config) -> TauriPlugin<R> {
    let theme = saved_theme_value_from_config(&config);
    for window in &mut config.tauri.windows {
        match theme {
            Theme::Auto => window.theme = None,
            Theme::Light => window.theme = Some(tauri::Theme::Light),
            Theme::Dark => window.theme = Some(tauri::Theme::Dark),
        }
    }
    Builder::new(PLUGIN_NAME)
        .invoke_handler(generate_handler![get_theme, set_theme])
        .build()
}

#[command]
fn get_theme<R: Runtime>(app: AppHandle<R>) -> Result<Theme, ()> {
    let theme = saved_theme_value(&app);
    Ok(theme)
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Theme {
    Auto,
    Light,
    Dark,
}

impl From<String> for Theme {
    fn from(value: String) -> Self {
        match value.as_str() {
            "light" => Theme::Light,
            "dark" => Theme::Dark,
            _ => Theme::Auto,
        }
    }
}

impl ToString for Theme {
    fn to_string(&self) -> String {
        match self {
            Theme::Auto => "auto".into(),
            Theme::Light => "light".into(),
            Theme::Dark => "dark".into(),
        }
    }
}

#[cfg(target_os = "windows")]
fn saved_theme_value_from_config(config: &Config) -> Theme {
    if let Some(dir) = dirs_next::config_dir() {
        let p = dir
            .join(&config.tauri.bundle.identifier)
            .join(CONFIG_FILENAME);
        return fs::read_to_string(p)
            .map(Theme::from)
            .unwrap_or(Theme::Auto);
    }
    Theme::Auto
}

pub(crate) fn saved_theme_value<R: Runtime>(app: &AppHandle<R>) -> Theme {
    let config_dir = app.path().app_config_dir().expect(ERROR_MESSAGE);
    let p = config_dir.join(CONFIG_FILENAME);
    fs::read_to_string(p)
        .map(Theme::from)
        .unwrap_or(Theme::Auto)
}

pub(crate) fn save_theme_value<R: Runtime>(app: &AppHandle<R>, theme: Theme) {
    let config_dir = app.path().app_config_dir().expect(ERROR_MESSAGE);
    if !config_dir.exists() {
        let _ = std::fs::create_dir_all(&config_dir);
    }
    let p = config_dir.join(CONFIG_FILENAME);
    let _ = fs::write(p, theme.to_string());
}
