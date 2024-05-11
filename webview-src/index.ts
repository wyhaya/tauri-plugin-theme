import { invoke } from "@tauri-apps/api/tauri";

export enum Theme {
  Auto = "auto",
  Light = "light",
  Dark = "dark",
}

export async function setTheme(theme: Theme) {
  await invoke("plugin:theme|cmd_set_theme", {
    theme,
  });
};

export async function getTheme() {
  const theme = await invoke("plugin:theme|cmd_get_theme");
  return theme as Theme;
};
