use tauri::command;

#[command]
pub fn set_theme() -> Result<(), &'static str> {
    Err("Windows currently not supported")
}
