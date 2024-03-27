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
import { FormInst, FormItemRule, FormRules, NAvatar, NButton, NCard, NDynamicInput, NForm, NFormItem, NIcon, NInput, NInputNumber, NPopconfirm, NSelect, NSpin, NSwitch, NTooltip, NVirtualList, SelectGroupOption, SelectOption, useNotification} from 'naive-ui';
import { DateFormat, DateTime} from '../services/date.ts';
import { app_state_store } from '../store/index.ts';
import { StatusCard } from './status_card.tsx';
import { bell_ico, clean_ico, envelope_ico, error_ico, palm_ico } from '../services/svg.ts';
import { CopyModifer, IPacket, Task, VN, taskClone} from '../models/types.ts';
import { service, settings } from '../services/tauri-service.ts';
import { number, string } from 'ts-pattern/dist/patterns';
import { AddSharp, CheckmarkCircleOutline, Cut, FolderOpenOutline, TrashBin } from '@vicons/ionicons5';
import { HeaderWithDescription } from './header_with_description.tsx';
import { Filter } from '../models/types.ts';
import { naive_notify, notify } from '../services/notification.ts';
import { timer } from '../services/helpers.ts';
import { Loader } from './loader.tsx';
export const ServicesAsync = defineAsyncComponent({
    loader: () => import ('./settings_editor.tsx'),
    loadingComponent: h(NSpin)
})

export const Services =  defineComponent({
    setup () 
    {
        const notify_inj = useNotification();
        const in_work = ref(false);
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
                    h('div',
                    {
                        style:
                        {
                            width: '100%',
                            display: 'flex',
                            flexDirection: 'row',
                        } as CSSProperties
                    },
                    [
                        clean_button()
                    ])
                ]),
            ]
            );
        }

    const clean_button = () => 
    {
       return h(NTooltip,{placement: 'bottom'},
        {
            trigger:() =>
            h(NButton,
            {
                round: true,
                text: true,
                size: 'small',
                disabled: in_work.value,
                onClick: async (c) =>
                {
                    
                    in_work.value = true;
                    const result = await service.clean_dirs();
                    console.log(result)
                    if (result != undefined)
                    {
                        if(result === 'string')
                        {
                            naive_notify(notify_inj, 'error', "Ошибка очистки", result);
                        }
                        else
                            naive_notify(notify_inj, 'success', "Очистка успешно завершена", "Найдено и удалено " + result + " пакетов");
                    }
                    in_work.value = false;
                },
                style:
                {
                    backgroundColor: 'transparent'
                }
            },
            {
                default:() => in_work.value ? h(Loader) : h(NAvatar,
                {
                    size: 40,
                    src: clean_ico,
                    class: 'hover-button',
                    
                    style:
                    {
                        backgroundColor: 'transparent',
                        marginRight: '5px',
                        minWidth: '50px'
                    }   as CSSProperties
                    
                }),
            }),
            default:() => in_work.value ? "Очитска запущена, ожидайте" : "Начать очистку",
        })
    }            
    return {list}
    },
    render ()
    {
        return this.list()
    }
})