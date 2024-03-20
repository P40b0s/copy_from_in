import { AddCircleOutline, Close, Home, RemoveOutline } from "@vicons/ionicons5";
import { FormInst, FormItemRule, FormRules, NButton, NCard, NDatePicker, NDynamicInput, NForm, NFormItem, NInput, NInputGroup, NInputNumber, NScrollbar, NSelect, NSpin, NTooltip, SelectGroupOption, SelectOption } from "naive-ui";
import { CSSProperties, PropType, VNode, defineAsyncComponent, defineComponent, defineModel, defineProps, defineEmits, defineSlots, h, ref, toRaw, Ref, watch } from "vue";
import { ComponentType, Disease, Id, Phones, User, } from "../../models/user.ts";
import { TypesBuilder  } from "../../services/data.ts";
import { DateFormat, DateTime, dateToString, parseDate, parseDateObj, parseDateObj2 } from "../../services/date.ts";
import { TauriCommands } from "../../services/tauri.ts";
import app_state_store from "../../store/app_state_store.ts";
import emitter from "../../services/emit.ts";



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

const rules = () : FormRules =>
{
    return {
        name_1: 
        {
            type: 'string',
            required: true,
            trigger: ['change', 'focus'],
            message: 'Необходимо ввести имя'
        },
        name_2: 
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
                type: 'number',
                required: true,
                validator (rule: FormItemRule, value: number) 
                {
                    return value > 0
                },
                trigger: ['change', 'focus'],
                message: 'Необходимо выбрать должность'
            },
            departmentId: 
            {
                type: 'number',
                validator (rule: FormItemRule, value: number) 
                {
                    return value > 0
                },
                required: true,
                trigger: ['change', 'focus'],
                message: 'Необходимо выбрать отдел'
            },
        }
    }
}

export const OrderEditorAsync = defineAsyncComponent({
    loader: () => import ('./ordered_editor.tsx'),
    loadingComponent: h(NSpin)
})


