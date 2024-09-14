#![allow(unused_variables)]

mod platform;

use platform::set_theme;
use serde::{Deserialize, Serialize};
use std::fs;
use tauri::plugin::{Builder, TauriPlugin};
use tauri::{command, generate_handler, AppHandle, Config, Manager, Runtime};

const CONFIG_FILENAME: &str = "tauri-plugin-theme";
const ERROR_MESSAGE: &str = "Get app config dir failed";

pub fn init<R: Runtime>(config: &mut Config) -> TauriPlugin<R> {
    #[cfg(any(target_os = "macos", target_os = "windows"))]
    {
        let theme = saved_theme_value_from_config(config);
        for window in &mut config.app.windows {
            match theme {
                Theme::Auto => window.theme = None,
                Theme::Light => window.theme = Some(tauri::Theme::Light),
                Theme::Dark => window.theme = Some(tauri::Theme::Dark),
            }
        }
    }
    Builder::new("theme")
        .invoke_handler(generate_handler![get_theme, set_theme])
        .on_event(|app, e| {
            #[cfg(target_os = "linux")]
            if let tauri::RunEvent::Ready = e {
                if let Err(err) = set_theme(app.clone(), saved_theme_value(&app)) {
                    eprintln!("Failed to set theme: {}", err);
                }
            }
        })
        .build()
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
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

impl std::fmt::Display for Theme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Theme::Auto => write!(f, "auto"),
            Theme::Light => write!(f, "light"),
            Theme::Dark => write!(f, "dark"),
        }
    }
}

#[command]
fn get_theme<R: Runtime>(app: AppHandle<R>) -> Result<Theme, ()> {
    let theme = saved_theme_value(&app);
    Ok(theme)
}

#[cfg(any(target_os = "macos", target_os = "windows"))]
fn saved_theme_value_from_config(config: &Config) -> Theme {
    if let Some(dir) = dirs_next::config_dir() {
        let p = dir.join(&config.identifier).join(CONFIG_FILENAME);
        return fs::read_to_string(p)
            .map(Theme::from)
            .unwrap_or(Theme::Auto);
    }
    Theme::Auto
}

fn saved_theme_value<R: Runtime>(app: &AppHandle<R>) -> Theme {
    let config_dir = app.path().app_config_dir().expect(ERROR_MESSAGE);
    let p = config_dir.join(CONFIG_FILENAME);
    fs::read_to_string(p)
        .map(Theme::from)
        .unwrap_or(Theme::Auto)
}

fn save_theme_value<R: Runtime>(app: &AppHandle<R>, theme: Theme) {
    let config_dir = app.path().app_config_dir().expect(ERROR_MESSAGE);
    if !config_dir.exists() {
        let _ = std::fs::create_dir_all(&config_dir);
    }
    let p = config_dir.join(CONFIG_FILENAME);
    let _ = fs::write(p, theme.to_string());
}
