import 
{
    h,
    defineComponent,
    defineAsyncComponent,
    CSSProperties,
    onMounted,
    ref,
    Ref,
  } from 'vue'
import { open } from '@tauri-apps/api/dialog';
import { relaunch } from '@tauri-apps/api/process';
import { FormInst, FormItemRule, FormRules, NButton, NCard, NDynamicInput, NForm, NFormItem, NIcon, NInput, NInputNumber, NPopconfirm, NSelect, NSpin, NSwitch, NTooltip, NVirtualList, SelectGroupOption, SelectOption} from 'naive-ui';
import { DateFormat, DateTime} from '../services/date.ts';
import { app_state_store } from '../store/index.ts';
import { StatusCard } from './status_card.tsx';
import { bell_ico, envelope_ico, error_ico } from '../services/svg.ts';
import { CopyModifer, IPacket, Task, VN, taskClone} from '../models/types.ts';
import { settings } from '../services/tauri-service.ts';
import { string } from 'ts-pattern/dist/patterns';
import { AddSharp, CheckmarkCircleOutline, FolderOpenOutline, TrashBin } from '@vicons/ionicons5';
import { HeaderWithDescription } from './header_with_description.tsx';
import { Filter } from '../models/types.ts';
import { notify } from '../services/notification.ts';
import { timer } from '../services/helpers.ts';
import "./loader.scss";

export const Loader =  defineComponent({
    setup () 
    {
        const arr = (len: number) : VN[] =>
        {
            let array: VN[] = [];
            for (let index = 0; index < len; index++) 
            {
                array.push(h("i"));
            }
            return array;
        }
        const list = () =>
        {
            return h("div", {class:"loader"},
            arr(72))
        }          
        return {list}
    },
    render ()
    {
        return this.list()
    }
})