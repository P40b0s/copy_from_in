import 
{
    h,
    defineComponent,
    defineAsyncComponent,
    CSSProperties,
    onMounted,
    ref,
  } from 'vue'

import { NSelect, NSpin, NVirtualList, SelectGroupOption, SelectOption} from 'naive-ui';
import { DateFormat, DateTime} from '../services/date.ts';
import { app_state_store } from '../store/index.ts';
import { StatusCard } from './status_card.tsx';
import { bell_ico, envelope_ico, error_ico } from '../services/svg.ts';
import { IPacket, Task } from '../models/types.ts';
import { settings } from '../services/tauri-service.ts';
import { string } from 'ts-pattern/dist/patterns';
export const SettingsEditorAsync = defineAsyncComponent({
    loader: () => import ('./settings_editor.tsx'),
    loadingComponent: h(NSpin)
})



export const SettingsEditor =  defineComponent({
    setup () 
    {
        const tasks = ref<Task[]>([]);
        const selected_task = ref<Task>();
        onMounted(async ()=>
        {
            let s = await settings.load_settings()
            if(s != undefined)
            {
                //если пришла строка то ошибка
                if(typeof s === "string")
                {
                    console.error(s);
                } 
                else
                {
                    tasks.value = s;
                    selected_task.value = s[0];
                }
                console.log(tasks.value);
            }
        })
        const settings_names = (): Array<SelectOption | SelectGroupOption> =>
        {
            return tasks.value.map(r=>
            {
                return {
                    label: r.name,
                    value: r.name,
                    disabled: false
                }
            })
        }
       
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
                        width: '200px',
                        display: 'flex',
                        flexDirection: 'column',
                    } as CSSProperties
                },
                h(NSelect,
                {
                    value: selected_task.value?.name,
                    options: settings_names(),
                    onUpdateValue:(v: string)=>
                    {
                        selected_task.value = tasks.value.find(f=>f.name == v);
                        
                    }

                })
                ),
                h("div",
                {
                    style:
                    {
                        width: '100%',
                        display: 'flex',
                        flexDirection: 'column',
                    } as CSSProperties
                },
                "НАСТРОЙКИ"
                ),
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