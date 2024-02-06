import { AddCircleOutline, Close } from "@vicons/ionicons5";
import { NButton, NCard, NDatePicker, NDynamicInput, NFormItem, NInput, NScrollbar, NSelect, NSpin, NTooltip, SelectGroupOption, SelectOption } from "naive-ui";
import { CSSProperties, PropType, VNode, defineAsyncComponent, defineComponent, defineModel, defineProps, defineEmits, defineSlots, h, ref, toRaw, Ref } from "vue";
import { ComponentType, Disease, DiseaseType, Id, User, get_dict_value } from "../../models/user";
import { TypesBuilder, updateDiseases, updateUser } from "../../services/data.ts";
import { boolean } from "ts-pattern/dist/patterns";
import { clinics, disease_types } from "../../services/dictionaries.ts";
import { DateFormat, DateTime } from "../../services/date.ts";
import { TauriCommands } from "../../services/tauri.ts";
import emitter from "../../services/emit.ts";
import { app_state_store, global_store } from "../../store/index.ts";


const clinics_names = (): Array<SelectOption | SelectGroupOption> =>
{
    return clinics.value.map(c=>
    {
        return {
            label: c.name,
            value: c.id,
            disabled: false
        }
    })
}


const localProps = 
{
    /**Человек */
    user: 
    {
        type: Object as PropType<User>,
        required: true
    },
    onCreate: Function as PropType<(index: number) => any>,
    save: Function as PropType<() => boolean>,
    /**стили полей редактора */
    styles: 
    {
        type: Object as PropType<CSSProperties>,
        required: true
    },
} as const

export const DiseaseEditorAsync = defineAsyncComponent({
    loader: () => import ('./disease_editor.tsx'),
    loadingComponent: h(NSpin)
})

export const DiseaseEditor = defineComponent({
props: localProps,
setup(props, {slots}) 
{
    const diseases_types = (): Array<SelectOption | SelectGroupOption> =>
    {
        return disease_types.value.map(d=>
        {
            return {
                label: d.name,
                value: d.id,
                disabled: false
            }
        })
    }
    async function save_form(): Promise<boolean>
    {
        const app_state = await TauriCommands.Statuses.update_diseases(diseases.value, props.user.id);
        if (app_state)
        {
            app_state_store.set_app_state = app_state;
            emitter.emit('userUpdated');
            return true;
        }
        else
        {
            console.error("Ошибка при сохранении списка заболеваний для юзера  " + diseases.value[0].userId);
            return false;
        }
    }
    async function validate(): Promise<boolean>
    {
        return true;
    }
  
    const diseases = ref(structuredClone(toRaw(props.user.diseases)));
    const dynamic_editor = () =>
    {
        return h(NDynamicInput,
            {
                value: diseases.value,
                onRemove:(r) => diseases.value.splice(r, 1),
                onCreate:(c) =>
                {
                    diseases.value.splice(0, 0, TypesBuilder.build_disease(props.user.id))
                }
            },
            {
                default:({ value }: {value: Disease}) =>
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
                        h(NTooltip,{},
                        {
                            default:()=> "Вид заболевания",
                            trigger:()=>
                            h(NSelect,
                            {
                                value: value.diseaseType.name,
                                options: diseases_types(),
                                onUpdateValue:(v: string)=>
                                {
                                    const r = disease_types.value.find(f=>f.id == v);
                                    value.diseaseType = r as DiseaseType;
                                } 
                            }),
                        }),
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
                            h(NTooltip,{placement: 'left'},
                            {
                                default:()=> "Дата открытия больничного листа",
                                trigger:()=>
                                h(NDatePicker,
                                {
                                    type: 'date',
                                    clearable: false,
                                    placeholder: "дата открытия больничного листа",
                                    formattedValue: new DateTime(value.dateOfIllness).to_string(DateFormat.CalendarFormat),
                                    valueFormat: DateFormat.CalendarFormat.toString(),
                                    format: DateFormat.CalendarFormat.toString(),
                                    onUpdateFormattedValue:(val) => value.dateOfIllness = new DateTime(val).to_string(DateFormat.SerializedDateTime)
                                }),
                            }),
                            h(NTooltip,{ placement: 'top'},
                                {
                                    default:()=> "Дата закрытия больничного листа, пока больничный не закрыт сюда дату ставить не нужно",
                                    trigger:()=>
                                    h(NDatePicker,
                                    {
                                        type: 'date',
                                        placeholder: "дата закрытия больничного листа",
                                        clearable: true,
                                        formattedValue: value.dateOfRecovery ? new DateTime(value.dateOfRecovery).to_string(DateFormat.CalendarFormat) : undefined,
                                        valueFormat: DateFormat.CalendarFormat.toString(),
                                        format: DateFormat.CalendarFormat.toString(),
                                        onUpdateFormattedValue:(val) => 
                                        {
                                            if(val == null)
                                                value.dateOfRecovery = undefined;
                                            else
                                                value.dateOfRecovery = new DateTime(val).to_string(DateFormat.SerializedDateTime);
                                        }
                                        
                                    })
                                }),
                            
                        ]),
                        h(NTooltip,{placement: 'right-end'},
                        {
                            default:()=> "Поликлинника в которую обращался сотрудник и в которой был открыт больничный лист",
                            trigger:()=>
                            h(NSelect,
                            {
                                value: value.clinic.name,
                                options: clinics_names(),
                                onUpdateValue:(c: string)=>
                                {
                                    value.clinic = get_dict_value(clinics.value, c);
                                } 
                            }),
                        }),
                        h(NInput,
                        {
                            value: value.note,
                            placeholder: "Дополнительная информация",
                            onUpdateValue:(v)=> value.note = v
                        })
                       
                    ])
            })
    }
        return { dynamic_editor, save_form, validate};
    },
    render ()
    {
        return this.dynamic_editor()
    }
});