import 
{
    h,
    defineComponent,
    defineAsyncComponent,
    CSSProperties,
    ref,
    onUnmounted,
  } from 'vue'
import { NAvatar, NButton, NSpin, NTooltip, useNotification} from 'naive-ui';
import { clean_ico, cut_ico, offline_ico, online_ico} from '../services/svg.ts';
import { commands_service } from '../services/tauri/commands.ts';
import { naive_notify } from '../services/notification.ts';
import { Loader } from './loader.tsx';
import store from '../store/app_state_store.ts';
import { events } from '../services/tauri/events.ts';
export const ServicesAsync = defineAsyncComponent({
    loader: () => import ('./settings_editor.tsx'),
    loadingComponent: h(NSpin)
})

export const Services =  defineComponent({
setup () 
{
    const notify = useNotification();
    const in_work = ref(false);
    const clean_start_event = events.clean_start(async () => 
    {
        in_work.value = true;
        naive_notify(notify, 'info', "Стартовала задача очистки пакетов", "", 2000);
    })
    const clean_complete_event = events.clean_complete(async (count) => 
    {
        in_work.value = false;
        naive_notify(notify, 'success', "Очистка пакетов завершена, удалено " + count.payload + " пакетов", "", 2000);
    })
    const new_packet_event = events.packets_update(async (packet) => 
    {
        naive_notify(notify, 'info', `В ${packet.payload.parseTime} Найден новый пакет: ${packet.payload.name}"`, "", 2000);
    })
    onUnmounted(()=>
    {
        clean_start_event.then(v=> v.unsubscribe());
        clean_complete_event.then(u=> u.unsubscribe());
        new_packet_event.then(u=> u.unsubscribe());
    })
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
                    clean_button(),
                    truncate_button(),
                    right_panel()

                ])
            ]),
        ]
        );
    }


    const right_panel = () =>
    {
        return h('div',
        {
            style:{
                width: '100%',
                display: 'flex',
                flexDirection: 'row-reverse',
                alignItems: 'center',

            } as CSSProperties
        },
        h(NTooltip,{placement: 'left'},
        {
            trigger:() =>
            h(NAvatar,
            {
                size: 30,
                src: store.getState().server_is_online ? online_ico : offline_ico,
                class: 'hover-button',
                style:
                {
                    backgroundColor: 'transparent',
                    marginRight: '5px',
                    minWidth: '50px'
                }   as CSSProperties
                
            }),
            default:() => store.getState().server_is_online ? "Сервер онлайн" : "Нет соединения с сервером!",
        }))
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
                    await commands_service.clean_dirs();
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

    const truncate_button = () => 
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
                    const result = await commands_service.truncate_tasks_excepts();
                    if (result.is_ok())
                    {
                        naive_notify(notify, 'success', "Обрезка файлов задач успешно завершена", "Найдено и удалено " + result.get_value() + " несовпадающих записей");
                    }
                    else
                    {
                        naive_notify(notify, 'error', "Ошибка обрезки файла задачи", result.get_error());
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
                    src: cut_ico,
                    class: 'hover-button',
                    style:
                    {
                        backgroundColor: 'transparent',
                        marginRight: '5px',
                        minWidth: '50px'
                    }   as CSSProperties
                    
                }),
            }),
            default:() => in_work.value ? "Обрезка файлов задач запущена, ожидайте" : "Начать обрезку файлов задач",
        })
    }            

    return {list}
    },
    render ()
    {
        return this.list()
    }
})