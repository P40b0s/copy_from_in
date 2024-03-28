import 
{
    h,
    defineComponent,
    defineAsyncComponent,
    CSSProperties,
    ref,
  } from 'vue'
import { NAvatar, NButton, NSpin, NTooltip, useNotification} from 'naive-ui';
import { clean_ico, cut_ico} from '../services/svg.ts';
import { service } from '../services/tauri-service.ts';
import { naive_notify } from '../services/notification.ts';
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
                        clean_button(),
                        truncate_button()

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
                    const result = await service.truncate_tasks_excepts();
                    console.log(result)
                    if (result != undefined)
                    {
                        if(result === 'string')
                        {
                            naive_notify(notify_inj, 'error', "Ошибка обрезки файла задачи", result);
                        }
                        else
                            naive_notify(notify_inj, 'success', "Обрезка файлов задач успешно завершена", "Найдено и удалено " + result + " несовпадающих записей");
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