export const OrderedEditor = defineComponent({
props: localProps,
setup(props, {slots}) 
{
    const ordered = ref(structuredClone(toRaw(props.user.ordered)));
    const formRef = ref<FormInst | null>(null)

    async function save_form(): Promise<boolean>
    {
        const app_state = await TauriCommands.Statuses.update_ordered(ordered.value, props.user.id);
        if (app_state)
        {
            app_state_store.set_app_state = app_state;
            emitter.emit('userUpdated');
            return true;
        }
        else
        {
            console.error("Ошибка при сохранении списка командировок для юзера  " + ordered.value[0].userId);
            return false;
        }
    }
    async function validate(): Promise<boolean>
    {
        return true;
    }

    const dynamic_editor = () =>
    {
        return h(NDynamicInput,
            {
                value: ordered.value,
                onRemove:(r) => ordered.value.splice(r, 1),
                onCreate:(c) => ordered.value.splice(0, 0, TypesBuilder.build_ordered(props.user.id, new DateTime()))
            },
            {
                default:({ value }: {value: Ordered}) =>
                h('div',
                    {
                        style: 
                        {
                            display: 'flex',
                            flexDirection: 'column',
                            alignItems: 'center',
                            gap: '2px',
                            width: '100%'
                        } as CSSProperties
                    },
                    [
                        place_input(value),
                        h('div',
                        {
                                style: 
                                {
                                    display: 'flex',
                                    flexDirection: 'row',
                                    alignItems: 'center',
                                    gap: '2px',
                                    width: '100%'
                                } as CSSProperties
                        },
                        [
                            h(NDatePicker,
                            {
                                type: 'date',
                                placeholder: "начало",
                                clearable: false,
                                formattedValue: new DateTime(value.startDate).to_string(DateFormat.CalendarFormat),
                                valueFormat: DateFormat.CalendarFormat.toString(),
                                format: DateFormat.CalendarFormat.toString(),
                                onUpdateFormattedValue:(val) => value.startDate = new DateTime(val).to_string(DateFormat.SerializedDateTime)
                            }),
                            h(NDatePicker,
                            {
                                type: 'date',
                                placeholder: "окончание",
                                clearable: false,
                                formattedValue: new DateTime(value.endDate).to_string(DateFormat.CalendarFormat),
                                valueFormat: DateFormat.CalendarFormat.toString(),
                                format: DateFormat.CalendarFormat.toString(),
                                onUpdateFormattedValue:(val) => value.endDate = new DateTime(val).to_string(DateFormat.SerializedDateTime)
                            }),
                        ])
                       
                    ])
            })
    }

    const place_input = (value: Ordered) =>
    {
        return h(NInput,
        {
            value: value.place,
            onUpdateValue:(v)=> value.place = v,
            placeholder: "Введите место"
        },
        {
            
        })
        
        
    }
    // const editor = () => 
    // {
    //     if(slots?.default)
    //     {
    //         return slots.default()
    //     }
    //     else
    //     return h(NForm,
    //         {
    //             rules: rules(),
    //             ref: formRef,
    //             labelPlacement: 'top',
    //             model: vac_list.value
    //         },
    //         {
    //             default:() =>
    //         [
    //             h('div',
    //             [
    //                 vac_list.value.filter(f=>f.id == props.user.id).map(d=>
    //                 {
    //                     return h(NCard,
    //                     {
    //                         hoverable: true,
    //                         style:
    //                         {
    //                             maxWidth: '500px',
    //                             width: '500px'
    //                         }   as CSSProperties
    //                     },
    //                     {
    //                         header:() =>  h(NFormItem,
    //                         {
    //                             path: 'place',
    //                             label: "Место проведения отпуска",
    //                             labelStyle: props.styles
    //                         },
    //                         {
    //                             default:() => h(NInputGroup,
    //                             {
                                    
    //                             },
    //                             {
    //                                 default:() =>
    //                                 [
    //                                     h(NInput,
    //                                     {
    //                                         value: d.place,
    //                                         onUpdateValue:(v)=> d.place = v
    //                                     }),
    //                                     h(NTooltip,
    //                                         {
    //                                             placement:'right'
    //                                         },
    //                                         {
    //                                             trigger:() =>
    //                                             h(NButton,
    //                                             {
    //                                                 type: 'primary',
    //                                                 round: true,
    //                                                 text: true,
    //                                                 size: 'large',
    //                                                 textColor: 'green',
    //                                                 style:
    //                                                 {
    //                                                     fontSize: '24px'
    //                                                 } as CSSProperties,
    //                                                 onClick:() =>
    //                                                 {
    //                                                     d.place = props.user.livePlace + " (дома)";
    //                                                 } 
    //                                             },
    //                                             {
    //                                                 icon:()=> h(Home)
    //                                             }),
    //                                             default:()=> "Выбрать место постоянного проживания",
    //                                         }),
    //                                 ]
    //                             })
    //                         }),
    //                         "header-extra":() => h(NTooltip,
    //                         {
    //                             placement:'right'
    //                         },
    //                         {
    //                             trigger:() =>
    //                             h(NButton,
    //                             {
    //                                 type: 'primary',
    //                                 round: true,
    //                                 text: true,
    //                                 size: 'large',
    //                                 textColor: 'red',
    //                                 style:
    //                                 {
    //                                     marginTop: '-90px',
    //                                     marginRight: '-18px'
    //                                 } as CSSProperties,
    //                                 onClick:() =>
    //                                 {
    //                                     const ind = vac_list.value.findIndex(f=> f.id == d.id && f.startDate == d.startDate);
    //                                     vac_list.value.splice(ind, 1);
    //                                 } 
    //                             },
    //                             {
    //                                 icon:()=> h(Close)
    //                             }),
    //                             default:()=> "Удалить отпуск",
    //                         }),
    //                         default:() => h('div',
    //                         [
    //                             h(NFormItem,
    //                             {
    //                                 path: 'start_dis',
    //                                 label: "Дата начала отпуска",
    //                                 labelStyle: props.styles
    //                             },
    //                             {
    //                                 default:() =>
    //                                 h(NDatePicker,
    //                                 {
    //                                     type: 'date',
    //                                     clearable: true,
    //                                     formattedValue: d.startDate,
    //                                     valueFormat: 'dd.MM.yyyy',
    //                                     format: 'dd.MM.yyyy',
    //                                     onUpdateFormattedValue:(val) => d.startDate = val
    //                                 })
    //                             }),
    //                             h(NFormItem,
    //                             {
    //                                 path: 'start_dis',
    //                                 label: "Дата окончания отпуска",
    //                                 labelStyle: props.styles
    //                             },
    //                             {
    //                                 default:() =>
    //                                 h(NDatePicker,
    //                                 {
    //                                     type: 'date',
    //                                     clearable: true,
    //                                     formattedValue: d.endDate,
    //                                     valueFormat: 'dd.MM.yyyy',
    //                                     format: 'dd.MM.yyyy',
    //                                     onUpdateFormattedValue:(val) => d.endDate = val
    //                                 })
    //                             }),
    //                         ])
    //                     })
                    
    //                 }),
    //                 h(NTooltip,
    //                 {
    //                     placement:'right'
    //                 },
    //                 {
    //                     trigger:() =>
    //                     h(NButton,
    //                     {
    //                         type: 'success',
    //                         round: true,
    //                         text: true,
    //                         size: 'large',
    //                         textColor: 'green',
    //                         onClick:() =>
    //                         {
    //                             let vac = TypesBuilder.build_vacation(props.user.id);
    //                             vac_list.value.push(vac);
    //                         } 
    //                     },
    //                     {
    //                         icon:()=> h(AddCircleOutline),
    //                         default:() => "Добавить отпуск",
    //                     }),
    //                     default:()=> "Добавить отпуск",
    //                 }),
    //             ]),
    //         slots.save ? slots.save({items: vac_list.value, form: formRef.value}): [],
    //     ],
    // })
    //}
        return {dynamic_editor, save_form, validate};
    },
    render()
    {
        return this.dynamic_editor();
    }
});