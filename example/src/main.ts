import { invoke } from "@tauri-apps/api/tauri";

type Theme = "auto" | "light" | "dark";

const setTheme = (theme: Theme) => {
  invoke("plugin:theme|set_theme", {
    theme,
  }).catch((err) => {
    alert(err.toString());
  });
};

document.getElementById("auto")!.onclick = () => setTheme("auto");
document.getElementById("light")!.onclick = () => setTheme("light");
document.getElementById("dark")!.onclick = () => setTheme("dark");
