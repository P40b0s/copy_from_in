import 
{
    h,
    defineComponent,
    defineAsyncComponent,
    inject,
    onMounted,
    CSSProperties,
    PropType,
    toRefs,
    toRef,
    ref,
    VNode,
    RendererNode,
    RendererElement,
    toRaw,
    computed
  } from 'vue'

import { NAvatar, NButton, NCard, NCheckbox, NConfigProvider, NDatePicker, NDynamicInput, NForm, NFormItem, NInput, NInputGroup, NInputNumber, NModal, NScrollbar, NSelect, NSpin, NSwitch, NTab, NTabPane, NTable, NTabs, NTimePicker, NTooltip, SelectGroupOption, SelectOption } from 'naive-ui';
import { ComponentType, Disease, DiseaseTest, DiseaseType, EditorType, Journal, JournalItem, Dictionary, Phones, TimeWarning, User, Vacation, Vactination, enumKeys } from '../../models/user.ts';
import { dateToString, parseDate, parseDateObj, timeToString } from '../../services/date.ts';
import { AddCircleOutline, Close, Home, RemoveOutline } from '@vicons/ionicons5';
import { match } from 'ts-pattern';
import { ruRU, dateRuRU } from 'naive-ui'
import { sortTimeWarnings } from '../../services/data.ts';

const localProps = 
{
    value: 
    {
        type: Array as PropType<TimeWarning[]>,
        required: true
    },
    is_open: 
    {
        type: Boolean,
        required: true,
        default: false
    },
} as const

export const TimeWarningsEditorAsync = defineAsyncComponent({
    loader: () => import ('./time_warnings_editor.tsx'),
    loadingComponent: h(NSpin)
})

