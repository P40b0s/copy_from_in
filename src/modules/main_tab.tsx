import 
{
    h,
    defineComponent,
    defineAsyncComponent,
    CSSProperties,
    Suspense
  } from 'vue'

import { NCard, NSpin, NTabPane, NTabs} from 'naive-ui';
import { PacketsViewer } from './packets_viewer.tsx';
import Loader2 from './Loader/Loader2.vue';
import { SettingsEditor } from './settings_editor.tsx';
import SendersViewer from './Senders/SendersViewer.vue';

export const MainTab =  defineComponent({
    setup (props) 
    {
    const crd = () => 
    {
        return h(NCard,
            {
                style:
                {
                    marginBottom: '0px',
                    height: '100%',
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
                default:() =>  tab_view(),
                    // h(Suspense, 
                    // null,
                    // {
                    //     default:()=> h(FileViewer),
                    //     fallback:() => h(Loader2)
                    // })])
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
                defaultValue: "pak",
                style:
                {
                      height: '100%'
                }
            },
            {
                default:() => [packets_tab(), senders_tab(), settings_tab()]
            }
        )
    }

    const packets_tab = () => 
    {
        return h(NTabPane,
            {
                tab: 'Пакеты',
                name: 'pak',
                style:
                {
                      height: '100%'
                }
            },
            {
                default:() =>
                h(Suspense, 
                null,
                {
                    default:()=> h(PacketsViewer),
                    fallback:() => h(Loader2)
                })
            }
        )
    }
    const senders_tab = () => 
        {
            return h(NTabPane,
                {
                    tab: 'Отправители',
                    name: 'snd',
                    style:
                    {
                        height: '100%'
                    }
                },
                {
                    default:() =>
                    h(Suspense, 
                    null,
                    {
                        default:()=> h(SendersViewer),
                        fallback:() => h(Loader2)
                    })
                }
            )
        }
    const settings_tab = () => 
    {
        return h(NTabPane,
            {
                tab: 'Настройки',
                name: 'set',
                style:
                {
                    height: '100%'
                }
            },
            {
                default:() => 
                h(Suspense, 
                null,
                {
                    default:()=> h(SettingsEditor),
                    fallback:() => h(Loader2)
                })
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