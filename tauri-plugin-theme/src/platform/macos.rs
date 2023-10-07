use crate::Theme;
use cocoa::{
    appkit::{NSAppearance, NSAppearanceNameVibrantDark, NSAppearanceNameVibrantLight, NSWindow},
    base::{id, nil},
};
use tauri::{command, AppHandle, Manager, Runtime};

#[command]
pub fn set_theme<R: Runtime>(app: AppHandle<R>, theme: Theme) -> Result<(), &'static str> {
    for window in app.windows().values() {
        let ptr = window.ns_window().map_err(|_| "Invalid window handle")?;
        unsafe {
            let val = match theme {
                Theme::Auto => nil,
                Theme::Light => NSAppearance(NSAppearanceNameVibrantLight),
                Theme::Dark => NSAppearance(NSAppearanceNameVibrantDark),
            };
            (ptr as id).setAppearance(val);
        }
    }
    Ok(())
}
