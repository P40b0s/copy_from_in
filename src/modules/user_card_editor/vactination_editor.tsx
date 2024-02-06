import { AddCircleOutline, Close } from "@vicons/ionicons5";
import { NButton, NCard, NDatePicker, NDynamicInput, NFormItem, NInput, NScrollbar, NSelect, NSpin, NTooltip, SelectGroupOption, SelectOption } from "naive-ui";
import { CSSProperties, PropType, VNode, defineAsyncComponent, defineComponent, defineModel, defineProps, defineEmits, defineSlots, h, ref, toRaw, Ref } from "vue";
import { ComponentType, Disease, Id, User, Vactination } from "../../models/user.ts";
import { TypesBuilder, updateDiseases, updateUser} from "../../services/data.ts";
import { boolean } from "ts-pattern/dist/patterns";
import { disease_types } from "../../services/dictionaries.ts";
import { DateFormat, DateTime } from "../../services/date.ts";


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

export const VactinationEditorAsync = defineAsyncComponent({
    loader: () => import ('./vactination_editor.tsx'),
    loadingComponent: h(NSpin)
})

export const VactinationEditor = defineComponent({
props: localProps,
setup(props, {slots}) 
{
    async function save_form(): Promise<boolean>
    {
        updateVactinations(vac_list.value);
        return true;
    }
    async function validate(): Promise<boolean>
    {
        return true;
    }
  
    const vac_list = ref(structuredClone(toRaw(vactinations.value)));
    const dynamic_editor = () =>
    {
        return h(NDynamicInput,
            {
                value: vac_list.value.filter(f=>f.id == props.user.id),
                onRemove:(r) => vac_list.value.splice(r, 1),
                onCreate:(c) =>
                {
                    vac_list.value.splice(0, 0, TypesBuilder.build_vactination(props.user.id))
                }
                 
            },
            {
                default:({ value }: {value: Vactination}) =>
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
                        h(NTooltip,{placement: 'top'},
                        {
                            default:()=> "От чего вакцинация",
                            trigger:()=>
                            h(NSelect,
                            {
                                value: value.type,
                                options: diseases_types(),
                                onUpdateValue:(v: string)=>
                                {
                                    value.type = v;
                                } 
                            }),
                        }),
                        h('div',
                        {
                                style: 
                                {
                                    width: '100%'
                                } as CSSProperties
                        },
                        h(NTooltip,{placement: 'top'},
                        {
                            default:()=> "Дата вакцинации",
                            trigger:()=>
                            h(NDatePicker,
                            {
                                type: 'date',
                                clearable: false,
                                placeholder: "дата вакцинации",
                                formattedValue: new DateTime(value.date).to_string(DateFormat.CalendarFormat),
                                valueFormat: DateFormat.CalendarFormat.toString(),
                                format: DateFormat.CalendarFormat.toString(),
                                onUpdateFormattedValue:(val) => value.date = new DateTime(val).to_string(DateFormat.SerializedDateTime)
                            }),
                        }),  
                        ),
                    ]),
                    h(NInput,
                    {
                        value: value.note,
                        placeholder: "дополнительная информация",
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