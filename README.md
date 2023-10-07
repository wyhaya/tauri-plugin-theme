# tauri-plugin-theme

Dynamically change Tauri App theme

## Install

```bash
cargo add tauri-plugin-theme
```

```rust
use tauri_plugin_theme::ThemePlugin;

tauri::Builder::default()
    // Init plugin !!!
    .plugin(ThemePlugin::init())
```

## Usage

```javascript
import { invoke } from "@tauri-apps/api/tauri";

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
```

## Support

| MacOS | Linux | Windows |
| ----- | ----- | ------- |
| ✅    | ✅    | ❌      |

- On the `Linux` platform, it has not been extensively tested. If you encounter any issues, please submit an issue.
- Currently does not support the `Windows` platform, PR are welcome.
