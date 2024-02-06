import 
{
    h,
    defineComponent,
    defineAsyncComponent,
    inject,
    onMounted,
    CSSProperties,
    ref,
    PropType,
    nextTick,
    getCurrentInstance,
    toRef
  } from 'vue'

import { NAvatar, NButton, NCard, NModal, NSpin, NTab, NTabPane, NTable, NTabs, NTooltip } from 'naive-ui';
import { Disease, DiseaseTest, DiseaseType, EditorType, Phones, User, Vacation, Vactination } from '../models/user.ts';
import { parseDate } from '../services/date.ts';
import UserStatus from './user_status.tsx';
import UserVactinationStatus from './user_vactination_status.tsx'
import { Heart, PersonSharp, Rocket } from '@vicons/ionicons5'
import { global_store } from '../store/index.ts';

import { match } from 'ts-pattern';
import { health_ico, palm_ico, user_ico } from '../services/svg.ts';
import { updateCounts } from '../services/data.ts';
const localProps = 
{
    users: 
    {
        type: Array as PropType<User[]>,
        default: []
    },
} as const

export const UsersConsumption = defineAsyncComponent({
    loader: () => import ('./users_consumption.tsx'),
    loadingComponent: h(NSpin)
})

export default defineComponent({
name: 'UsersConsumption',
props: localProps,
    setup (props, { slots }) 
    {
        const users = toRef(props, 'users')
        const table = () =>
        {
            return h(NTable,
                {
                    singleLine: false,
                    bordered: true,
                    size: 'small'
                },
                {
                    default:() => [
                    h('thead',
                        h('tr',
                        {
                            style:
                            {
                                backgroundColor: '#a61111 !important',
                                color: '#a61111 ',
                                fontSize: '24px',
                                fontWeight: '500'
                                
                            } as CSSProperties
                        },  

                            [
                                h('th', "" ),
                                h('th', "По списку" ),
                                h('th', "На лицо" ),
                                h('th', "Наряд" ),
                                h('th', "Командировка" ),
                                h('th', "Отпуск" ),
                                h('th', "Болезнь" ),
                                h('th', "Прочее" )
                            ]
                        )
                    ),
                    h('tbody',
                    [
                        h('tr',
                                [
                                    h('td',
                                    [
                                        h('div',
                                        {
                                            style:
                                            {
                                                display: 'flex',
                                                flexDirection: 'row',
                                                alignItems: 'center'
                                            } as CSSProperties
                                        },
                                        [
                                            h('span', "Руководство")
                                        ])
                                    ]),
                                    h('td', users.value.filter(f=>f.post.department == '')),
                                    h('td', m.phones[0].phoneNumber),
                                    h('td',
                                        h(UserStatus, 
                                        {
                                            user : m,
                                        })
                                    ),
                                    h('td',
                                        h(UserVactinationStatus, 
                                            {
                                                user : m,
                                            })
                                    )
                                ])
                    ]
                       
                    )]
                })
        }


        const show_editor = ref(false);
        let selected_user :User|null = null;
        let editorType: EditorType = 'user';
        const card_title = () =>
        {
            return match(editorType)
            .with('disease', () => "Заболевания")
            .with('user', () => "Личные данные")
            .with('vacation', () => "Отпуска")
            .otherwise(()=> "")
        }
        const modal_editor = (u: User) =>
        {
            return h(NModal,
            {
                show: show_editor.value,
                closable: true,
                closeOnEsc: true,
                preset: 'card',
                title: card_title(),
                "onUpdate:show":()=> show_editor.value = false,
                style:
                {
                    width: '600px'
                } as CSSProperties,
                
            },
            {
                default:()=>
                selected_user ?
                h(CardEditor,
                {
                    user: selected_user as User,
                    type: editorType,
                    'onUpdate:user':(u) =>
                    {
                        const ind = users.value.indexOf(selected_user as User);
                        users.value.splice(ind, 1, u);
                        //выполняем функции для проверки общего количества отпускников и болезней
                        //добавляем данные в глобалстор
                        updateCounts();
                        show_editor.value = false;
                        selected_user = null;
                    }  
                }) : []
            }
            )
        }

        return {complex}
    },

   
    
    render ()
    {
        return h(this.complex)
    }
})