export const TimeWarningsEditor =  defineComponent({
props: localProps,
emits:
{
    'update:value': (value: TimeWarning[]) => value,
    'onClose': (v: boolean) => v
},
    setup (props, {emit}) 
    {
        //const values = ref(structuredClone(toRaw(props.value)));
        //const values = computed(() => props.value);
        //const t = Object.assign({}, props.value);
        const values = ref<TimeWarning[]>(sortTimeWarnings(props.value).map(o => new TimeWarning(o.text, o.time, o.weekDay, o.date, o._notifyBeforeTargetTime)));
        //Object.assign(values.value, toRaw(props.value))
        const modal = () =>
        {
            return h(NModal,
            {
                show: props.is_open,
                preset: 'dialog',
                closable: true,
                blockScroll: true,
                showIcon: false,
                "onUpdate:show":(v)=>
                {
                    //console.log("заново инициализировано значение планировщика событий!", v)
                    //if(v)
                    //    values.value = sortTimeWarnings(props.value).map(o => new TimeWarning(o.text, o.time, o.weekDay, o.date, o._notifyBeforeTargetTime));
                },
                style:
                {
                    minWidth: '1000px',
                    width: '1000px'
                } as CSSProperties,
            },
            {
                header:() =>
                h('div',
                {
                    style:
                    {
                        display: 'flex',
                        flexDirection: 'row',
                        gap: '10px'
                    } as CSSProperties,
                },
                [
                "Диспетчер задач",
                ]),
                default:()=> dynamic_editor(),
                action:() => 
                h('div',
                {
                    style: 
                    {
                        display: 'flex',
                        flexDirection: 'row',
                        alignItems: 'center',
                        width: '100%',
                        justifyContent: 'center'
                    } as CSSProperties
                },
                [
                    save_button(values.value)
                ]
                )
            })
        }
        const date_selector = (tw: TimeWarning) =>
        {
            return h(NTooltip,{placement: 'top'},
                {
                    default:()=> "Дата на которую запланировано оповещение",
                    trigger:()=>
                    h(NDatePicker,
                    {
                        type: 'date',
                        clearable: false,
                        placeholder: "дата оповещения",
                        formattedValue: tw.date ? dateToString(tw.date) : null,
                        valueFormat: 'dd.MM.yyyy',
                        format: 'dd.MM.yyyy',
                        onUpdateFormattedValue:(val) =>
                        {
                            tw.date = parseDate(val);
                            tw.weekDay = undefined;
                            console.warn("НЕОБХОДИМО СДЕЛАТЬ ПОИСК ЖУРНАЛА ПО БАЗЕ И ЗАМЕНИТЬ ВСЕ НАЙДЕННЫМ ЗНАЧЕНИЕМ!")
                        }
                    }),
                })
        }
        const time_selector = (tw: TimeWarning) =>
        {
            return h(NTooltip,{placement: 'top'},
                {
                    default:()=> "Время на которое запланировано оповещение",
                    trigger:()=>
                    h(NTimePicker,
                    {
                    
                        clearable: true,
                        placeholder: "время оповещения",
                        formattedValue: tw.time ? tw.time : null,
                        valueFormat: 'HH:mm',
                        format: 'HH:mm',
                        onUpdateFormattedValue:(val) =>
                        {
                            if(val)
                                tw.time = val;
                            //tw.weekDay = undefined;
                            //console.warn("НЕОБХОДИМО СДЕЛАТЬ ПОИСК ЖУРНАЛА ПО БАЗЕ И ЗАМЕНИТЬ ВСЕ НАЙДЕННЫМ ЗНАЧЕНИЕМ!")
                        }
                    }),
                })
        }
        const week_day_switch = (tw: TimeWarning, tooltip: string, day_number: number) =>
        {
            if(!tw.weekDay)
                tw.weekDay = [];
            const numbers = tw.weekDay;
            const have_day = numbers.findLast(f=>f == day_number);
            return h(NSwitch,
                {
                    value: numbers.includes(day_number) ? true : false,
                    "onUpdate:value":(v: boolean)=>
                    {
                        if(v)
                        {
                            numbers.push(day_number);
                            tw.date = undefined;
                            //console.log(day_number, numbers)
                        }
                        else
                        {
                            const ind = numbers.indexOf(day_number);
                            numbers.splice(ind, 1);
                        }
                    }
                },
                {
                    checked:() => tooltip,
                    unchecked:() => tooltip,
                })
        }
        const weekday_selector = (tw: TimeWarning) =>
        {
            if(!tw.weekDay)
                tw.weekDay = [];
            return h(NTooltip,{placement: 'top'},
                {
                    default:()=> "Дни недели на которые запланировано оповещение",
                    trigger:()=>
                    h('div',
                    {
                        style:
                        {
                            display: 'flex',
                            flexDirection: 'row',
                            gap: '5px'
                        } as CSSProperties
                    },
                    [
                        week_day_switch(tw, "пн.", 1),
                        week_day_switch(tw, "вт.", 2),
                        week_day_switch(tw, "ср.", 3),
                        week_day_switch(tw, "чт.", 4),
                        week_day_switch(tw, "пт.", 5),
                        week_day_switch(tw, "сб.", 6),
                        week_day_switch(tw, "вс.", 0),
                    ])
                })
        }

        const time_is_over = (tw: TimeWarning) =>
        {
            //const standart = '#03352d';
            const standart = '#000f';
            if(tw.date)
            {
                if(tw.date.getTime() < new Date().setHours(0, 0,0,0))
                    return '#550606';
                else return standart;
            }
            else
            return standart;
        }
        const dynamic_editor = () =>
        {
            return h(NScrollbar,
                {
                    style: {
                        maxHeight: '550px',
                        paddingRight: '15px',
                    } as CSSProperties
                },
                {
                    default:() =>
                    h(NDynamicInput,
                        {
                            value: values.value,
                            onRemove:(r) => values.value.splice(r, 1),
                            onCreate:(c) => {

                                values.value.splice(0, 0, new TimeWarning("", undefined, undefined, undefined))
                            }
                        },
                        {
                            default:({ value }: {value: TimeWarning}) =>
                            h('div',
                                {
                                    style: 
                                    {
                                        display: 'flex',
                                        flexDirection: 'row',
                                        alignItems: 'center',
                                        gap: '5px',
                                        borderRadius: '3px',
                                        background: time_is_over(value),
                                        width: '100%'
                                    } as CSSProperties
                                },
                                [
                                    h('div',
                                    {
                                        style: 
                                        {
                                            display: 'flex',
                                            flexDirection: 'column',
                                            alignItems: 'start',
                                            gap: '5px',
                                            width: '100%'
                                        } as CSSProperties
                                    },
                                    [
                                        h('div',
                                        {
                                            style:
                                            {
                                                display: 'flex',
                                                flexDirection: 'row',
                                                alignItems: 'center',
                                                gap: '5px'
                                            } as CSSProperties
                                        },
                                        [
                                            time_selector(value),
                                            weekday_selector(value),
                                            date_selector(value),
                                        ]),
                                        h(NInput, {
                                            type: 'textarea',
                                            value: value.text,
                                            placeholder: "описание задания",
                                            onUpdateValue:(t) => value.text = t
                                        }),
                                    ]),
                                ])
                        })
                })
        }

        
        const save_button = (val: TimeWarning[]) => 
        {
            return h(NButton,
            {
                type: 'success',
                onClick:()=> 
                {
                    emit('update:value', val.map(o => new TimeWarning(o.text, o.time, o.weekDay, o.date, o._notifyBeforeTargetTime)));
                }
            },
            {
                default:()=> "Сохранить"
            })
        }
        return {dynamic_editor, modal}
    },
    render ()
    {
        return this.modal()
    }
})