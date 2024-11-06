import 
{
    h,
    defineComponent,
  } from 'vue'
import { HelpCircleOutline } from '@vicons/ionicons5';
import { NIcon, NTooltip } from 'naive-ui';

export const headerWithDescriptionProps = 
{
    name: String,
    description: String,
    maxWidth:
    {
        type: String,
        default: '300px'
    },
    fontSize:
    {
        type: String,
        default: '18px'
    },
} as const

const emits = 
{
    'update:items': (values: string[]) => values
}
export const HeaderWithDescription =  defineComponent({
props: headerWithDescriptionProps,
emits: emits,
    setup (props, emits) 
    {
        const element = () => {
            return h('div',
            {
                style:
                {
                    display: 'flex',
                    width: '100%',
                    justifyContent: 'center',
                    alignItems: 'self-start',
                    gap: '2px'
                }
            },
            [
                h('span',
                {
                    style:
                    {
                        fontSize: props.fontSize
                    }
                },
                props.name),
                h(
                    NTooltip,
                    {
                        style:
                        {
                            maxWidth: props.maxWidth
                        }
                    },
                    {
                        trigger: () => 
                            h(NIcon,
                                {
                                    color: "#69ec76",
                                    size: 15,
                                    style:
                                    {
                                        cursor: 'pointer'
                                    },
                                    component: HelpCircleOutline
                                },
                            ),
                        default: () => props.description
                    }
                )
            ])
        }
        return {element}
    },
    render ()
    {
        return this.element()
    }
})