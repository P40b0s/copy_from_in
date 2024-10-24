import 
{
    h,
    defineComponent,
    defineAsyncComponent,
    CSSProperties
  } from 'vue'

import { NCard, NSpin, NTabPane, NTabs} from 'naive-ui';
import { PacketsViewer } from './packets_viewer.tsx';
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
                tab: 'Пакеты',
                name: 'log',

            },
            {
                default:() => h(PacketsViewer)
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