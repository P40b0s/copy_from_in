import 
{
    h,
    defineComponent,
    defineAsyncComponent,
    inject,
    onMounted,
    CSSProperties,
    PropType,
    RendererNode,
    VNode,
    RendererElement,
    ref,
    toRef
  } from 'vue'

import { NAvatar, NButton, NCard, NIcon, NSpin, NTab, NTabPane, NTable, NTabs, NTooltip } from 'naive-ui';
import { Disease, DiseaseTest, DiseaseType, Phones, User, Vacation, Vactination } from '../models/user.ts';
import { parseDate } from '../services/date.ts';
import { test_ico, vacc_ico } from '../services/svg.ts';
import { StatusCard } from './status_card.tsx';
import { getDiseaseType } from '../services/dictionaries.ts';


const localProps = 
{
    user: 
    {
        type: Object as PropType<User>,
        required: true
    },
} as const

// export const UserStatus = defineAsyncComponent({
//     loader: () => import ('./user_status.tsx'),
//     loadingComponent: h(NSpin)
// })

export const UserVactinationStatus = defineComponent({
props: localProps,
    setup (props) 
    {
        //если данные будут только отображаться то надо сделать toRef
        const user = toRef(props, 'user');
        const renderStatus = () =>
        {
            let statuses: VNode<RendererNode, RendererElement, { [key: string] : any; }>[] = [];
            let now = new Date();
            let modifer = new Date();
            modifer.setDate(now.getDate() - 7);
            const tests = user.value.tests.filter(s=> parseDate(s.date) > modifer);
            if(tests.length > 0)
            {
                statuses.push(statusTests(tests));
            }
            const year = new Date().getFullYear() -1;
            const user_vactionations = user.value.value.filter(f=>f.id == user.value.id && parseDate(f.date).getTime() > new Date().setFullYear(year));
            if(user_vactionations.length > 0)
            {
                statuses.push(statusVactination(user_vactionations))
            }
            return statuses;
        }

        const statusTests = (tests: DiseaseTest[]) =>
        {
            return h('div',
            {
                style:
                {
                    display: 'flex',
                    flexDirection: 'row',
                    alignItems: 'center',
                    marginTop: '5px',
                    padding: '0px 0',
                    boxShadow: '0 1em 1em -1em rgba(0, 0, 0, .25)',
                }   as CSSProperties
            },
            [
                h(NTooltip,{placement: 'left'},
                {
                    trigger:() =>
                    h(NAvatar,
                    {
                        size: 50,
                        src: test_ico,
                        style:
                        {
                            backgroundColor: 'transparent',
                            marginRight: '5px'
                        }   as CSSProperties
                        
                    }),
                    default:() => "Тесты"
                }),
                h('div',
                tests.map(t=>
                {
                    if (t.isActive)
                    {
                        return h('div', 
                        {
                            style:
                            {
                                color: 'red',
                            }   as CSSProperties
                        }, t.date + " => положительный")
                    }
                    else
                    {
                        return h('div', 
                        {
                            style:
                            {
                                color: 'green',
                            }   as CSSProperties
                        },  t.date + " => отрицательный")
                    }
                }))
            ])
        }

        const statusVactination = (vac: Vactination[]) =>
        {

            //const dis_type = getDiseaseType(vac.type);
            //const progress = getDaysDiff(parseDate(dis.dateOfIllness), new Date());
            return h(StatusCard,
            {
                //FIXME Компонент не обновляется потому что привязанные свойства не меняются!
                //если мы будем собирать key из обновляющихся свойств то компонент будет ререндериться!
               
                key: vac.length,
                avatar: vacc_ico,
                shadowbox_color: 'rgba(88, 226, 24, 0.5)',
                tooltip: "Вакцинации"
            },
            {
                default:() =>h('div', 
                {
                    style: {
                        width: '100%',
                        display:'flex',
                        flexDirection:'column',
                    } as CSSProperties
                },
                vac.map(v=>
                {
                    return h('div', getDiseaseType(v.type) ?  getDiseaseType(v.type)?.name + " => " + v.date : [])
                }))
            })
        }
        return {renderStatus}
    },

   
    
    render ()
    {
        return this.renderStatus()
    }
})