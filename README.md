# tauri-plugin-theme

Tauri currently doesn't support dynamically change app theme, this plugin makes up for that.

https://github.com/wyhaya/tauri-plugin-theme/assets/23690145/2422ce95-418d-4f07-adf5-e78af2552f51

## Install

```bash
# For tauri@v1
cargo add tauri-plugin-theme@1
# For tauri@v2
cargo add tauri-plugin-theme@2
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

### Tauri permissions

If you're using `tauri@v2` you'll also need to add the plugin's permissions file

Add to `src-tauri/capabilities/default.json`

```json5
{
  ...
    "permissions": [
        ...
        "theme:default"
    ]
}
```

## Support

| MacOS | Linux | Windows | Android | iOS |
| ----- | ----- | ------- | ------- | ------- |
| ✅    | ✅    | ✅      | ❌      | ❌

## Note

- On `Windows` platform, when calling `set_theme`, the app will restart.
- On `Linux` platform, it has not been extensively tested.
