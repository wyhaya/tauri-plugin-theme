const COMMANDS: &[&str] = &["set_theme", "get_theme"];

fn main() {
    tauri_plugin::Builder::new(COMMANDS).build();
}
