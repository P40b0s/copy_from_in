import 
{
    h,
    defineComponent,
    defineAsyncComponent,
    Suspense,
  } from 'vue'

import { NConfigProvider, NNotificationProvider, NSpin, NotificationType, dateRuRU, ruRU, useNotification } from 'naive-ui';
import {MainTab} from './modules/main_tab.tsx';
import './main_grid.scss'
import { darkTheme } from 'naive-ui';
import { Services } from './modules/services.tsx';
import FileViewer from './modules/file_viewer/FileViewer.vue';
import Loader2 from './modules/Loader/Loader2.vue';
export const App = defineComponent({
    setup () 
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
                        class: 'main-container',
                    },
                    [
                        h(Services, {class: 'header'}),
                        h(MainTab, {class: "main-body"}),
                        h('div', {class: 'footer'}, ""),
                        h(Suspense, 
                        null,
                        {
                            default:()=> h(FileViewer),
                            fallback:() => h(Loader2)
                        })
                        //h(Modals),
                        //h(TimeWarningsViewer, {items: time_warnings.value, class: 'footer'}),
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

