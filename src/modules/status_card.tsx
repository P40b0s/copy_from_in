import 
{
    h,
    defineComponent,
    defineAsyncComponent,
    CSSProperties,
    PropType,
  } from 'vue'

import { NAvatar, NIcon, NSpin, NTooltip } from 'naive-ui';

import '../assets/styles/status_card.scss'
import { archive_ico, envelope_ico, image_ico, pdf_ico } from '../services/svg';
import { supported_files } from '../models/file_types';

const localProps = 
{
    avatar: 
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
    },
    files:
    {
        type: Array as PropType<string[]>
    }
    
} as const

export const StatusCard = defineComponent(
{
    props: localProps,
    setup (props, ctx) 
    {

        const files = () =>
        {
            if(props.files)
            {
                const files = props.files.sort((a, b) => supported_files.sorting_order_by_filename(a) - supported_files.sorting_order_by_filename(b))
                return files.map(f=>
                {
                    let file_type = supported_files.get_type_by_filename(f);
                    return h(NTooltip, {
                    },
                    {
                        trigger:() =>
                        h(NAvatar,
                        {
                            size: 15,
                            src: file_type?.icon,
                            style:
                            {
                                backgroundColor: 'transparent',
                                margin: '1px 1px 1px 1px',
                                minWidth: '15px',
                                borderRadius: "10px",
                                verticalAlign: 'top'
                            }   as CSSProperties
                            
                        }),
                        default:() => f
                    });
                })
            }
            else
            {
                return h('span')
            }
        }
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
                        width: 'inherit',
                        backgroundColor: props.task_color,
                        padding: "2px",
                        background: "rgba(27, 126, 110, 0.1)",
                        backdropFilter: "blur( 8px )",
                        "-webkit-backdrop-filter": "blur( 8px )",
                        borderRadius: "10px",
                        border: "1px solid rgba( 255, 255, 255, 0.18 )"
                       //background: 'linear-gradient(0.25turn, #000000cf, 90%, '+ props.task_color + ', #ebf8e100)',
                       //background: 'linear-gradient(0.25turn, '+ props.task_color + ',2%, #000000cf, 90% , #240921)',
                    } as CSSProperties
                },
                h('div',
                {
                    style:
                    {
                        display: 'flex',
                        flexDirection: 'row',
                        width: '100%'
                    }   as CSSProperties
                },
                [
                    h('div',{},
                    [
                        h(NAvatar,
                        {
                            size: 80,
                            src: props.avatar,
                            class: 'hover-button',
                            style:
                            {
                                backgroundColor: 'transparent',
                                margin: '1px 5px 5px 1px',
                                minWidth: '80px',
                                borderRadius: "10px",
                                verticalAlign: 'top'
                            }   as CSSProperties
                            
                        }),
                        h('div',
                        {
                            style:
                            {
                                display: 'flex',
                                flexDirection: 'row',
                                flexWrap:'wrap',
                                justifyContent: 'space-between',
                                width: '85px',
                             
                            } as CSSProperties
                        },
                        files())

                    ]),
                    h('div',
                    {
                        style:
                        {
                            flexGrow: '1',
                            display: 'flex',
                            placeItems: 'start',
                            height: '100%'

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