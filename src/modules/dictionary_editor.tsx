import 
{
    h,
    defineComponent,
    defineAsyncComponent,
    CSSProperties,
    PropType,
    ref,
    toRaw
  } from 'vue'

import { NButton, NCheckbox, NDynamicInput, NInput, NModal, NScrollbar, NSpin, NTooltip } from 'naive-ui';
import { DiseaseType, Dictionary} from '../models/user.ts';
import { v4 } from 'uuid';

const localProps = 
{
    values: 
    {
        type: Array as PropType<Dictionary[]>,
        required: true
    },
    is_open: 
    {
        type: Boolean,
        required: true,
        default: false
    },
} as const

export const DictionaryEditorAsync = defineAsyncComponent({
    loader: () => import ('./dictionary_editor.tsx'),
    loadingComponent: h(NSpin)
})

export const DictionaryEditor = defineComponent({
props: localProps,
emits:
{
    'update:value': (value: Dictionary[]) => value,
    'onClose': (v: boolean) => v
},
    setup (props, {emit}) 
    {
        const values = ref(structuredClone(toRaw(props.values)));
        const modal = () =>
        {
            return h(NModal,
            {
                show: props.is_open,
                preset: 'dialog',
                closable: true,
                blockScroll: true,
                showIcon: false,
                title: 'Редактор словаря',
                style:
                {
                    minWidth: '500px',
                    width: '500px'
                } as CSSProperties,
            },
            {
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
        const dynamic_editor = () =>
        {
            return h(NScrollbar,
                {
                    style: {
                        maxHeight: '250px',
                        paddingRight: '15px'
                    } as CSSProperties
                },
                {
                    default:() =>
                    h(NDynamicInput,
                        {
                            value: values.value,
                            onRemove:(r) => values.value.splice(r, 1),
                            onCreate:(c) => values.value.splice(c, 0, {id: v4(), name: ""})
                        },
                        {
                            default:({ value }: {value: Dictionary}) =>
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
                                    h(NInput, {
                                        type: 'text',
                                        value: value.name,
                                        placeholder: "Введите значение",
                                        onUpdateValue:(t) => value.name = t
                                    }),
                                    isDisease(value) ? h(NTooltip,{
                                    },
                                    {
                                        trigger:() =>  
                                        h(NCheckbox, 
                                        {
                                            checked: value.needReference,
                                            onUpdateChecked:(v: boolean) =>
                                            {
                                                value.needReference = v;
                                            },
                                        },
                                        {
                                            default:() => ""
                                        }),
                                        default:() => "По ходу заболевания необходимо формировать эпидемиологическую справку"
                                    }) : []
                                    
                                ])
                        })
                })
        }

        const isDisease = (val: Dictionary | DiseaseType): val is DiseaseType =>
        {
            return (val as DiseaseType).needReference !== undefined;
        }
        
        const save_button = (val: Dictionary[]) => 
        {
            return h(NButton,
            {
                type: 'success',
                onClick:()=> 
                {
                    emit('update:value', val);
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