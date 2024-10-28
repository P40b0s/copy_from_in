import { createApp } from 'vue'
import './style.css'
import {App} from './app.tsx'
import emitter from './services/emit'
import {hljsVuePlugin } from './hljs.ts'
const app = createApp(App)
app.provide('emitter', emitter);
app.use(hljsVuePlugin)
//app.use(unifiedApp);
app.mount('#app');