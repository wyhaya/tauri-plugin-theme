use crate::{save_theme_value, Theme};
use std::sync::atomic::{AtomicU8, Ordering};
use std::sync::Arc;
use tauri::{command, AppHandle, Manager, Runtime};
use webview2_com::Microsoft::Web::WebView2::Win32::*;
use windows_core::Interface;

#[command]
pub fn set_theme<R: Runtime>(app: AppHandle<R>, theme: Theme) -> Result<(), &'static str> {
    let err = Arc::new(AtomicU8::new(0));
    for (_, window) in app.webview_windows() {
        // Window
        let hwnd = window.hwnd().map_err(|_| "Invalid window handle")?;
        darkmode::try_window_theme(hwnd, theme, false);

        // Webview
        let (err, err2) = (err.clone(), err.clone());
        let rst = window.with_webview(move |view| unsafe {
            let controller = view.controller();
            let coreview = match controller.CoreWebView2() {
                Ok(coreview) => coreview,
                Err(_) => {
                    err.store(1, Ordering::SeqCst);
                    return;
                }
            };
            let webview = match coreview.cast::<ICoreWebView2_13>() {
                Ok(webview) => webview,
                Err(_) => {
                    err.store(2, Ordering::SeqCst);
                    return;
                }
            };
            let profile = match webview.Profile() {
                Ok(profile) => profile,
                Err(_) => {
                    err.store(3, Ordering::SeqCst);
                    return;
                }
            };
            let theme = match theme {
                Theme::Dark => COREWEBVIEW2_PREFERRED_COLOR_SCHEME_DARK,
                Theme::Light => COREWEBVIEW2_PREFERRED_COLOR_SCHEME_LIGHT,
                Theme::Auto => COREWEBVIEW2_PREFERRED_COLOR_SCHEME_AUTO,
            };
            if let Err(_) = profile.SetPreferredColorScheme(theme) {
                err.store(4, Ordering::SeqCst);
                return;
            }
        });
        if let Err(_) = rst {
            return Err("Get webview error");
        }
        match err2.load(Ordering::SeqCst) {
            1 => return Err("Get CoreWebView2 error"),
            2 => return Err("Get ICoreWebView2_13 error"),
            3 => return Err("Get webview.Profile error"),
            4 => return Err("Set SetPreferredColorScheme error"),
            _ => {}
        }
    }
    save_theme_value(&app, theme);
    Ok(())
}

// From: https://github.com/tauri-apps/tao/blob/2c6de758ac656c0621d20da7c1649adfb8847066/src/platform_impl/windows/dark_mode.rs
mod darkmode {
    // Copyright 2014-2021 The winit contributors
    // Copyright 2021-2023 Tauri Programme within The Commons Conservancy
    // SPDX-License-Identifier: Apache-2.0

    use crate::Theme;
    use std::ffi::c_void;
    use std::sync::LazyLock;
    /// This is a simple implementation of support for Windows Dark Mode,
    /// which is inspired by the solution in https://github.com/ysc3839/win32-darkmode
    use windows::{
        core::{s, w, PCSTR, PSTR},
        Win32::{
            Foundation::{BOOL, HANDLE, HMODULE, HWND, WPARAM},
            Graphics::Dwm::{DwmSetWindowAttribute, DWMWA_USE_IMMERSIVE_DARK_MODE},
            System::LibraryLoader::*,
            UI::{Accessibility::*, WindowsAndMessaging::*},
        },
    };

    static HUXTHEME: LazyLock<isize> =
        LazyLock::new(|| unsafe { LoadLibraryA(s!("uxtheme.dll")).unwrap_or_default().0 as _ });

    static WIN10_BUILD_VERSION: LazyLock<Option<u32>> = LazyLock::new(|| {
        let version = windows_version::OsVersion::current();
        if version.major == 10 && version.minor == 0 {
            Some(version.build)
        } else {
            None
        }
    });

    static DARK_MODE_SUPPORTED: LazyLock<bool> = LazyLock::new(|| {
        // We won't try to do anything for windows versions < 17763
        // (Windows 10 October 2018 update)
        match *WIN10_BUILD_VERSION {
            Some(v) => v >= 17763,
            None => false,
        }
    });

    /// Attempt to set a theme on a window, if necessary.
    /// Returns the theme that was picked
    pub fn try_window_theme(hwnd: HWND, preferred_theme: Theme, redraw_title_bar: bool) {
        if *DARK_MODE_SUPPORTED {
            let is_dark_mode = match preferred_theme {
                Theme::Auto => should_use_dark_mode(),
                Theme::Dark => true,
                Theme::Light => false,
            };
            refresh_titlebar_theme_color(hwnd, is_dark_mode, redraw_title_bar);
        }
    }

    fn refresh_titlebar_theme_color(hwnd: HWND, is_dark_mode: bool, redraw_title_bar: bool) {
        if let Some(ver) = *WIN10_BUILD_VERSION {
            if ver < 18362 {
                let mut is_dark_mode_bigbool: i32 = is_dark_mode.into();
                unsafe {
                    let _ = SetPropW(
                        hwnd,
                        w!("UseImmersiveDarkModeColors"),
                        HANDLE(&mut is_dark_mode_bigbool as *mut _ as _),
                    );
                }
            } else {
                let dark_mode = BOOL::from(is_dark_mode);
                unsafe {
                    let _ = DwmSetWindowAttribute(
                        hwnd,
                        DWMWA_USE_IMMERSIVE_DARK_MODE,
                        &dark_mode as *const BOOL as *const c_void,
                        std::mem::size_of::<BOOL>() as u32,
                    );
                }
                if redraw_title_bar {
                    unsafe { DefWindowProcW(hwnd, WM_NCACTIVATE, None, None) };
                    unsafe { DefWindowProcW(hwnd, WM_NCACTIVATE, WPARAM(true.into()), None) };
                }
            }
        }
    }

    fn should_use_dark_mode() -> bool {
        should_apps_use_dark_mode() && !is_high_contrast()
    }

    fn should_apps_use_dark_mode() -> bool {
        const UXTHEME_SHOULDAPPSUSEDARKMODE_ORDINAL: u16 = 132;
        type ShouldAppsUseDarkMode = unsafe extern "system" fn() -> bool;
        static SHOULD_APPS_USE_DARK_MODE: LazyLock<Option<ShouldAppsUseDarkMode>> =
            LazyLock::new(|| unsafe {
                if HMODULE(*HUXTHEME as _).is_invalid() {
                    return None;
                }

                GetProcAddress(
                    HMODULE(*HUXTHEME as _),
                    PCSTR::from_raw(UXTHEME_SHOULDAPPSUSEDARKMODE_ORDINAL as usize as *mut _),
                )
                .map(|handle| std::mem::transmute(handle))
            });

        SHOULD_APPS_USE_DARK_MODE
            .map(|should_apps_use_dark_mode| unsafe { (should_apps_use_dark_mode)() })
            .unwrap_or(false)
    }

    fn is_high_contrast() -> bool {
        const HCF_HIGHCONTRASTON: u32 = 1;

        let mut hc = HIGHCONTRASTA {
            cbSize: 0,
            dwFlags: Default::default(),
            lpszDefaultScheme: PSTR::null(),
        };

        let ok = unsafe {
            SystemParametersInfoA(
                SPI_GETHIGHCONTRAST,
                std::mem::size_of_val(&hc) as _,
                Some(&mut hc as *mut _ as _),
                Default::default(),
            )
        };

        ok.is_ok() && (HCF_HIGHCONTRASTON & hc.dwFlags.0) != 0
    }
}
