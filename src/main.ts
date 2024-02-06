import { createApp } from 'vue'
import './style.css'
import {App} from './app.tsx'
import emitter from './services/emit'
//import { unifiedApp } from './plugins/unified/unified-app';
// When using the Tauri API npm package:
// When using the Tauri global script (if not using the npm package)
// Be sure to set `build.withGlobalTauri` in `tauri.conf.json` to true
//const invoke = window.__TAURI__.invoke;
//await invoke('my_custom_command', { invoke_message: 'Hello!' })
const app = createApp(App)
app.provide('emitter', emitter);
//app.use(unifiedApp);
app.mount('#app');