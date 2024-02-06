import { AddCircleOutline, Close, Home, RemoveOutline } from "@vicons/ionicons5";
import { FormInst, FormItemRule, FormRules, NButton, NCard, NDatePicker, NDynamicInput, NForm, NFormItem, NInput, NInputGroup, NInputNumber, NProgress, NScrollbar, NSelect, NSpin, NTooltip, SelectGroupOption, SelectOption } from "naive-ui";
import { CSSProperties, PropType, VNode, defineAsyncComponent, defineComponent, defineModel, defineProps, defineEmits, defineSlots, h, ref, toRaw, Ref, watch } from "vue";
import { ComponentType, Disease, Id, Phones, Status, User, UserStatusType} from "../../models/user.ts";
import {  updateDiseases, updateUser,  TypesBuilder } from "../../services/data.ts";
import { DateFormat, DateTime, dateToString, getDaysDiff, parseDate, parseDateObj, parseDateObj2 } from "../../services/date.ts";
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

const statuses_list = (): Array<SelectOption | SelectGroupOption> =>
{
    return [
        {
            label: "Отпуск",
            value: UserStatusType.Vacation,
            disabled: false
        },
        {
            label: "Командировка",
            value: UserStatusType.Ordered,
            disabled: false
        },
        {
            label: "Распоряжение",
            value: UserStatusType.Trip,
            disabled: false
        }
    ]
    
}

// const rules = () : FormRules =>
// {
//     return {
//         name_1: 
//         {
//             type: 'string',
//             required: true,
//             trigger: ['change', 'focus'],
//             message: 'Необходимо ввести имя'
//         },
//         name_2: 
//         {
//             type: 'string',
//             required: true,
//             trigger: ['change', 'focus'],
//             message: 'Необходимо ввести отчество'
//         },
//         surname: 
//         {
//             type: 'string',
//             required: true,
//             trigger: ['change', 'focus'],
//             message: 'Необходимо ввести фамилию'
//         },
//         rank: 
//         {
//             type: 'string',
//             required: true,
//             trigger: ['change', 'focus'],
//             message: 'Необходимо выбрать звание'
//         },
//         livePlace: 
//         {
//             type: 'string',
//             required: true,
//             trigger: ['change', 'focus'],
//             message: 'Необходимо заполнить место постоянного проживания'
//         },
//         post:{
//             postId: 
//             {
//                 type: 'number',
//                 required: true,
//                 validator (rule: FormItemRule, value: number) 
//                 {
//                     return value > 0
//                 },
//                 trigger: ['change', 'focus'],
//                 message: 'Необходимо выбрать должность'
//             },
//             departmentId: 
//             {
//                 type: 'number',
//                 validator (rule: FormItemRule, value: number) 
//                 {
//                     return value > 0
//                 },
//                 required: true,
//                 trigger: ['change', 'focus'],
//                 message: 'Необходимо выбрать отдел'
//             },
//         }
//     }
// }

export const StatusEditorAsync = defineAsyncComponent({
    loader: () => import ('./status_editor.tsx'),
    loadingComponent: h(NSpin)
})


export const StatusEditor = defineComponent({
props: localProps,
setup(props, {slots}) 
{
    const statuses = ref(structuredClone(toRaw(props.user.statuses)));
    const formRef = ref<FormInst | null>(null)

    async function save_form(): Promise<boolean>
    {
        const app_state = await TauriCommands.Statuses.update_statuses(statuses.value, props.user.id);
        if (app_state)
        {
            app_state_store.set_app_state = app_state;
            emitter.emit('userUpdated');
            return true;
        }
        else
        {
            console.error("Ошибка при сохранении списка для юзера  " + statuses.value[0].userId);
            return false;
        }
    }
    async function validate(): Promise<boolean>
    {
        return true;
    }
    //TODO нужна фильтрация по типу статуса!
    const dynamic_editor = () =>
    {
        return h(NDynamicInput,
            {
                value: statuses.value,
                onRemove:(r) => statuses.value.splice(r, 1),
                onCreate:(c) => statuses.value.splice(0, 0, TypesBuilder.build_status(props.user.id, UserStatusType.Vacation, new DateTime()))
            },
            {
                default:({ value }: {value: Status}) =>
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
                        h(NSelect,
                        {
                            options: statuses_list(),
                            onUpdateValue:(v: string)=>
                            {
                                user.value.rank = get_dict_value(ranks_system.value, v);
                            } 
                        }),
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
                                placeholder: "Начало отпуска",
                                clearable: false,
                                formattedValue: new DateTime(value.startDate).to_string(DateFormat.CalendarFormat),
                                valueFormat: DateFormat.CalendarFormat.toString(),
                                format: DateFormat.CalendarFormat.toString(),
                                onUpdateFormattedValue:(val) => 
                                {
                                    value.startDate = new DateTime(val).to_string(DateFormat.SerializedDateTime)
                                    value.endDate = new DateTime(value.startDate).calc_end_date(value.daysProgress.days, DateFormat.SerializedDateTime)
                                }
                            }),
                            h(NInputNumber, 
                            {
                                value: value.daysProgress.days,
                                min: 1,
                                max: 100,
                                placeholder: "кол-во дней",
                                onUpdateValue:(n)=> 
                                {
                                    if(n != null)
                                    {
                                        value.daysProgress.days = n;
                                        value.endDate = new DateTime(value.startDate).calc_end_date(value.daysProgress.days, DateFormat.SerializedDateTime)
                                    }
                                }
                            }),
                            h('div',
                            {
                                style: 
                                {
                                    flexShrink: 0,
                                    background: new DateTime(value.endDate).date.getTime() < new Date().setHours(0, 0, 0, 0) ? 'red' : 'green',
                                    fontSize: '18px',
                                    verticalAlign: 'center',
                                    height: '100%'
                                } as CSSProperties
                            },
                            " по " + value.endDate
                            ),
                        ]),
                    ])
            })
    }

    const place_input = (value: Status) =>
    {
        return h(NInput,
        {
            value: value.place,
            onUpdateValue:(v)=> value.place = v,
            placeholder: "Введите место пребывания"
        },
        {
            prefix:() =>
            h(NTooltip,
                {
                    placement:'top'
                },
                {
                    trigger:() =>
                    h(NButton,
                    {
                        type: 'primary',
                        round: true,
                        text: true,
                        size: 'large',
                        textColor: 'green',
                        style:
                        {
                            fontSize: '24px'
                        } as CSSProperties,
                        onClick:() =>
                        {
                            value.place = props.user.livePlace + " (дома)";
                        } 
                    },
                    {
                        icon:()=> h(Home)
                    }),
                    default:()=> "Выбрать место постоянного проживания",
                }),
        })
        
        
    }
        return {dynamic_editor, save_form, validate};
    },
    render()
    {
        return this.dynamic_editor();
    }
});