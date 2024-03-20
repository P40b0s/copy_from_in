import { AddCircleOutline, Close, RemoveOutline } from "@vicons/ionicons5";
import { FormInst, FormItemRule, FormRules, NButton, NCard, NCheckbox, NDatePicker, NDynamicInput, NForm, NFormItem, NInput, NScrollbar, NSelect, NSpin, NTooltip, SelectGroupOption, SelectOption } from "naive-ui";
import { CSSProperties, PropType, VNode, defineAsyncComponent, defineComponent, defineModel, defineProps, defineEmits, defineSlots, h, ref, toRaw, Ref, watch } from "vue";
import { ComponentType, Dictionary, Disease, Id, Phones, User, get_dict_value } from "../../models/user.ts";
import { updateDiseases, updateUser } from "../../services/data.ts";
import { validateForm } from "./types.ts";
import { departments, posts, ranks as ranks_system } from "../../services/dictionaries.ts";


const localProps = 
{
    /**Человек */
    user: 
    {
        type: Object as PropType<User>,
        required: true
    },
    /**стили полей редактора */
    styles: 
    {
        type: Object as PropType<CSSProperties>,
        required: true
    },
} as const
const departmens_list = (): Array<SelectOption | SelectGroupOption> =>
{
    return departments.value.map(m=>
    {
        return {
                    label: m.name,
                    value: m.id,
                    disabled: false
                }
    })
}
const ranks = (): Array<SelectOption | SelectGroupOption> =>
{
    return ranks_system.value.map(r=>
    {
        return {
            label: r.name,
            value: r.id,
            disabled: false
        }
    })
}

const posts_option = (): Array<SelectOption | SelectGroupOption> =>
{
    return posts.value.map(c=>
    {
        return {
            label: c.name,
            value: c.id,
            disabled: false
        }
    })
}
const rules = () : FormRules =>
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

export const UserInfoEditorAsync = defineAsyncComponent({
    loader: () => import ('./user_info_editor.tsx'),
    loadingComponent: h(NSpin)
})
// const props = defineProps({
//     /**Человек */
//     user: 
//     {
//         type: Object as PropType<User>,
//         required: true
//     },
//     /**стили полей редактора */
//     styles: 
//     {
//         type: Object as PropType<CSSProperties>,
//         required: true
//     },
// })
export const UserInfoEditor = defineComponent({
props: localProps,
setup(props, {slots}) 
{
    async function save_form(): Promise<boolean>
    {
        const v = await validate();
        if(v)
            updateUser(user.value)
        return v;
    }
    async function validate(): Promise<boolean>
    {
        return await validateForm(formRef.value);
    }
    const user = ref(structuredClone(toRaw(props.user)));
    const formRef = ref<FormInst | null>(null);
    const phones_editor = (phones: Phones[]) =>
    {
         
        return h(NFormItem,
            {
                path: 'phones',
                label: "Телефоны",
                labelStyle: props.styles
            },
            {
                default:() =>
                h(NDynamicInput,
                {
                    value: phones,
                    onRemove:(r) => phones.splice(r, 1),
                    onCreate:(c) => 
                    {
                        const empty_phone: Phones = {phoneNumber: "", phoneType: "", isMain: false};
                        phones.push(empty_phone);
                    }
                },
                {
                    "create-button-default":() => "Добавить телефон",
                    default:({ value }: {value: Phones}) =>
                    h('div',
                        {
                            style: 
                            {
                                display: 'flex',
                                alignItems: 'center',
                                width: '100%',
                                gap: '5px'
                            } as CSSProperties
                        },
                        [
                            h(NInput, 
                            {
                                type: 'text',
                                value: value.phoneType,
                                placeholder: "Введите вид телефона",
                                onUpdateValue:(t) => value.phoneType = t
                            }),
                            h(NInput, {
                                type: 'text',
                                value: value.phoneNumber,
                                placeholder: "Введите номер телефона",
                                onUpdateValue:(t) => value.phoneNumber = t
                            }),
                            h(NTooltip,{},
                            {
                                default:()=> "Телефон является основным",
                                trigger:()=>
                                h(NCheckbox,
                                {
                                    checked: value.isMain,
                                    onUpdateChecked:(v)=>
                                    {
                                        console.log(v)
                                        value.isMain = v;
                                    },
                                }),
                            }),
                        ])
                })
                
            })
    }
    const editor = () => 
    {
        return slots.default?.() ?? [ 
            h(NForm,
            {
                rules: rules(),
                ref: formRef,
                labelPlacement: 'top',
                model: user.value,
            },
            {
                default:() =>[   
                h(NFormItem,
                {
                    path: 'surname',
                    label: "Фамилия",
                    labelStyle: props.styles
                },
                {
                    default:() =>
                    h(NInput,
                    {
                        value: user.value.surname,
                        onUpdateValue:(v)=> user.value.surname = v
                    })
                }),
                h(NFormItem,
                {
                    path: 'name1',
                    label: "Имя",
                    labelStyle: props.styles
                },
                {
                    default:() =>
                    h(NInput,
                    {
                        value: user.value.name1,
                        onUpdateValue:(v)=> user.value.name1 = v
                    })
                }),
                h(NFormItem,
                {
                    path: 'name2',
                    label: "Отчество",
                    labelStyle: props.styles
                },
                {
                    default:() =>
                    h(NInput,
                    {
                        value: user.value.name2,
                        onUpdateValue:(v)=> user.value.name2 = v
                    })
                }),
                h(NFormItem,
                {
                    path: 'rank.id',
                    label: "Звание",
                    labelStyle: props.styles
                },
                {
                    default:() =>
                    h(NSelect,
                    {
                        value: user.value.rank.name,
                        options: ranks(),
                        onUpdateValue:(v: string)=>
                        {
                            user.value.rank = get_dict_value(ranks_system.value, v);
                        } 
                    })
                }),
                h(NFormItem,
                {
                    path: 'department.id',
                    label: "Отдел",
                    labelStyle: props.styles
                },
                {
                    default:() =>
                    h(NSelect,
                    {
                        value: user.value.department.name,
                        options: departmens_list(),
                        onUpdateValue:(v: string)=>
                        {
                            user.value.department = get_dict_value(departments.value, v);
                        } 
                    })
                }),
                h(NFormItem,
                {
                    path: 'post.id',
                    label: "Должность",
                    labelStyle: props.styles
                },
                {
                    default:() =>
                    h(NSelect,
                    {
                        value: user.value.post.name,
                        options: posts_option(),
                        onUpdateValue:(v: string)=>
                        {
                            user.value.post = get_dict_value(posts.value, v);
                        } 
                    })
                }),
                h(NFormItem,
                {
                    path: 'livePlace',
                    label: "Место жительства",
                    labelStyle: props.styles
                },
                {
                    default:() =>
                    h(NInput,
                    {
                        value: user.value.livePlace,
                        onUpdateValue:(v)=> user.value.livePlace = v
                    })
                }),
                phones_editor(user.value.phones)
            ]
            }),
            slots.save ? slots.save({items: user.value, form: formRef.value}): []
        ]
    }
        return {editor, save_form, validate};
    },
    render()
    {
        return this.editor()
    }
});