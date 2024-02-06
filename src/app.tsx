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
import './main_grid.scss'
import emitter from './services/emit.ts';
import { match } from 'ts-pattern';
import { darkTheme } from 'naive-ui';

export const AppAsync = defineAsyncComponent({
    loader: () => import ('./app.tsx'),
    loadingComponent: h(NSpin)
})

export const App =  defineComponent({
    setup (props) 
    {
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
                       h('div', "ТЕСТ!")
                    ]),
                }),
            }) 
        }
        return {main_div}
    },
    
    render ()
    {
        return this.main_div()
    }
})

