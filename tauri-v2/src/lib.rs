mod platform;

use platform::set_theme;
use serde::{Deserialize, Serialize};
use std::fs;
use tauri::plugin::{Builder, TauriPlugin};
use tauri::{command, generate_handler, AppHandle, Manager, RunEvent, Runtime};

const CONFIG_FILENAME: &str = "tauri-plugin-theme";
const ERROR_MESSAGE: &str = "Get app config dir failed";

pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("theme")
        .invoke_handler(generate_handler![get_theme, set_theme])
        .on_event(|app, e| {
            if let RunEvent::Ready = e {
                let theme = saved_theme_value(&app);
                if let Err(err) = set_theme(app.clone(), theme) {
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

impl ToString for Theme {
    fn to_string(&self) -> String {
        match self {
            Theme::Auto => "auto".into(),
            Theme::Light => "light".into(),
            Theme::Dark => "dark".into(),
        }
    }
}

#[command]
fn get_theme<R: Runtime>(app: AppHandle<R>) -> Result<Theme, ()> {
    let theme = saved_theme_value(&app);
    Ok(theme)
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
