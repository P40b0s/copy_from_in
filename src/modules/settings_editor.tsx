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

import { FormInst, FormItemRule, FormRules, NButton, NForm, NFormItem, NInput, NInputNumber, NSelect, NSpin, NSwitch, NVirtualList, SelectGroupOption, SelectOption} from 'naive-ui';
import { DateFormat, DateTime} from '../services/date.ts';
import { app_state_store } from '../store/index.ts';
import { StatusCard } from './status_card.tsx';
import { bell_ico, envelope_ico, error_ico } from '../services/svg.ts';
import { IPacket, Task, taskClone} from '../models/types.ts';
import { settings } from '../services/tauri-service.ts';
import { string } from 'ts-pattern/dist/patterns';
export const SettingsEditorAsync = defineAsyncComponent({
    loader: () => import ('./settings_editor.tsx'),
    loadingComponent: h(NSpin)
})


const form_validation_rules = () : FormRules =>
{
    return {
        name1: 
        {
            type: 'string',
            required: true,
            trigger: ['change', 'focus'],
            message: 'Необходимо ввести имя'
        },
        name2: 
        {
            type: 'string',
            required: true,
            trigger: ['change', 'focus'],
            message: 'Необходимо ввести отчество'
        },
        surname: 
        {
            type: 'string',
            required: true,
            trigger: ['change', 'focus'],
            message: 'Необходимо ввести фамилию'
        },
        rank: 
        {
            type: 'string',
            required: true,
            trigger: ['change', 'focus'],
            message: 'Необходимо выбрать звание'
        },
        livePlace: 
        {
            type: 'string',
            required: true,
            trigger: ['change', 'focus'],
            message: 'Необходимо заполнить место постоянного проживания'
        },
        post:{
            postId: 
            {
                type: 'string',
                required: true,
                validator (rule: FormItemRule, value: string) 
                {
                    return value.length > 0
                },
                trigger: ['change', 'focus'],
                message: 'Необходимо выбрать должность'
            },
            departmentId: 
            {
                type: 'string',
                validator (rule: FormItemRule, value: string) 
                {
                    return value.length > 0
                },
                required: true,
                trigger: ['change', 'focus'],
                message: 'Необходимо выбрать отдел'
            },
        }
    }
}
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
                    selected_task.value =  taskClone.clone(s[0]);
                }
                //console.log(tasks.value);
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
                        width: '100%',
                        display: 'flex',
                        flexDirection: 'column',
                    } as CSSProperties
                },
                [
                    settings_selector(),
                    settings_dashboard()
                ]
                
                ),
            ]
            );
        }

        const settings_selector = () =>
        {
            return h("div",
            {
                style:
                {
                    width: '100%',
                    display: 'flex',
                    flexDirection: 'row',
                    alignSelf: "start",
                    alignContent: 'start'
                } as CSSProperties
            },
            [
                h(NSelect,
                {
                    value: selected_task.value?.name,
                    options: settings_names(),
                    onUpdateValue:(v: string)=>
                    {
                        selected_task.value = taskClone.clone(tasks.value.find(f=>f.name == v));
                    }
                }),
                h(NButton,
                {
                    type: 'primary',
                    style:
                    {
                        marginLeft: '5px'
                    }    as CSSProperties
                },
                {
                    default:() => "СОХРАНИТЬ"
                }),
                h(NButton,
                {
                    type: 'error',
                    style:
                    {
                        marginLeft: '5px'
                    }    as CSSProperties
                },
                {
                    default:() => "ОТМЕНА"
                }),
            ]
            )
        }
        const formRef = ref<FormInst | null>(null);
        const settings_dashboard = () =>
        {
            if(selected_task.value != undefined)
            {
                return h(NForm,
                    {
                        rules: form_validation_rules(),
                        ref: formRef,
                        labelPlacement: 'top',
                        model: selected_task.value,
                    },
                    {
                        default:() =>[   
                        h(NFormItem,
                        {
                            path: 'name',
                            label: "Имя задачи",
                            
                        },
                        {
                            default:() =>
                            h(NInput,
                            {
                                value: selected_task.value?.name,
                                onUpdateValue:(v)=> (selected_task.value as Task).name = v
                            })
                        }),
                        h(NFormItem,
                        {
                            path: 'sourcedir',
                            label: "Исходная директория",
                        },
                        {
                            default:() =>
                            h(NInput,
                            {
                                value: selected_task.value?.source_dir,
                                onUpdateValue:(v)=> (selected_task.value as Task).source_dir = v
                            })
                        }),
                        h(NFormItem,
                        {
                            path: 'targetdir',
                            label: "Целевая директория",
                        },
                        {
                            default:() =>
                            h(NInput,
                            {
                                value: selected_task.value?.target_dir,
                                onUpdateValue:(v)=> (selected_task.value as Task).target_dir = v
                            })
                        }),
                        h(NFormItem,
                        {
                            path: 'timer',
                            label: "Переодичность сканирования (мс.)",
                        },
                        {
                            default:() =>
                            h(NInputNumber,
                            {
                                value: selected_task.value?.timer,
                                onUpdateValue:(v)=> (selected_task.value as Task).timer = v ?? 100000
                            })
                        }),
                        h(NFormItem,
                        {
                            path: 'dac',
                            label: "Удалять после копирования",
                        },
                        {
                            default:() =>
                            h(NSwitch,
                            {
                                value: selected_task.value?.delete_after_copy,
                                onUpdateValue:(v: boolean)=>
                                {
                                    (selected_task.value as Task).delete_after_copy = v;
                                } 
                            })
                        }),
                    ]
                    })
            } else return [];
        }
        return {list}
    },
    render ()
    {
        return this.list()
    }
})