import 
{
    h,
    defineComponent,
    defineAsyncComponent,
    CSSProperties,
    onMounted,
    ref,
    onUnmounted,
  } from 'vue'
import { open } from '@tauri-apps/api/dialog';
import { FormInst, FormItemRule, FormRules, NButton, NCard, NColorPicker, NDynamicInput, NForm, NFormItem, NIcon, NInput, NInputNumber, NPopconfirm, NScrollbar, NSelect, NSpin, NSwitch, NTooltip, SelectGroupOption, SelectOption, useNotification} from 'naive-ui';
import { CopyModifer, Task, VN, taskClone} from '../models/types.ts';
import { commands_settings } from '../services/tauri/commands.ts';
import { events } from '../services/tauri/events.ts';
import { AddSharp, CheckmarkCircleOutline, FolderOpenOutline, TrashBin, WarningSharp } from '@vicons/ionicons5';
import { HeaderWithDescription } from './header_with_description.tsx';
import { Filter } from '../models/types';
import { naive_notify } from '../services/notification.ts';
import { sleepNow } from '../services/helpers.ts';

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
    async setup () 
    {
        const notify = useNotification();
        const tasks = ref<Task[]>([]);
        const selected_task = ref<Task>();
        const is_new_task = ref(false);
        const get_tasks = async () =>
        {
            let s = await commands_settings.load_settings()
            if (s.is_ok())
            {
                tasks.value = s.get_value();
                selected_task.value =  taskClone.clone(tasks.value[0]);
            }
            else
            {
                console.error(s.get_error());
            }
        }
        const updated_event = events.task_updated(async (task) => 
        {
            const new_task = task.payload;
            const saved = tasks.value.findIndex(t=>t.name == new_task.name);
            //новая задача
            if (saved == -1)
            {
                tasks.value.push(new_task);
                naive_notify(notify, 'success', "Добавлена задача " + new_task.name, "", 2000);
            }
            //задача уже есть в списке
            else
            {
                tasks.value.splice(saved, 1, new_task);
                if (selected_task.value && selected_task.value.name == new_task.name)
                {
                    selected_task.value = tasks.value[saved];
                }
                if(is_new_task.value == false)
                    naive_notify(notify, 'success', "Задача " + new_task.name + " была изменена", "", 2000);
                else
                    naive_notify(notify, 'success', "Задача " + new_task.name + " была сохранена", "", 2000);
            }
            is_new_task.value = false;
            (selected_task.value as Task).generate_exclude_file = false;
            tasks.value[saved].generate_exclude_file = false;
        })
        const delete_event = events.task_deleted(async (task) => 
        {
            const task_name = task.payload;
            const saved = tasks.value.findIndex(t=>t.name == task_name);
            if (saved != -1)
            {
                tasks.value.splice(saved, 1);
                naive_notify(notify, 'success', "Удалена задача " + task_name, "", 2000);
                selected_task.value = tasks.value[0];
            }
        })
        onUnmounted(()=>
        {
            updated_event.then(v=> v.unsubscribe())
            delete_event.then(t=>t.unsubscribe())
        })
        await get_tasks();
      
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

        const copy_modifers = (): Array<SelectOption | SelectGroupOption> =>
        {
            return [
                {
                    label: "Копировать все",
                    value: 'CopyAll',
                    disabled: false
                },
                {
                    label: "Копировать только",
                    value: 'CopyOnly',
                    disabled: false
                },
                {
                    label: "Копировать кроме",
                    value: 'CopyExcept',
                    disabled: false
                },
            ]
        }
      
        const list = () =>
        {
            return h('div',
            {
                style:
                {
                    display: 'flex',
                    flexDirection: 'row',
                    width: '100%',
                    height: '100%'
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
                    settings_dashboard(),
                    
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
                                description: "",
                                source_dir: "",
                                target_dirs: [""],
                                report_dir: "",
                                timer: 120000,
                                delete_after_copy: false,
                                copy_modifier: "CopyAll",
                                is_active: true,
                                generate_exclude_file: true,
                                clean_types: [],
                                sound: false,
                                autocleaning: false,
                                color: '#0ff00f',
                                visible: true,
                                filters: f
                            }
                            is_new_task.value = true;
                            tasks.value.push(task);
                            selected_task.value = tasks.value[tasks.value.length - 1];
                        }
                    },
                    {
                        icon:() => h(NIcon, {component: AddSharp})
                    }),
                    default:() => "Добавить новую задачу"
                }),
                h(NSelect,
                {
                    style:
                    {
                        marginLeft: '5px'
                    } as CSSProperties,
                    disabled: is_new_task.value,
                    value: selected_task.value?.name,
                    options: settings_names(),
                    onUpdateValue:(v: string)=>
                    {
                        selected_task.value = taskClone.clone(tasks.value.find(f=>f.name == v));
                    }
                }),
                save_button(),
                (selected_task.value != undefined && is_new_task.value == false) ? del_button() : [],
                (selected_task.value != undefined && is_new_task.value) ? cancel_button() : []
                
                
            ]
            )
        }

        const save_button = () :VN =>
        {
            const save_button_label = ref<string | VN>("СОХРАНИТЬ");
            return h(NButton,
            {
                type: 'primary',
                style:
                {
                    marginLeft: '5px',
                    width: '100px'
                }    as CSSProperties,
                onClick: async () => 
                {
                    if(selected_task.value != undefined)
                    {
                        const result = await commands_settings.save_task(selected_task.value);
                        if (result.is_err())
                        {
                            save_button_label.value = h(NIcon, {component: WarningSharp, color: 'red', size: 'large'})
                            const res = result.get_error().split("\\n");
                            if (res.length == 1)
                                naive_notify(notify, 'error', "Ошибка сохранения настроек", result.get_error());
                            else
                                naive_notify(notify, 'error', "Ошибка сохранения настроек", () => 
                            {
                                return h('div',null,
                                    res.map(r=> h('div', 
                                    {
                                        style:{
                                            color: 'red'
                                        } as CSSProperties
                                    },
                                    r))
                                );
                            });   
                        }
                    }
                }
            },
            {
                default:() => save_button_label.value
            });
        }
        const cancel_button = () :VN =>
        {
            const save_button_label = ref<string | VN>("ОТМЕНА");
            return h(NButton,
            {
                type: 'warning',
                style:
                {
                    marginLeft: '5px',
                    width: '100px',
                }    as CSSProperties,
                onClick: async () => 
                {
                    selected_task.value = tasks.value[0];
                    tasks.value.pop();
                    is_new_task.value = false;
                }
            },
            {
                default:() => save_button_label.value
            });
        }

        const formRef = ref<FormInst | null>(null);
        const settings_dashboard = () =>
        {
            if(selected_task.value != undefined)
            {
                return h(NCard,
                {
                    style:
                    {
                        marginTop:'5px'
                    } as CSSProperties

                },
                    () => h(NScrollbar,
                    {
                        trigger: 'hover',
                        
                        style:
                        {
                            maxHeight: '78vh',
                            padding: '10px'
                        } as CSSProperties
                    },
                    {
                        default:() =>  h('div', 
                        {
                            style:
                            {
                                display: 'flex',
                                height: '100%',
                                justifyContent: 'space-between',
                                flexDirection: 'row',
                                
                            } as CSSProperties
                        },
                        [
                            left_form(),
                            h('div',   {style:
                                {
                                    flexGrow: '2',
                                    marginTop:'5px'
                                } as CSSProperties}),
                            right_form(),
                        ])
                    })
                )
            } else return [];
        }

        const del_button =() =>
        {
            return h(NPopconfirm,
            {
                style:
                {
                   
                } as CSSProperties,
                positiveText: "Удалить",
                onPositiveClick: async () => 
                {
                    //const current_task = tasks.value.findIndex(t=> t.name == selected_task.value?.name)
                    //tasks.value.splice(current_task, 1);
                    let dl = await commands_settings.delete_task(selected_task.value as Task)
                    if (dl.is_err())
                    {
                        naive_notify(notify, 'error', "Ошибка удаления задачи " + selected_task.value?.name, () => 
                        {
                            return h('div', 
                            {
                                style:
                                {
                                    color: 'red'
                                } as CSSProperties,
                            },
                            dl.get_error()
                            );
                        });
                    }
                }
            },
            {
                trigger:() =>  h(NTooltip,null,
                {
                    trigger:() =>  h(NButton,
                    {
                        type: 'error',
                        color: "#d90d0d",
                        style:
                        {
                            marginLeft: '5px',
                        }    as CSSProperties,
                    },
                    {
                        icon:() => h(NIcon, {component: TrashBin})
                    }),
                    default:() => "Удалить задачу"
                }),
                default:() => "Вы хотите удалить задачу " + selected_task.value?.name + "?"
            })
        }

        const left_form = () =>
        {
            return h(NForm,
                {
                    rules: form_validation_rules(),
                    ref: formRef,
                    labelPlacement: 'top',
                    model: selected_task.value,
                    style:
                    {
                        flexGrow: '3',
                        marginTop:'5px',
                    } as CSSProperties
                },
                {
                    default:() =>[   
                    h(NFormItem,
                    {
                        path: 'name',
                    },
                    {
                        label:() => h(HeaderWithDescription,{
                            name: "Имя задачи",
                            description: "Наименование тещуей задачи, изменяется только при создании новой задачи, потом не может быть изменено",
                            fontSize: '14px'
                        }),
                        default:() =>
                        h(NInput,
                        {
                            disabled: !is_new_task.value,
                            value: selected_task.value?.name,
                            onUpdateValue:(v)=> (selected_task.value as Task).name = v
                        })
                    }),
                    h(NFormItem,
                    {
                        path: 'descr',
                    },
                    {
                        label:() => h(HeaderWithDescription,{
                            name: "Описание",
                            description: "Краткое описание для чего создана задача",
                            fontSize: '14px'
                        }),
                        default:() =>
                        h(NInput,
                        {
                            value: selected_task.value?.description,
                            onUpdateValue:(v)=> (selected_task.value as Task).description = v
                        })
                    }),
                    h(NFormItem,
                    {
                        path: 'sourcedir',
                    },
                    {
                        label:() => h(HeaderWithDescription,{
                            name: "Исходная директория",
                            description: "Директория в которой будут отслеживаться новые пакеты, и при необходимости копироваться в целевую директорию",
                            fontSize: '14px'
                        }),
                        default:() =>
                        h(NInput,
                        {
                            readonly: false,
                            value: selected_task.value?.source_dir,
                            onUpdateValue:(v)=> (selected_task.value as Task).source_dir = v,
                        },
                        {
                            prefix: () =>
                            h(NButton,
                                {
                                    color: "#8a2be2",
                                    size: 'large',
                                    text: true,
                                    onClick: async ()=>
                                    {
                                        // Open a selection dialog for files
                                        // const selected = await open({
                                        //     multiple: false,
                                        //     title: "Выбор исходной директории",
                                        //     defaultPath: selected_task.value?.source_dir,
                                        //     directory: true,
                                        //     });
                                        //     if(selected != null)
                                        //     {
                                        //         (selected_task.value as Task).source_dir = selected as string
                                        //     }
                                    }
                                },
                                {
                                    icon:() => h(NIcon, {component:  FolderOpenOutline})
                                })
                        }),
                    }),
                    h(NFormItem,
                    {
                        path: 'targetdir',
                    },
                    {
                        label:() => h(HeaderWithDescription,{
                            name: "Целевая директория",
                            description: "Директория в которую будут копироваться пакеты",
                            fontSize: '14px'
                        }),
                        default:() =>  h(NDynamicInput,
                        {
                            value: selected_task.value?.target_dirs,
                            onUpdateValue(v)
                            {
                                (selected_task.value as Task).target_dirs = v as string[];
                            },
                            onCreate(index)
                            {
                                selected_task.value?.target_dirs.splice(index, 0, "")
                            },
                            onRemove(index) 
                            {
                                selected_task.value?.target_dirs.splice(index, 1);
                            },
                        }),
                    }),
                    // h(NFormItem,
                    // {
                    //     path: 'targetdir',
                    // },
                    // {
                    //     label:() => h(HeaderWithDescription,{
                    //         name: "Целевая директория",
                    //         description: "Директория в которую будут копироваться пакеты",
                    //         fontSize: '14px'
                    //     }),
                    //     default:() =>
                    //     h(NInput,
                    //     {
                    //         readonly: false,
                    //         value: selected_task.value?.target_dir,
                    //         onUpdateValue:(v)=> (selected_task.value as Task).target_dir = v
                    //     },
                    //     {
                    //         prefix: () =>
                    //         h(NButton,
                    //             {
                    //                 color: "#8a2be2",
                    //                 size: 'large',
                    //                 text: true,
                    //                 onClick: async ()=>
                    //                 {
                    //                     // Open a selection dialog for image files
                    //                     // const selected = await open({
                    //                     //     multiple: false,
                    //                     //     title: "Выбор целевой директории",
                    //                     //     defaultPath: selected_task.value?.target_dir,
                    //                     //     directory: true,
                    //                     //     });
                    //                     //     if(selected != null)
                    //                     //     {
                    //                     //         (selected_task.value as Task).target_dir = selected as string
                    //                     //     }
                    //                 }
                    //             },
                    //             {
                    //                 icon:() => h(NIcon, {component:  FolderOpenOutline})
                    //             })
                    //     })
                    // }),
                    h(NFormItem,
                    {
                        path: 'reportdir',
                    },
                    {
                        label:() => h(HeaderWithDescription,{
                            name: "Директория для отправки уведомлений",
                            description: "Если указана директория то по пакетам поступившим по этой задаче будут автоматически формироваться уведомления о доставке и отправлятся оправившему пакет органу",
                            fontSize: '14px'
                        }),
                        default:() =>
                        h(NInput,
                        {
                            readonly: false,
                            value: selected_task.value?.report_dir,
                            onUpdateValue:(v)=> (selected_task.value as Task).report_dir = v
                        },
                        {
                            prefix: () =>
                            h(NButton,
                                {
                                    color: "#8a2be2",
                                    size: 'large',
                                    text: true,
                                    onClick: async ()=>
                                    {
                                        // Open a selection dialog for image files
                                        // const selected = await open({
                                        //     multiple: false,
                                        //     title: "Выбор директории отправки уведомлений",
                                        //     defaultPath: selected_task.value?.report_dir,
                                        //     directory: true,
                                        //     });
                                        //     if(selected != null)
                                        //     {
                                        //         (selected_task.value as Task).report_dir = selected as string
                                        //     }
                                    }
                                },
                                {
                                    icon:() => h(NIcon, {component:  FolderOpenOutline})
                                })
                        })
                    }),
                    h(NFormItem,
                    {
                        path: 'cl_p',
                    },
                    {
                        label:() => h(HeaderWithDescription,{
                            name: "Типы пакетов для операции очистки",
                            description: "При нажатии кнопки с кисточкой начнется очистка исходных директорий от пакетов которые введены в этом поле",
                            fontSize: '14px'
                        }),
                        default:() =>  h(NDynamicInput,
                        {
                            value: selected_task.value?.clean_types,
                            onUpdateValue(v)
                            {
                                (selected_task.value as Task).clean_types = v as string[];
                            },
                            onCreate(index)
                            {
                                selected_task.value?.clean_types.splice(index, 0, "")
                            },
                            onRemove(index) 
                            {
                                selected_task.value?.clean_types.splice(index, 1);
                            },
                        }),
                    }),
                    h(NFormItem,
                    {
                        path: 'mod',
                    },
                    {
                        label:() => h(HeaderWithDescription,{
                            name: "Модификатор копирования",
                            description: "Копирование всех пакетов, или копирование пакетов согласно правилам фильтрации",
                            fontSize: '14px'
                        }),
                        default:() =>  h(NSelect,
                        {
                            value: selected_task.value?.copy_modifier,
                            options: copy_modifers(),
                            onUpdateValue:(v: CopyModifer)=>
                            {
                                (selected_task.value as Task).copy_modifier = v;
                            }
                        }),
                    }
                    ),
                    h(NFormItem,
                        {
                            path: 'fil_tp',
                            label: "Фильтрация по виду документа",
                            style:{
                                visibility: (selected_task.value as Task).copy_modifier == 'CopyAll' ? 'collapse' : 'visible' 
                            } as CSSProperties
                        },
                        {
                            label:() => h(HeaderWithDescription,{
                                name: "Фильтрация по виду документа",
                                description: "Вид документа который указан в xml файле пакета в тегах `<xdms:header xdms:type=\"Транспортный контейнер\"...`",
                                fontSize: '14px'
                            }),
                            default:() =>  h(NDynamicInput,
                            {
                                value: selected_task.value?.filters.document_types,
                                onUpdateValue(v)
                                {
                                    (selected_task.value as Task).filters.document_types = v as string[];
                                },
                                onCreate(index)
                                {
                                    selected_task.value?.filters.document_types.splice(index, 0, "")
                                },
                                onRemove(index) 
                                {
                                    selected_task.value?.filters.document_types.splice(index, 1);
                                },
                            }),
                        }
                    ),
                    h(NFormItem,
                        {
                            path: 'fil_uid',
                            style:{
                                visibility: (selected_task.value as Task).copy_modifier == 'CopyAll' ? 'collapse' : 'visible' 
                            } as CSSProperties
                        },
                        {
                            label:() => h(HeaderWithDescription,{
                                name: "Фильтрация по UID отправителя",
                                description: "UID отправителя который указан в xml файле пакета в тегах ` <xdms:source xdms:uid=\"db617a7c-bd8f-4159-afda-aabdbbcdba18\">`",
                                fontSize: '14px'
                            }),
                            default:() => h(NDynamicInput,
                            {
                                value: selected_task.value?.filters.document_uids,
                                onUpdateValue(v)
                                {
                                    (selected_task.value as Task).filters.document_uids = v as string[];
                                },
                                onCreate(index)
                                {
                                    selected_task.value?.filters.document_uids.splice(index, 0, "")
                                },
                                onRemove(index) 
                                {
                                    selected_task.value?.filters.document_uids.splice(index, 1);
                                },
                            }),
                        }
                        
                    )
                ]
                })
        }
        const right_form = () =>
        {
            return h(NForm,
                {
                    rules: form_validation_rules(),
                    ref: formRef,
                    labelPlacement: 'top',
                    model: selected_task.value,
                    style:
                    {
                        marginTop:'5px',
                        marginLeft: '5px',
                        marginRight: '10px'
                    } as CSSProperties
                },
                {
                    default:() =>[   
                    h(NFormItem,
                    {
                        path: 'isactive',
                    },
                    {
                        label:() => h(HeaderWithDescription,{
                            name: "Задача активна",
                            description: "Работают только активные задачи, неактивные задачи не выполняются",
                            fontSize: '14px'
                        }),
                        default:() =>
                        h(NSwitch,
                        {
                            value: selected_task.value?.is_active,
                            onUpdateValue:(v: boolean)=>
                            {
                                (selected_task.value as Task).is_active = v;
                            } 
                        })
                    }),
                    h(NFormItem,
                    {
                        path: 'dac',
                    },
                    {
                        label:() => h(HeaderWithDescription,{
                            name: "Удалять после копирования",
                            description: "После копирования пакета из исходной директории в целевую директорию, пакет удаляется из исходной директории",
                            fontSize: '14px'
                        }),
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
                    h(NFormItem,
                    {
                        path: 'timer',
                    },
                    {
                        label:() => h(HeaderWithDescription,{
                            name: "Переодичность сканирования",
                            description: "Интервал сканирования исходной директории при поиске новых пакетов, устанавливается кратным 15с.",
                            fontSize: '14px'
                        }),
                        default:() =>
                        h(NInputNumber,
                        {
                            value: ((selected_task.value as Task).timer / 1000),
                            min: 15,
                            step: 15,
                            onUpdateValue:(v)=> 
                            {
                                if (v)
                                {
                                    (selected_task.value as Task).timer = v* 1000;
                                }
                                else
                                {
                                    (selected_task.value as Task).timer = 1000;
                                }
                            }
                        })
                    }),
                    h(NFormItem,
                    {
                        path: 'exclud',
                    },
                    {
                        label:() => h(HeaderWithDescription,{
                            name: "Сгенерировать файл исключений",
                            description: "При сохранении настроек будет дополнительно сгенерирован файл исключений, в нем будут присутсвовать все имена пакетов из целевой директории, для исключения копирования уже существующих в системе пакетов",
                            fontSize: '14px'
                        }),
                        default:() =>
                        h(NSwitch,
                        {
                            value: selected_task.value?.generate_exclude_file,
                            onUpdateValue:(v: boolean)=>
                            {
                                (selected_task.value as Task).generate_exclude_file = v;
                            } 
                        })
                    }),
                    h(NFormItem,
                    {
                        path: 'clr',
                    },
                    {
                        label:() => h(HeaderWithDescription,{
                            name: "Цвет уведомления",
                            description: "У данного уведомления будет такой цвет при отображении в логе задач",
                            fontSize: '14px'
                        }),
                        default:() =>
                        h(NColorPicker,
                        {
                            value: selected_task.value?.color,
                            showAlpha: false,
                            onUpdateValue:(v: string)=>
                            {
                                (selected_task.value as Task).color = v;
                            } 
                        })
                    }),
                    
                    h(NFormItem,
                    {
                        path: 'autocln',
                    },
                    {
                        label:() => h(HeaderWithDescription,{
                            name: "Автоочистка",
                            description: "Пакеты указанные в поле \"Типы пакетов для операции очистки\" при поступлении будут удалятся автоматически",
                            fontSize: '14px'
                        }),
                        default:() =>
                        h(NSwitch,
                        {
                            value: selected_task.value?.autocleaning,
                            onUpdateValue:(v: boolean)=>
                            {
                                (selected_task.value as Task).autocleaning = v;
                            } 
                        })
                        }),
                        h(NFormItem,
                        {
                            path: 'visible',
                        },
                        {
                            label:() => h(HeaderWithDescription,{
                                name: "Отображение в списке пакетов",
                                description: "Отображать ли процесс данной задачи во вкладке \"Пакеты\"",
                                fontSize: '14px'
                            }),
                            default:() =>
                            h(NSwitch,
                            {
                                value: selected_task.value?.visible,
                                onUpdateValue:(v: boolean)=>
                                {
                                    (selected_task.value as Task).visible = v;
                                } 
                            })
                        }),
            ]})
        }
        return {list}
    },
    render ()
    {
        return this.list()
    }
})