import { invoke } from "@tauri-apps/api/primitives";

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

document.getElementById("get")!.onclick = async () => {
  const theme = await invoke<string>("plugin:theme|get_theme");
  alert(`Current theme value: '${theme}'`);
};
