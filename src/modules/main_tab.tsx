import 
{
    h,
    defineComponent,
    defineAsyncComponent,
    inject,
    onMounted,
    CSSProperties
  } from 'vue'

import { NAvatar, NBadge, NButton, NCard, NSpin, NTab, NTabPane, NTabs, NTooltip } from 'naive-ui';
//import { global_store } from '../store/index.ts';
import { disease_ico, palm_ico } from '../services/svg.ts';
import { LogViewer } from './log_viewer.tsx';
import { SettingsEditor } from './settings_editor.tsx';
import { Services } from './services.tsx';

export const MainTabAsync = defineAsyncComponent({
    loader: () => import ('./main_tab.tsx'),
    loadingComponent: h(NSpin)
})

export const MainTab =  defineComponent({
    setup (props) 
    {
    const crd = () => 
    {
        return h(NCard,
            {
                style:
                {
                    marginBottom: '0px'
                } as CSSProperties
            },
            {
                header:() =>
                h('div',
                {
                    style:
                    {
                        display: 'flex',
                        flexDirection: 'row'
                    } as CSSProperties
                },
                [
                ]),
                default:() =>  tab_view()
            }
        )
    }

    const tab_view = () =>
    {
        return h(NTabs,
            {
                justifyContent: 'space-evenly',
                type: 'line',
                size: 'large',
                defaultValue: "log"
            },
            {
                default:() => [log_tab(), settings_tab()]
            }
        )
    }

    const log_tab = () => 
    {
        return h(NTabPane,
            {
                tab: 'Логирование',
                name: 'log',

            },
            {
                default:() => h(LogViewer)
            }
        )
    }
    const service_tab = () => 
    {
        return h(NTabPane,
            {
                tab: 'Сервис',
                name: 'serv'
            },
            {
                default:() => h(Services)
            }
        )
    }
    const settings_tab = () => 
    {
        return h(NTabPane,
            {
                tab: 'Настройки',
                name: 'set'
            },
            {
                default:() => h(SettingsEditor)
            }
        )
    }

    return {crd}
    },
    render ()
    {
        return this.crd();
    }
})