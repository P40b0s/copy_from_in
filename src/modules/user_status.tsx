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

import { NAvatar, NButton, NCard, NIcon, NProgress, NSpin, NTab, NTabPane, NTable, NTabs, NTooltip } from 'naive-ui';
import { Disease, DiseaseTest, DiseaseType, Phones, Status, User,  Vactination } from '../models/user.ts';
import { DateTime, getDaysDiff, parseDate } from '../services/date.ts';
import { app_state_store, global_store } from '../store/index.ts';
import { disease_ico, disease_red_ico, palm_ico, run_man_ico, train_ico } from '../services/svg.ts';
import {StatusCard } from './status_card.tsx';
import { getDiseaseType } from '../services/dictionaries.ts';
import { match } from 'ts-pattern';

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
// const getDaysDiff = (start_date: Date, end_date: Date) : {progress: number, days_left: number, days_over: number} =>
// {
//     const date_now = new Date().setHours(0, 0, 0);
//     const oneDay = 24 * 60 * 60 * 1000; // hours*minutes*seconds*milliseconds
//     const diffFullVacation = Math.round(Math.abs((end_date.getTime() - start_date.getTime()) / oneDay));
//     const diffFromNow = Math.round(Math.abs((end_date.getTime() - date_now) / oneDay)) +1;
//     return {progress: 100 - Math.round((diffFromNow / diffFullVacation) * 100), days_left: diffFromNow, days_over: diffFullVacation};
// }

export const UserStatus = defineComponent({
name: 'UserStatus',
props: localProps,
    setup (props) 
    {
        //если данные будут только отображаться то надо сделать toRef
        const user = toRef(props, 'user');
        const renderStatus = () =>
        {
            let statuses: VNode<RendererNode, RendererElement, { [key: string] : any; }>[] = [];
            let now = new DateTime(app_state_store.getState().appState.current_date)
            now.set_hours(0, 0);
            const dis = user.value.diseases.filter(s=> s.dateOfRecovery == undefined);
            if(dis.length > 0)
            {
                statuses.push(statusDisease(dis[0]));
            }
            //const vac = user.value.vacations.filter(s=> (now >= new DateTime(s.startDate).as_date() && (now <= new DateTime(s.endDate).as_date())));
            const stat = user.value.statuses.filter(s=> now.greater_or_equal_then(new DateTime(s.startDate)) && now.lower_or_equal_then(new DateTime(s.endDate)));
            if(stat.length > 0)
            {
                stat.forEach(s=>
                {
                    match(s.statusType)
                    .with(0, () =>  statuses.push(statusVacation(s)))
                    .with(1, () =>  statuses.push(statusOrdered(s, "Командировка ")))
                    .with(2, () =>  statuses.push(statusBuisnessTrip(s, "В распоряжении ")))
                })
            }
            return statuses;
        }

        const statusDisease = (dis: Disease) =>
        {
            const dis_type = dis.diseaseType;
            const progress = getDaysDiff(parseDate(dis.dateOfIllness), new Date());
            if (dis_type)
            return h(StatusCard,
            {
                //FIXME Компонент не обновляется потому что привязанные свойства не меняются!
                //если мы будем собирать key из обновляющихся свойств то компонент будет ререндериться!
               
                key: dis.diseaseType.id + dis.dateOfIllness,
                avatar: dis_type.needReference ? disease_red_ico : disease_ico,
                shadowbox_color: 'rgba(188, 16, 16, 0.4)',
                tooltip: "Заболевание, болен " + (progress.overall) + " дн."
            },
            {
                default:() =>h('div', 
                {
                    // style: {
                    //     width: '100%',
                    //     color: dis_type.needReference ? 'rgba(188, 16, 16, 0.8)' : 'rgb(0, 0, 0)' 
                    // } as CSSProperties
                },
                [
                    h('div', dis_type.name),
                    h('div', dis.dateOfIllness),
                ])
            })
            else return h('div',[]);
        }

        const statusVacation = (vac: Status) =>
        {
            const progress = getDaysDiff(parseDate(vac.startDate), parseDate(vac.endDate));
            return h(StatusCard,
                {
                    key: vac.endDate + vac.startDate + vac.place + progress.progress,
                    avatar: palm_ico,
                    shadowbox_color: 'rgba(37, 165, 233, 0.6)',
                    tooltip: "Отпуск " + (progress.overall) +" дней, осталось " + progress.left + " дн."
                },
                {
                    default:() =>
                    h('div', 
                    {
                        style: {
                            width: '100%',
                        } as CSSProperties
                    },
                    [
                        h('div', vac.place),
                        h('div', vac.startDate + " - " + vac.endDate),
                        h(NProgress,
                        {
                            type: 'line',
                            indicatorPlacement: 'inside',
                            status: 'info',
                            percentage: progress.progress,
                            color: 'rgba(37, 165, 233, 0.8)',
                            railColor: 'rgba(37, 165, 233, 0.1)'
                            
                        },
                        {
                            default:()=> h('span', {class: 'neon-blue'}, progress.left + " дн.")
                        })
                       
                    ])
                })
        }

        const statusOrdered = (ord: Status, title: string) =>
        {
            const progress = getDaysDiff(parseDate(ord.startDate), parseDate(ord.endDate));
            return h(StatusCard,
                {
                    key: ord.endDate + ord.startDate  + progress.progress + ord.place,
                    avatar: run_man_ico,
                    shadowbox_color: 'rgba(227, 142, 22, 0.6)',
                    tooltip: title + (progress.overall) +" дней, осталось " + progress.left + " дн."
                },
                {
                    default:() =>
                    h('div', 
                    {
                        style: {
                            width: '100%',
                        } as CSSProperties
                    },
                    [
                        h('div', ord.place),
                        h('div', ord.startDate + " - " + ord.endDate),
                        h(NProgress,
                            {
                                type: 'line',
                                indicatorPlacement: 'inside',
                                status: 'info',
                                percentage: progress.progress,
                                color: 'rgba(227, 142, 22, 0.6)',
                                railColor: 'rgba(227, 142, 22, 0.1)'
                               
                            },
                            {
                                default:()=> h('span', {class: 'neon-blue'}, progress.left + " дн.")
                            })
                       
                    ])
                })
        }

        const statusBuisnessTrip = (ord: Status, title: string) =>
        {
            const progress = getDaysDiff(parseDate(ord.startDate), parseDate(ord.endDate));
            return h(StatusCard,
                {
                    key: ord.endDate + ord.startDate  + progress.progress + ord.place,
                    avatar: train_ico,
                    shadowbox_color: 'rgba(22, 227, 84, 0.6)',
                    tooltip: title + (progress.overall) +" дней, осталось " + progress.left + " дн."
                },
                {
                    default:() =>
                    h('div', 
                    {
                        style: {
                            width: '100%',
                        } as CSSProperties
                    },
                    [
                        h('div', ord.place),
                        h('div', ord.startDate + " - " + ord.endDate),
                        h(NProgress,
                            {
                                type: 'line',
                                indicatorPlacement: 'inside',
                                status: 'info',
                                percentage: progress.progress,
                                color: 'rgba(22, 227, 84, 0.6)',
                                railColor: 'rgba(22, 227, 84, 0.1)'
                               
                            },
                            {
                                default:()=> h('span', {class: 'neon-blue'}, progress.left + " дн.")
                            })
                       
                    ])
                })
        }
        return {renderStatus}
    },

   
    
    render ()
    {
        return this.renderStatus()
    }
})