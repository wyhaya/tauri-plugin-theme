import { defineConfig } from 'vite'

export default defineConfig({
  build: {
    lib: {
      entry: './webview-src/index.ts',
      name: 'index',
      fileName: 'index'
    },
    outDir: 'webview-dist'
  }
})
