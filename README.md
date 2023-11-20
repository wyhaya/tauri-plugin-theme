# tauri-plugin-theme

Dynamically change Tauri App theme

## Install

tauri-plugin-theme has been migrated to `tauri@v2`. If you use `tauri@v1`, please use `tauri-plugin-theme@0.2.0` version

```bash
cargo add tauri-plugin-theme
```

```rust
use tauri_plugin_theme::ThemePlugin;

let mut ctx = tauri::generate_context!();
tauri::Builder::default()
    // Init plugin and auto restore window theme !!!
    .plugin(ThemePlugin::init(ctx.config_mut()))
    ...
```

## Usage

```javascript
// Follow system theme setting
invoke("plugin:theme|set_theme", {
  theme: "auto",
});

// Always use light theme
invoke("plugin:theme|set_theme", {
  theme: "light",
});

// Always use dark theme
invoke("plugin:theme|set_theme", {
  theme: "dark",
});

// Get saved theme (default: auto)
const theme = await invoke("plugin:theme|get_theme");
```

## Support

| MacOS | Linux | Windows |
| ----- | ----- | ------- |
| ✅    | ✅    | ✅      |

## Note

- On `Windows` platform, when calling `set_theme`, the app will restart.
- On `Linux` platform, it has not been extensively tested. If you encounter any issues, please submit an issue.
