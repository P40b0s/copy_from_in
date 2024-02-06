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
    toRef,
    onUnmounted
  } from 'vue'
import '../assets/styles/scrollable_table.scss'
import { NAvatar, NButton, NCard, NModal, NPagination, NScrollbar, NSpin, NTab, NTabPane, NTable, NTabs, NTbody, NThead, NTooltip, NTr } from 'naive-ui';
import { Disease, DiseaseTest, DiseaseType, EditorType, Phones, User,  Vactination } from '../models/user.ts';
import { parseDate } from '../services/date.ts';
import {UserStatus} from './user_status.tsx';
//import {UserVactinationStatus} from './user_vactination_status.tsx'
import { Heart, PersonSharp, Rocket } from '@vicons/ionicons5'
import { app_state_store, global_store } from '../store/index.ts';
import {LiveSearch} from './live_search.tsx'
import { match } from 'ts-pattern';
import { add_user_ico, health_ico, info_ico, palm_ico, user_ico } from '../services/svg.ts';
import { TypesBuilder, filterUsers, users } from '../services/data.ts';
import emitter from '../services/emit.ts';
import { departments, posts } from '../services/dictionaries.ts';
import { InvokeArgs } from '@tauri-apps/api/tauri';
import { TauriCommands } from '../services/tauri.ts';

