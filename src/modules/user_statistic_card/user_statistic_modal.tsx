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
    toRaw
  } from 'vue'

import { NAvatar, NButton, NCard, NCheckbox, NConfigProvider, NDatePicker, NDynamicInput, NForm, NFormItem, NInput, NInputGroup, NInputNumber, NModal, NScrollbar, NSelect, NSpin, NTab, NTabPane, NTable, NTabs, NTooltip, SelectGroupOption, SelectOption } from 'naive-ui';
import { Disease, DiseaseTest, DiseaseType, EditorType, Dictionary, Phones, User, Vacation, Vactination, enumKeys } from '../../models/user.ts';
import { AddCircleOutline, Close, Home, RemoveOutline } from '@vicons/ionicons5';
import './style.scss'

const localProps = 
{
    user: 
    {
        type: Object as PropType<User>,
        required: true
    },
    is_open: 
    {
        type: Boolean,
        required: true,
        default: false
    },
} as const

export const UserStatisticAsync = defineAsyncComponent({
    loader: () => import ('./user_statistic_modal.tsx'),
    loadingComponent: h(NSpin)
})

export const UserStatistic = defineComponent({
props: localProps,
emits:
{
    'onClose': (v: boolean) => v
},
    setup (props, {emit}) 
    {
        const user = toRef(props.user);

        const modal = () =>
        {
           return h('div',
           {
                id: 'openModal',
                class: 'modalWindow'
           },
                h('div',
                [
                    h('div', {class: 'modalHeader'},
                    [
                        "Заголовок модального окна",
                        h('a', 
                        {
                            href: '#close',
                            title: 'Закрыть',
                            class: 'close'
                        })
                    ]
                    ),
                    h('div', {class: 'modalContent'},
                    "Тело модального окна"
                    ),
                    h('div', {class: 'modalFooter'},
                    "Футер модального окна"
                    ),
                ])
            )
        }
        
        // const save_button = (val: Dictionary[]) => 
        // {
        //     return h(NButton,
        //     {
        //         type: 'success',
        //         onClick:()=> 
        //         {
        //             emit('update:value', val);
        //         }
        //     },
        //     {
        //         default:()=> "Сохранить"
        //     })
        // }
        return {modal}
    },
    render ()
    {
        return this.modal()
    }
})