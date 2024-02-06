import 
{
    h,
    defineComponent,
  } from 'vue'
import { NInput} from 'naive-ui';
import { timer } from '../services/helpers'
const localProps = 
{
    value: 
    {
        type: String,
        default: ""
    },
    palceholder: 
    {
        type: String,
        default: "Фильтрация списка"
    },
    inputDelay: 
    {
        type: Number,
        default: 500
    },
} as const

export const LiveSearch = defineComponent({
name: 'LiveSearch',
props: localProps,
emits:
{
    'update:value': (value: string) => true
},
    setup (props, {emit}) 
    {
        let tm = timer(props.inputDelay);
        const element = () =>
        {
            return h(NInput,
            {
                placeholder: props.palceholder,
                clearable: true,
                onUpdateValue: async (val: string) =>
                {
                    clearTimeout(tm)
                    tm = setTimeout(async () => 
                    {
                        emit('update:value', val)
                    }, props.inputDelay)
                }
            },
            {

            });
        }
        return {element}
    },
    render ()
    {
        return h(this.element)
    }
})