import 
{
    h,
    defineComponent,
    defineAsyncComponent,
    CSSProperties,
  } from 'vue'

import { NAvatar, NSpin, NTooltip } from 'naive-ui';

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
    },
    task_color:
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
                       width: '95%',
                       borderRadius: '3px',
                       //background: 'linear-gradient(0.25turn, #000000cf, 90%, '+ props.task_color + ', #ebf8e100)',
                       background: 'linear-gradient(0.25turn, '+ props.task_color + ',2%, #000000cf, 90% , #240921)',
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