type Pagination = 
{
    row: number,
    offset: number
}
export const UsersListAsync = defineAsyncComponent({
    loader: () => import ('./user_list.tsx'),
    loadingComponent: h(NSpin)
})
const search_value = ref("");
export const UsersList = defineComponent({
    setup (props, { slots }) 
    {
        //TODOзагрузка данных из файла и выборка нужных данных в global_store
        const current_page = ref(1);
        const items_on_page = 20;
        let current_offset = 0;
        
        const get_users = async (offset: number) =>
        {
            console.log("Запрос юзеров оффсет!", offset);
            current_offset = 0;
            const u = await TauriCommands.Users.get_users(items_on_page, offset);
            if (u)
                users.value = u;
        }
        onMounted(async ()=> 
        {
            await get_users(0);
            emitter.on('userUpdated', async ()=> 
            {
                await get_users(current_offset);
                console.log("юзеры апдейтнулись" + users.value + current_offset);
            });
            
        })
        onUnmounted(()=> emitter.off('userUpdated'))
        const complex = () =>
        {
            return h('div',
            [
                h(list),
                h(NPagination,
                {
                    itemCount: app_state_store.getState().appState.users_count,
                    pageSizes: [items_on_page],
                    showSizePicker: false,
                    simple: true,
                    page: current_page.value,
                    onUpdatePage: async (page) => 
                    {
                        current_page.value = page;
                        current_offset = page * items_on_page;
                        await get_users(current_offset);
                    },
                },
                {
                    
                })

            ])
        }

        const scrollable_body = () => 
        {
            return {
                position: 'sticky',
                top: '0'
            } as CSSProperties
        }

        const label_1 = "ФИО";
        const label_2 = "Должность";
        const label_3 = "Телефон";
        const label_4 = "Статус";
        const label_5 = "Вакцинации и тесты"
        const list = () =>
        {
            return h('div',
                {
                    width: '100%',
                    style:
                    {
                        fontSize: '17px'
                    } as CSSProperties
                    
                },
                [
                    h('ui',
                    {
                        class: 'responsive-table'
                    },
                    [
                        h('li',
                        {
                            style:
                            {
                                alignSelf: 'center',
                                fontSize:"17px",
                                fontWeight: "700"
                            } as CSSProperties,
                            class: 'table-header'
                        },  
                            [
                                h('div',{class:'col col-1'},
                                
                                    h('div',
                                    {
                                        style:
                                        {
                                            display: 'flex',
                                            flexDirection: 'row',
                                            alignItems: 'center'
                                        } as CSSProperties,
                                    },
                                    [
                                        menu_button(TypesBuilder.build_user(), 'new', "Добавить нового человека", add_user_ico),
                                        h('span', {style:{marginLeft: '10px'}}, "ФИО"),
                                        h(LiveSearch,
                                            {
                                                value: search_value.value,
                                                "onUpdate:value":(s: string) => filterUsers(s),
                                                style:{
                                                    width: '200px',
                                                    fontSize:"12px",
                                                    fontWeight: "100",
                                                    marginLeft: '20px'
                                                } as CSSProperties
                                            }),
                                        
                                    ])
                                
                                 ),
                                h('div',{class:'col col-2'}, "Должность" ),
                                h('div',{class:'col col-3'}, "Телефон" ),
                                h('div',{class:'col col-4'}, "Статус" ),
                                //h('div',{class:'col col-5'}, "Вакцинации и тесты" )
                            ]
                        ),
                       
                        h(NScrollbar,
                            {
                               style:{
                                maxHeight: '600px'
                               } as CSSProperties
                            },
                            {
                                default:() => 
                                users.value.map(m=>
                                    {
                                        return h("li",
                                        {
                                            class: "table-row"
                                        },
                                        [
                                            h('div',
                                            {
                                                class: 'col col-1',
                                                style: {
                                                    display: 'flex',
                                                    flexDirection: 'row',
                                                    alignItems: 'center',
                                                    justifyContent: 'start',
                                                    gap: '10px'
                                                } as CSSProperties
            
                                            }, 
                                            [
                                                menu_button(m, 'edit', "Данные сотрудника", user_ico),
                                                h('span', {
                                                    style: {
                                                       //flexGrow:1
                                                    } as CSSProperties
                                                }, m.surname + " " + m.name1 + " " + m.name2)
                                            ]
                                            ),
                                            h('div',
                                            {
                                                class: 'col col-2',
                                                style: {
                                                    display: 'flex',
                                                    flexDirection: 'column',
                                                    justifyContent: 'center'
                                                } as CSSProperties
                                            },
                                            [
                                                h('div', m.post.name),
                                                h('div', m.department.name),
                                            ]),
                                            h('div',
                                            {
                                                class: 'col col-3',
                                                style: {
                                                    display: 'flex',
                                                    flexDirection: 'column',
                                                    justifyContent: 'center'
                                                } as CSSProperties
                                            }, 
                                            m.phones[0]?.phoneNumber),
                                            h('div',{class: 'col col-4'},   
                                                h(UserStatus, 
                                                {
                                                    user : m,
                                                })
                                            ),
                                            // h('div',{class: 'col col-5'},   
                                            //     h(UserVactinationStatus, 
                                            //     {
                                            //         user : m,
                                            //     })
                                            // )
                                        ])
                                    })
                                }),
                    ]),
                ])
        }

        const buttons = (user: User) =>
        {
            return h('div',
            {
                style:
                {
                    display: 'flex',
                    flexDirection: 'row',
                    alignItems: 'center',
                } as CSSProperties
            },
            [
                h('div',
                {
                    style:
                    {
                        display: 'flex',
                        flexDirection: 'row',
                        justifyContent: 'space-between',
                        marginRight: '10px',
                        height: '80px',
                    } as CSSProperties
                },
                [
                    menu_button(user, 'edit', "Данные сотрудника", user_ico),
                ]),
            ])
        }

        const menu_button = (user: User, type: EditorType, tooltip: string, icon: string) =>
        {
            return h(NTooltip,{placement: 'top'},
            {
                trigger:() =>
                h(NButton,{
                    round: true,
                    text: true,
                    size: 'small',
                    onClick:() =>
                    {
                        emitter.emit("openUserCardEditor", {current_user: user, type: type})
                    },
                    //class: 'hover-button',
                    style:
                    {
                        backgroundColor: 'transparent'
                    }
                },
                {
                        default:() => h(NAvatar,
                    {
                        size: 40,
                        src: icon,
                        round: true,
                        class: 'hover-button',
                        style:
                        {
                            '--hover-button-boxshadow': '#6cdb39c7',
                            backgroundColor: 'transparent',
                        }   as CSSProperties
                        
                    }),
                }),
                default:() => tooltip
            })
        }  
        return {complex}
    },

   
    render ()
    {
        return h(this.complex)
    }
})