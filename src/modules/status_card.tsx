import 
{
    h,
    defineComponent,
    defineAsyncComponent,
    inject,
    onMounted,
    CSSProperties,
    PropType,
    RendererNode,
    VNode,
    RendererElement,
    ref,
    toRef
  } from 'vue'

import { NAvatar, NButton, NCard, NIcon, NSpin, NTab, NTabPane, NTable, NTabs, NTooltip } from 'naive-ui';
import { parseDate } from '../services/date.ts';
import { disease_ico, test_ico, vacc_ico } from '../services/svg.ts';

import '../assets/styles/status_card.scss'

const localProps = 
{
    avatar: 
    {
        type: String,
        required: true
    },
    tooltip: 
    {
        type: String,
        required: true
    },
    shadowbox_color:
    {
        type: String,
        default: 'rgba(100, 243, 18, 0.4)'
    }
    
} as const

export const StatusCardAsync = defineAsyncComponent({
     loader: () => import ('./status_card.tsx'),
     loadingComponent: h(NSpin)
})

export const StatusCard = defineComponent({
name: 'StatusCard',
props: localProps,
    setup (props, ctx) 
    {
        const card = () =>
        {
            return h('div',
            {
                style:{
                    '--card-boxshadow': props.shadowbox_color,
                    //'--hover-button-background': props.shadowbox_color,
                    //'--hover-button-boxshadow': 'transparent',
                    //'--card-background': 'radial-gradient(#92d3d9, #e4f0f1)'
                    //'--card-background': '#e4f0f1)'
                } as CSSProperties
            },
            [
                h('div',
                {
                    class: 'card',
                    style:
                    {
                       width: '95%'
                    } as CSSProperties
                },
                h('div',
                {
                    style:
                    {
                        display: 'flex',
                        flexDirection: 'row',
                        alignItems: 'center',
                        width: '100%'
                    }   as CSSProperties
                },
                [
                    h(NTooltip,{placement: 'left'},
                    {
                        trigger:() =>
                        h(NAvatar,
                        {
                            size: 30,
                            src: props.avatar,
                            class: 'hover-button',
                            style:
                            {
                                backgroundColor: 'transparent',
                                marginRight: '5px',
                                minWidth: '50px'
                            }   as CSSProperties
                            
                        }),
                        default:() => props.tooltip
                    }),
                    h('div',
                    {
                        style:
                        {
                            flexGrow: '1',
                            display: 'flex',
                            placeItems: 'start'

                        }   as CSSProperties
                    },
                    ctx.slots.default?.() ?? []
                    )
                ])
                )
            ])
        }
        return {card}
    },

    render ()
    {
        return this.card()
    }
})