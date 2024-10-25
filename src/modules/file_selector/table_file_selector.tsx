import 
{
    h,
    ref,
    defineComponent,
    PropType,
  } from 'vue'
import { PacketInfo } from '../../models/backend/document'
import { NPopselect, NTag } from 'naive-ui';
import { fileSelectorLabel, on_update_val, options, get_dir_type } from './file_selector_label';
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
        
        type: Object as PropType<PacketInfo>,
        required: true
    }
} as const

export default defineComponent({
name: 'FileSelector',
props: fileSelectorProps,
    setup (props) 
    {
        const value = ref("");
        const pop = () => {
            return  h(
                NPopselect,
                {
                    placement: 'left',
                    value: value.value,
                    options: options(props.packet),
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
                        default: () => props.packet?.packetDirectory
                    }
                    )
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