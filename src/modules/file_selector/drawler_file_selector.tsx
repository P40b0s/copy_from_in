import 
{
    h,
    defineComponent,
    PropType,
  } from 'vue'
import { PacketInfo } from '../../models/backend/document'
import { NSelect, NTag} from 'naive-ui';
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
    },
    selected: 
    {
        type: String,
        required: true
    }
} as const

export default defineComponent({
name: 'FileSelector',
props: fileSelectorProps,
    setup (props) 
    {
        
        const pop = () => {
            return  h(
                NSelect,
                {
                    value: props.selected,
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
                    })
                }
            )
        }
        return pop
        
    },
    render () 
    {
        return h(this)
    }
})