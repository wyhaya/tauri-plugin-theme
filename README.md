# tauri-plugin-theme

> [!NOTE]  
> This only applies to tauri 1x version and is no longer maintained.

## Install

```bash
cargo add tauri-plugin-theme@1.0.0
```

```rust
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

When you call `set_theme`, the theme will be auto saved, and it will be restored auto after the App restart.

## Support

| MacOS | Linux | Windows |
| ----- | ----- | ------- |
| ✅    | ✅    | ✅      |

> [!WARNING]  
> On `Windows` platform, when calling `set_theme`, the app will restart.
