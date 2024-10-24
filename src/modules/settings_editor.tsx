import 
{
    h,
    defineComponent,
    defineAsyncComponent,
    CSSProperties,
    onMounted,
    ref,
  } from 'vue'
import { open } from '@tauri-apps/api/dialog';
import { FormInst, FormItemRule, FormRules, NButton, NCard, NColorPicker, NDynamicInput, NForm, NFormItem, NIcon, NInput, NInputNumber, NPopconfirm, NScrollbar, NSelect, NSpin, NSwitch, NTooltip, SelectGroupOption, SelectOption} from 'naive-ui';
import { CopyModifer, Task, VN, taskClone} from '../models/types.ts';
import { settings } from '../services/tauri-service.ts';
import { AddSharp, CheckmarkCircleOutline, FolderOpenOutline, TrashBin } from '@vicons/ionicons5';
import { HeaderWithDescription } from './header_with_description.tsx';
import { Filter } from '../models/types';
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
        const is_new_task = ref(false);
        const save_error = ref<string|undefined>();
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
                    error(),
                    settings_dashboard(),
                    
                ]
                
                ),
            ]
            );
        }
        const error = () =>
        {
            return h("span", {
            style:
                {
                    color: 'red',
                    visibility: save_error.value ? 'visible' : 'collapse'
                } as CSSProperties
            },
            save_error.value
            )
        }

        const settings_selector = () =>
        {
            const save_button_label = ref<string | VN>("СОХРАНИТЬ");
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
                                target_dir: "",
                                report_dir: "",
                                timer: 120000,
                                delete_after_copy: false,
                                copy_modifier: "CopyAll",
                                is_active: true,
                                generate_exclude_file: true,
                                clean_types: [],
                                color: '#4f46',
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
                h(NButton,
                {
                    type: 'primary',
                    style:
                    {
                        marginLeft: '5px',
                        width: '100px'
                    }    as CSSProperties,
                    onClick: async () => 
                    {
                        const saved = tasks.value.findIndex(t=>t.name == selected_task.value?.name);
                        tasks.value.splice(saved, 1, selected_task.value as Task);
                        const result = await settings.save_task(tasks.value[saved]);
                        if (result === 'string')
                        {
                            console.error(result);
                            save_error.value = result;
                        }
                        else
                        {
                            is_new_task.value = false;
                            //await notify("Настройки успешно сохранены", "Настройки успешно сохранены")
                            (selected_task.value as Task).generate_exclude_file = false;
                            tasks.value[saved].generate_exclude_file = false;
                            save_button_label.value = h(NIcon, {component: CheckmarkCircleOutline, color: 'green', size: 'large'})
                            setTimeout(() => 
                            {
                                save_button_label.value = "СОХРАНИТЬ"
                            }, 1000);
                              
                        }
                    }
                },
                {
                    default:() => save_button_label.value
                }),
            ]
            )
        }
        const formRef = ref<FormInst | null>(null);
        const settings_dashboard = () =>
        {
            if(selected_task.value != undefined)
            {
                return h(NCard,{
                    style:
                    {
                        marginTop:'5px'
                    } as CSSProperties

                },
                    h(NScrollbar,
                    {
                        trigger: 'hover',
                        style:
                        {
                            maxHeight: '570px',
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
                            right_form(),
                            del_button(),
                            
                        ])
                    }))
            } else return [];
        }

        const del_button =() =>
        {
            return h(NPopconfirm,
            {
                positiveText: "Удалить",
                onPositiveClick: async () => 
                {
                    const current_task = tasks.value.findIndex(t=> t.name == selected_task.value?.name)
                    tasks.value.splice(current_task, 1);
                    let dl = await settings.delete_task(selected_task.value as Task)
                    selected_task.value = tasks.value[0];
                    is_new_task.value = false;
                }
            },
            {
                trigger:() =>  h(NTooltip,null,
                {
                    trigger:() =>  h(NButton,
                    {
                        type: 'error',
                        color: "#d90d0d",
                        size: 'large',
                        text: true,
                        style:
                        {
                            position: 'absolute',
                            top: '15px',
                            right: '15px'
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
                       // width: '600px',
                        flexGrow: '3',
                        marginTop:'5px'
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
                            readonly: true,
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
                                        // Open a selection dialog for image files
                                        const selected = await open({
                                            multiple: false,
                                            title: "Выбор исходной директории",
                                            defaultPath: selected_task.value?.source_dir,
                                            directory: true,
                                            });
                                            if(selected != null)
                                            {
                                                (selected_task.value as Task).source_dir = selected as string
                                            }
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
                        default:() =>
                        h(NInput,
                        {
                            readonly: true,
                            value: selected_task.value?.target_dir,
                            onUpdateValue:(v)=> (selected_task.value as Task).target_dir = v
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
                                        const selected = await open({
                                            multiple: false,
                                            title: "Выбор целевой директории",
                                            defaultPath: selected_task.value?.target_dir,
                                            directory: true,
                                            });
                                            if(selected != null)
                                            {
                                                (selected_task.value as Task).target_dir = selected as string
                                            }
                                    }
                                },
                                {
                                    icon:() => h(NIcon, {component:  FolderOpenOutline})
                                })
                        })
                    }),
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
                            readonly: true,
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
                                        const selected = await open({
                                            multiple: false,
                                            title: "Выбор директории отправки уведомлений",
                                            defaultPath: selected_task.value?.report_dir,
                                            directory: true,
                                            });
                                            if(selected != null)
                                            {
                                                (selected_task.value as Task).report_dir = selected as string
                                            }
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
                        marginLeft: '5px'
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
            ]})
        }
        return {list}
    },
    render ()
    {
        return this.list()
    }
})