import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'

export default defineConfig({
  clearScreen: false,
  server: {
    port: 8080,
    strictPort: true,
  },
  envPrefix: ['VITE_', 'TAURI_'],
  build: {
    target: ['es2021', 'chrome97', 'safari13'],
    minify: !process.env.TAURI_DEBUG ? 'esbuild' : false,
    sourcemap: !!process.env.TAURI_DEBUG,
  },
  plugins: [
    react(),
  ],
});
