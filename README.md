# tauri-plugin-theme

Tauri currently doesn't support dynamically change app theme, this plugin makes up for that.

https://github.com/wyhaya/tauri-plugin-theme/assets/23690145/2422ce95-418d-4f07-adf5-e78af2552f51

## Run example

```
cd [this-repo]
pnpm i
pnpm tauri dev
```

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
    .plugin(tauri_plugin_theme::init(ctx.config_mut()))
    // ...
    .run(ctx)
    // ...
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

When you call `set_theme`, the theme will `auto saved`, when the App is restarted, theme will be `auto restored`.

> If you're using tauri@v2 you'll also need to add the plugin's permissions file

Add file `src-tauri/capabilities/theme.json`

```json
{
  "$schema": "./schemas/desktop-schema.json",
  "identifier": "main-theme-capability",
  "description": "tauri-plugin-theme",
  "windows": ["main"],
  "permissions": ["theme:allow-set-theme", "theme:allow-get-theme"]
}
```

## Support

| MacOS | Linux | Windows |
| ----- | ----- | ------- |
| ✅    | ✅    | ✅      |

## Note

- On `Windows` platform, when calling `set_theme`, the app will restart.
- On `Linux` platform, it has not been extensively tested. If you encounter any issues, please submit an issue.
