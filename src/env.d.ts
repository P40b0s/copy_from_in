
/// <reference types="vite/client" />
// declare module '*.vue' {
//     import type { DefineComponent } from 'vue'
//     // eslint-disable-next-line @typescript-eslint/no-explicit-any, @typescript-eslint/ban-types
//     const component: DefineComponent<{}, {}, any>
//     export default component
//   }
  
interface ImportMetaEnv 
{
    readonly VITE_API_HOST: string
    readonly VITE_API_WS: string
    // more env variables...
  }
  
  interface ImportMeta 
  {
    readonly env: ImportMetaEnv
  }

// import type * as app from '@tauri-apps/api/app';
// import { invoke } from '@tauri-apps/api/tauri';

// declare global {
//   interface Window {
//     __TAURI__: {
//       app?: typeof app,
//       invoke?: typeof invoke
//       // ... the other tauri modules
//     };
//   }
// }