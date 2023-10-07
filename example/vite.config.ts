import { defineConfig } from "vite";

export default defineConfig(async () => ({
  clearScreen: false,
  server: {
    port: 1420,
    strictPort: true,
  },
  root: "./src/",
  envPrefix: ["VITE_", "TAURI_"],
}));
