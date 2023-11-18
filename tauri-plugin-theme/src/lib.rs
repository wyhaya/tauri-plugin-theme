mod platform;

use platform::set_theme;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use tauri::plugin::{Builder, TauriPlugin};
use tauri::utils::config::TauriConfig;
use tauri::Manager;
use tauri::{command, generate_handler, Config};
use tauri::{AppHandle, Runtime};

const PLUGIN_NAME: &str = "theme";
const CONFIG_FILENAME: &str = "tauri-plugin-theme";

pub struct ThemePlugin;

impl ThemePlugin {
    #[cfg(target_os = "macos")]
    pub fn init<R: Runtime>(_config: &mut Config) -> TauriPlugin<R, TauriConfig> {
        use tauri::RunEvent;
        Builder::new(PLUGIN_NAME)
            .invoke_handler(generate_handler![get_theme, set_theme])
            .on_event(|app, e| {
                if let RunEvent::Ready = e {
                    let theme = saved_theme_value(
                        &app.path().config_dir().expect("failed to get config dir"),
                    );
                    let _ = set_theme(app.clone(), theme);
                }
            })
            .build()
    }

    #[cfg(target_os = "linux")]
    pub fn init<R: Runtime>(_config: &mut Config) -> TauriPlugin<R, TauriConfig> {
        Builder::new(PLUGIN_NAME)
            .invoke_handler(generate_handler![get_theme, set_theme])
            .setup(|app| {
                let theme = saved_theme_value(&app.config());
                let _ = set_theme(app.clone(), theme);
                Ok(())
            })
            .build()
    }

    #[cfg(target_os = "windows")]
    pub fn init<R: Runtime>(config: &mut Config) -> TauriPlugin<R, TauriConfig> {
        let theme = saved_theme_value(config);
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
}

#[command]
fn get_theme<R: Runtime>(app: AppHandle<R>) -> Result<Theme, ()> {
    let theme = saved_theme_value(&app.path().config_dir().expect("failed to get config dir"));
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

pub(crate) fn saved_theme_value(path: &PathBuf) -> Theme {
    fs::read_to_string(path)
        .map(Theme::from)
        .unwrap_or(Theme::Auto)
}

pub(crate) fn save_theme_value(path: &PathBuf, theme: Theme) {
    if !path.exists() {
        let _ = std::fs::create_dir_all(&path);
    }
    let p = path.join(CONFIG_FILENAME);
    let _ = fs::write(p, theme.to_string());
}
