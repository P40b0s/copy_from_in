import 
{
    h,
    defineComponent,
    PropType,
    ref,
    CSSProperties,
  } from 'vue'
import { NSelect, NTag} from 'naive-ui';
import { fileSelectorLabel, options, SelectedValue } from './file_selector_label';
import { type IPacket } from '../../../models/types';
import { SelectBaseOption, type Value } from 'naive-ui/es/select/src/interface';
import { emit } from '@tauri-apps/api/event';

export const fileSelectorProps = 
{
    placement: 
    {
        type: String as PropType<"left" | "right | bottom">,
        default: "bottom"
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
        //открываем первый попавшийся pdf
        let first = opt.find(f=>f.ext == "pdf");
        const selected = ref<Value|null|undefined>(first?.value);
        if(selected.value)
        {
            emits.emit('onSelect', first as SelectedValue);
        }
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
                        selected.value = opt[s].value
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
                        type: "success",
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