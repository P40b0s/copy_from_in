import 
{
    h,
    ref,
    defineComponent,
    PropType,
  } from 'vue'
import { NPopselect, NTag } from 'naive-ui';
import { fileSelectorLabel, on_update_val, options, get_dir_type } from './file_selector_label';
import { type IPacket } from '../../../models/types';
export const fileSelectorProps = 
{
    placement: 
    {
        type: String as PropType<"left" | "right">,
        default: "left"
    },
    /**Транспотрный пакет */
    packet: 
    {
        
        type: Object as PropType<IPacket>,
        required: true
    }
} as const

export default defineComponent({
props: fileSelectorProps,
    async setup (props) 
    {
        const opt = await options(props.packet);
        const value = ref("");
        const pop = () => {
            return  h(
                NPopselect,
                {
                    placement: 'left',
                    value: value.value,
                    options: opt,
                    onUpdateValue: on_update_val,
                    renderLabel: fileSelectorLabel
                },
                {
                    default: () => 
                    h(NTag,
                    {
                        style: 
                        {
                            marginRight: '6px',
                        },
                        type: get_dir_type(props.packet),
                        bordered: false
                    },
                    {
                        default: () => props.packet?.name
                    })
                }
            )
        }
        return {
            pop,
        }
    },
    render ()
    {
        // const 
        // {
        //     pop
        // } = this;
        return h(this.pop)
    }
})