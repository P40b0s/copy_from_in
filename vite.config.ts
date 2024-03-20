import { defineConfig } from 'vite'
import vue ,{ type Options} from '@vitejs/plugin-vue'

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
    vue(
      { 
        script: 
        {
          defineModel: true,
          defineSlots: true
        },
      } as Options),
  ],
});
