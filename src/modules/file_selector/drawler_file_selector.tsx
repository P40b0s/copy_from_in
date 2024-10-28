import 
{
    h,
    defineComponent,
    PropType,
    ref,
  } from 'vue'
import { NSelect, NTag} from 'naive-ui';
import { fileSelectorLabel, on_update_val, options, get_dir_type, SelectedValue } from './file_selector_label';
import { type IPacket } from '../../models/types';
import { SelectBaseOption } from 'naive-ui/es/select/src/interface';
import { emit } from '@tauri-apps/api/event';
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
    },
    
} as const

export default defineComponent({
props: fileSelectorProps,
emits:
{
    'onSelect': (value: SelectedValue) => true
},
    async setup (props, emits) 
    {
        const opt = await options(props.packet);
        const selected = ref("");
        const pop = () => 
        {
            return  h(
                NSelect,
                {
                    value: selected.value,
                    options: opt,
                    onUpdateValue:  (val: string, option: SelectBaseOption|null) =>
                    {
                        let s = opt.findIndex(i=> i.path == val);
                        emits.emit('onSelect', opt[s]);
                        selected.value = opt[s].label;
                        
                    },
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
                        default: () => props.packet.name
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