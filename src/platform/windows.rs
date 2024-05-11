use crate::{save_theme_value, Theme};
use tauri::{command, AppHandle, Manager, Runtime};
use windows_sys::Win32::Graphics::Dwm::*;
use windows_sys::Win32::Foundation::*;

#[command]
pub fn cmd_set_theme<R: Runtime>(app: AppHandle<R>, theme: Theme) -> Result<(), &'static str> {
    save_theme_value(&app.config(), theme);
    for window in app.windows().values() {
        unsafe {
            let handle = window.hwnd().unwrap().0;
            let val: Option<BOOL> = match theme {
                Theme::Dark => Some(1),
                Theme::Light => Some(0),
                Theme::Auto => {
                    match window.theme() {
                        Ok(tauri::Theme::Dark) => Some(1),
                        Ok(tauri::Theme::Light) => Some(0),
                        _ => None,
                    }
                },
            };
            if let Some(value) = val {
                let attribute = DWMWA_USE_IMMERSIVE_DARK_MODE;
                DwmSetWindowAttribute(handle, attribute as u32, &value as *const _ as *const _, std::mem::size_of::<BOOL>() as u32);
            }
        }
    }
    Ok(())
}
