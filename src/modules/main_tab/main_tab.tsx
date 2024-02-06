import 
{
    h,
    defineComponent,
    defineAsyncComponent,
    inject,
    onMounted,
    CSSProperties
  } from 'vue'

import { NAvatar, NBadge, NButton, NCard, NSpin, NTab, NTabPane, NTabs, NTooltip } from 'naive-ui';
import {UsersList} from '../user_list.tsx'
import { global_store } from '../../store/index.ts';
import { Disease, DiseaseTest, DiseaseType, Phones, User, Vacation, Vactination } from '../../models/user.ts';
import { disease_ico, palm_ico } from '../../services/svg.ts';
import { users } from '../../services/data.ts';
import {NavMenu} from '../nav.tsx'
const localProps = 
{
    size: 
    {
        type: Number,
        default: 100
    },
} as const

export const MainTabAsync = defineAsyncComponent({
    loader: () => import ('./main_tab.tsx'),
    loadingComponent: h(NSpin)
})

export const MainTab =  defineComponent({
props: localProps,
    setup (props) 
    {
    const crd = () => 
    {
        return h(NCard,
            {
                style:
                {
                    marginBottom: '0px'
                } as CSSProperties
            },
            {
                header:() =>
                h('div',
                {
                    style:
                    {
                        display: 'flex',
                        flexDirection: 'row'
                    } as CSSProperties
                },
                [
                  
                    // h(NBadge,
                    //     {
                    //         type: 'info'
                    //     },
                    //     {
                    //         value:() => global_store.getState().usersInVacationCount,
                    //         default:() => 
                    //         h(NTooltip,{placement: 'left'},
                    //         {
                    //             trigger:() =>
                    //             h(NAvatar,
                    //             {
                    //                 size: 50,
                    //                 src: palm_ico,
                    //                 style:
                    //                 {
                    //                     backgroundColor: 'transparent',
                    //                     marginRight: '5px'
                    //                 }   as CSSProperties
                                    
                    //             }),
                    //             default:() => "Отпуск"
                    //         }),
                    //     }),
                    //     h(NBadge,
                    //     {
                    //     },
                    //     {
                    //         value:() => global_store.getState().usersInDiseaseCount,
                    //         default:() => 
                    //         h(NTooltip,{placement: 'left'},
                    //         {
                    //             trigger:() =>
                    //             h(NAvatar,
                    //             {
                    //                 size: 50,
                    //                 src: disease_ico,
                    //                 style:
                    //                 {
                    //                     backgroundColor: 'transparent',
                    //                     marginRight: '5px'
                    //                 }   as CSSProperties
                                    
                    //             }),
                    //             default:() => "Болезнь"
                    //         }),
                    //     })
                ]),
                default:() =>  tab_view()
            }
        )
    }

    const tab_view = () =>
    {
        return h(NTabs,
            {
                justifyContent: 'space-evenly',
                type: 'line',
                defaultValue: "ras"
            },
            {
                default:() => [users_list(), rashod2()]
            }
        )
    }

    const users_list = () => 
    {
        return h(NTabPane,
            {
                tab: 'Список сотрудников',
                name: 'ras'
            },
            {
                default:() => h(UsersList)
            }
        )
    }


    const rashod2 = () => 
    {
        return h(NTabPane,
            {
                tab: 'Расход',
                name: 'ras2'
            },
            {
                default:() => h(UsersList)
            }
        )
    }
        return {crd}
    },
    
    render ()
    {
        return this.crd();
    }
})