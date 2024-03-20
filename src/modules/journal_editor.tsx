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
    watch
  } from 'vue'

import { NAvatar, NButton, NCard, NCheckbox, NConfigProvider, NDatePicker, NDynamicInput, NForm, NFormItem, NInput, NInputGroup, NInputNumber, NModal, NScrollbar, NSelect, NSpin, NTab, NTabPane, NTable, NTabs, NTooltip, SelectGroupOption, SelectOption } from 'naive-ui';
import { Disease, DiseaseTest, DiseaseType, EditorType, Journal, JournalItem, Dictionary, Phones, User, Vacation, Vactination, enumKeys } from '../models/user.ts';
import { DateFormat, DateTime, dateToString, parseDate, parseDateObj, timeToString } from '../services/date.ts';
import { AddCircleOutline, Close, Home, RemoveOutline } from '@vicons/ionicons5';
import { match } from 'ts-pattern';
import { ruRU, dateRuRU } from 'naive-ui'
import { test_journal } from '../services/test_data.ts';

const localProps = 
{
    is_open: 
    {
        type: Boolean,
        required: true,
        default: false
    },
} as const

export const JournalEditorAsync = defineAsyncComponent({
    loader: () => import ('./journal_editor.tsx'),
    loadingComponent: h(NSpin)
})

export const JournalEditor =  defineComponent({
props: localProps,
emits:
{
    'update:value': (value: Journal) => value,
    'onClose': (v: boolean) => v
},
    setup (props, {emit}) 
    {
        const load_journal = async (date?: String) : Promise<Journal> =>
        {
            if(!date)
                date = new DateTime().to_string(DateFormat.CalendarFormat)
            console.warn("Журнал еще не сделан!")
            return test_journal;
        }
        const values = ref<Journal>();
        onMounted(async ()=> values.value = await load_journal());

        const modal = () =>
        {
            return h(NModal,
            {
                show: props.is_open,
                preset: 'dialog',
                closable: true,
                blockScroll: true,
                showIcon: false,
                style:
                {
                    minWidth: '1000px',
                    width: '1000px'
                } as CSSProperties,
                onUpdateShow: async (u)=>
                {
                    if(u)
                    {
                        values.value = await load_journal();
                    }
                },
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
                "Журнал от ",
                h(NTooltip,{placement: 'top'},
                {
                    default:()=> "Дата журнала",
                    trigger:()=>
                    h(NDatePicker,
                    {
                        type: 'date',
                        clearable: false,
                        placeholder: "дата журнала",
                        formattedValue: new DateTime(values.value?.date).to_string(DateFormat.CalendarFormat),
                        valueFormat: DateFormat.CalendarFormat.toString(),
                        format: DateFormat.CalendarFormat.toString(),
                        onUpdateFormattedValue: async (val) =>
                        {
                            if(values.value)
                                values.value.date = new DateTime(val).to_string(DateFormat.SerializedDateTime)
                            values.value = await load_journal(val);
                        }
                    }),
                }),
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
                    save_button()
                ]
                )
            })
        }
        const dynamic_editor = () =>
        {
            return h(NScrollbar,
                {
                    style: {
                        maxHeight: '550px'
                    } as CSSProperties
                },
                {
                    default:() =>
                    h(NDynamicInput,
                        {
                            value: values.value?.items,
                            onRemove:(r) => values.value?.items.splice(r, 1),
                            onCreate:(c) => 
                            {
                                const index = values.value?.items.length ?? -1;
                                values.value?.items.splice(index, 0, {time: new DateTime().to_string(DateFormat.Time), note: ""});
                            }
                        },
                        {
                            default:({ value }: {value: JournalItem}) =>
                            h('div',
                                {
                                    style: 
                                    {
                                        display: 'flex',
                                        alignItems: 'center',
                                        width: '100%'
                                    } as CSSProperties
                                },
                                [
                                    h('div', 
                                    {
                                        style: 
                                        {
                                            alignSelf: 'self-start',
                                            class: "neon-blue",
                                            fontSize: '18px',
                                            width: '60px'
                                        } as CSSProperties
                                    },
                                    value.time,
                                    ),
                                    h(NInput, {
                                        type: 'textarea',
                                        value: value.note,
                                        placeholder: "Введите заметку",
                                        onUpdateValue:(t) => value.note = t
                                    }),
                                ])
                        })
                })
        }

        
        const save_button = () => 
        {
            return h(NButton,
            {
                type: 'success',
                disabled: values.value == undefined,
                onClick: async ()=> 
                {
                    if (values.value)
                        //await save_cmd<Journal>(TauriSaveCmd.Journal, values.value); 
                    console.error("Сохранение журнала не реализовано!")
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