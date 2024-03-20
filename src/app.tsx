import 
{
    h,
    defineComponent,
    defineAsyncComponent,
    inject,
    onMounted,
    ref
  } from 'vue'

import { NAvatar, NButton, NConfigProvider, NNotificationProvider, NSpin, dateRuRU, ruRU } from 'naive-ui';
import { invoke } from '@tauri-apps/api/tauri';
import {MainTab} from './modules/main_tab.tsx';
import './main_grid.scss'
//import  {NavMenu}  from './modules/nav.tsx';
//import emitter from './services/emit.ts';
import { match } from 'ts-pattern';
//import {Modals} from './modules/modals.tsx'
//import { TimeWarningsViewer } from './modules/time_warnings/time_warnings_viewer.tsx';
import { darkTheme } from 'naive-ui';

export const AppAsync = defineAsyncComponent({
    loader: () => import ('./app.tsx'),
    loadingComponent: h(NSpin)
})

export const App = defineComponent({
    setup (props) 
    {
        //const deps_editor = ref(false);
        //const posts_editor = ref(false);
        async function backendAdd() 
        {
            console.log("кнопка нажата");
            await invoke('my_custom_command', {  invokeMessage: 'Hello!' })
        }


        const main_div = () =>
        {
           return h(NNotificationProvider,{},
            {
                default:() => h(NConfigProvider,
                {
                    locale: ruRU,
                    dateLocale: dateRuRU,
                    theme: darkTheme,
                    class: 'main-body'
        
                },
                {
                    default:() => 
                    h('div',
                    {
                        class: 'main-container'
                    },
                    [
                        //h(NavMenu, {class: 'menu'}),
                        h(MainTab),
                        //h(Modals),
                        //h(TimeWarningsViewer, {items: time_warnings.value, class: 'footer'}),
                    ]),
                }),
            }) 
        }
       
        const configuration = () =>
        {
            return h(NConfigProvider,
                {
                    locale: ruRU,
                    dateLocale: dateRuRU
        
                },
                {
                    default:() => h(MainTab),
                        
                })
        }
        //const tab = () => h(main_tab)
        return {main_div}
    },
    render ()
    {
        return this.main_div()
    }
})

