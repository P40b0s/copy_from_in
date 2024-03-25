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
import { Loader } from './loader.tsx';
export const ServicesAsync = defineAsyncComponent({
    loader: () => import ('./settings_editor.tsx'),
    loadingComponent: h(NSpin)
})


export const Services =  defineComponent({
    setup () 
    {
      
        const list = () =>
        {
            return h('div',
            {
                style:
                {
                    display: 'flex',
                    flexDirection: 'row',
                    width: '100%'
                }   as CSSProperties
            },
            [
                h("div",
                {
                    style:
                    {
                        width: '100%',
                        display: 'flex',
                        flexDirection: 'column',
                    } as CSSProperties
                },
                [
                    h(HeaderWithDescription,{
                        name: "Модификатор копирования",
                        description: "Копирование всех пакетов, или копирование пакетов согласно правилам фильтрации",
                        fontSize: '14px'
                    }),
                    h(NTooltip,
                        {
        
                        },
                        {
                            trigger:() =>  h(NButton,
                            {
                                type: 'primary',
                                onClick: async () => 
                                {
                                    const f : Filter = 
                                    {
                                        document_types: [],
                                        document_uids: []
                                    }
                                    const task: Task = {
                                        name: "новая задача",
                                        source_dir: "",
                                        target_dir: "",
                                        timer: 120000,
                                        delete_after_copy: false,
                                        copy_modifier: "CopyAll",
                                        is_active: true,
                                        filters: f
                                    }
                                }
                            },
                            {
                                icon:() => h(NIcon, {component: AddSharp})
                            }),
                            default:() => "Добавить новую задачу"
                        }),
                        h(Loader)
                    
                ]),
            ]
            );
        }

     
       
                        
                       
        return {list}
    },
    render ()
    {
        return this.list()
    }
})