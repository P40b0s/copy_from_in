import 
{
    h,
    defineComponent,
    ref,
    watch,
    CSSProperties,
    PropType,
    toRaw,
    Ref,
    toRef,
  } from 'vue'
import { NButton, NIcon, NInput, NProgress, NTooltip, NotificationType, useNotification} from 'naive-ui';
import { timer } from '../../services/helpers'
import { global_store } from '../../store';
import {WarningOutline} from '@vicons/ionicons5'
import { TimeWarning } from '../../models/user';
import { TimeProgress, dateToString, timeLeft } from '../../services/date';
import { match } from 'ts-pattern';
import { sortTimeWarnings } from '../../services/data';
const localProps = 
{
    /**Список задач */
    items: 
    {
        type: Array as PropType<TimeWarning[]>,
        required: true
    },
    /**Время в минутах за которое начнут показываться уведомления (например если в задаче указано время 9:30 а notifyTimeout стоит 30 мин, то уведомления начнут показываться в период с 9:00 до 9:30) */
    notifyTimeout: 
    {
        type: Number,
        default: 30
    },
    /**Уведомления так же будут показываться с этой переодичность(т.е. один раз в этот интервал) */
    notifyShowDelay: 
    {
        type: Number,
        default: 5
    },
} as const

export const TimeWarningsViewer = defineComponent({
name: 'TimeWarningsViewer',
props: localProps,
emits:
{
    'update:items': (values: TimeWarning[]) => values
},
    setup (props, {emit}) 
    {
        const items = toRef(props, 'items') as Ref<(TimeWarning & TimeProgress)[]>;
        const notification = useNotification();
        const current_date = ref(new Date());
        const refreshTimeProgress = (t: TimeWarning & TimeProgress) =>
        {
            const tl = timeLeft(t.warningTime);
            if(tl)
            {
                t.progress = tl.progress;
                t.minutes_left = tl.minutes_left,
                t.minutes = tl.minutes;
                t.hours = tl.hours;
            }
        }

        const notify =  (type: NotificationType, tw: TimeWarning & TimeProgress) => 
        {
            const n = notification.create({
                type: type,
                title: "Осталось " + tw.minutes_left + " мин.",
                description: "Автоматическое напоминание на " + tw.time,
                content: tw.text,
               
                //meta: tw.id.toString(),
                action: () =>
                    h(
                    NButton,
                    {
                        text: true,
                        type: 'primary',
                        onClick: () => 
                        {
                            tw.showNotify = false;
                            n.destroy()
                        }
                    },
                    {
                        default: () => 'Больше не показывать'
                    }
                    ),
                avatar:() => h(NProgress,
                    {
                        style:{
                            width: '100px',
                            marginBottom: '20px'
                        } as CSSProperties,
                        type: 'circle',
                        //circleGap: 0.6,
                        //strokeWidth: 10,
                        status: 'info',
                        indicatorPosition: 'inside',
                        
                        percentage: tw.progress,
                        color: 'rgba(255, 43, 15, 0.8)',
                        railColor: 'rgba(84, 237, 33, 0.8)'
                        
                    },
                    {
                        default:()=> h(NIcon,
                        {
                            style:{
                                marginTop: '14px'
                            } as CSSProperties,
                            size: '20',
                            color: 'rgba(255, 43, 15, 0.8)',
                            component: WarningOutline
                        }),
                    }
                    ),
                duration: 25500,
                keepAliveOnHover: true
            })
        }
        console.log("Уведомления начнут показываться за " + props.notifyTimeout + " минут до указанного в правилах времени, c переодичностю в " +props.notifyShowDelay+ " минут")
        const showTimeWarningsNotify = (id: string, show: boolean) => 
        {
          items.value.filter(f=>f.id == id)[0].showNotify = show;
        }
        watch(() => global_store.getState().currentDate, (old, n) =>
        {
            current_date.value = new Date();
            if(current_date.value.getHours() == 0 && current_date.value.getMinutes() == 0)
            {
              console.log("наступила полночь!", n, global_store.getState().currentDate)
              items.value.forEach(f=>f.showNotify = true);
            }
            //if(cur_date.getMinutes() % props.checkTimeout == 0)
            {
                checkTimeWarnings();
                console.log("проверка списка!")
            }
        })
        watch(() => props.items, (old, n) =>
        {
            checkTimeWarnings();
        })
        
        const date_time_string = (tw: TimeWarning) =>
        {
            let ts = "";
            if(tw.date)
            {
                if(tw.time)
                    ts = ts + tw.time;  
                else
                ts = ts + dateToString(tw.date);
            }
            else
            if(tw.time)
                ts = ts + tw.time;  
            return ts;
        }
        const check_notify = (t: TimeWarning & TimeProgress) => 
        {
            if (t.startNotifyTime && (current_date.value.getMinutes() % props.notifyShowDelay == 0))
            {
                const c_time = new Date().setSeconds(0, 0);
                if( (c_time >= t.startNotifyTime) && t.showNotify)
                {
                    notify('warning', t)
                    
                }
            }
        }

        const checkTimeWarnings = () => 
        {
            items.value.forEach(t=>
            {
                if(t.time)
                {
                    refreshTimeProgress(t);
                    if (t.date)
                    {
                        if(t.date.getTime() == global_store.getState().currentDate.setHours(0, 0, 0, 0)) 
                        {
                            set_visibility(t);
                        }
                    } 
                    else if(t.weekDay)
                    {
                        if(t.weekDay.some(s=>s == global_store.getState().weekDay))
                        {
                            console.log(global_store.getState())
                            set_visibility(t);
                        }
                        else
                        {
                            t.isVisible = false;
                        }
                    }
                    else
                    {
                        set_visibility(t);
                    }
                }
                else
                {
                    if(t.weekDay && t.weekDay.some(s=>s == global_store.getState().weekDay))
                    {
                        t.isVisible = true;
                    }
                    else if(t.date && t.date.getTime() == global_store.getState().currentDate.setHours(0, 0, 0, 0)) 
                    {
                        t.isVisible = true;
                    }
                }
            })
        }
        const set_visibility = (tw: TimeWarning & TimeProgress) =>
        {
            const c_time = new Date().setSeconds(0, 0);
            if(tw.warningTime as number < c_time)
                tw.isVisible = false;
            else
            {
                tw.isVisible = true;
                check_notify(tw);
            }
        }
        
        checkTimeWarnings();
        
        const progress_bar = (tw: TimeWarning & TimeProgress) =>
        {
            return h(NTooltip,{},
            {
                default: ()=> tw.hours == 0 ? "Осталось " + tw.minutes_left + " мин." : "Осталось " + tw.hours + " час. " + tw.minutes + " мин.",
                trigger:() => 
                h(NProgress,
                {
                    style:{
                        width: '30px'
                    } as CSSProperties,
                    type: 'circle',
                    //circleGap: 0.6,
                    strokeWidth: 10,
                    status: 'info',
                    percentage: tw.progress,
                    color: 'rgba(255, 43, 15, 0.8)',
                    railColor: 'rgba(84, 237, 33, 0.8)'
                    
                },
                {
                    default:()=> h(NIcon,
                    {
                        style:{
                            marginBottom: '2px'
                        } as CSSProperties,
                        size: '20',
                        color: 'yellow',
                        component: WarningOutline
                    }),
                }),
            })
        }

        const viewer = () =>
        {
            return h('div',
            {
                style:
                {
                    gap: '2px',

                } as CSSProperties
            },
                items.value.map(tw=>
                {
                    return tw.isVisible ?
                        h('div',{},
                        [
                            h('div',
                        {
                            style:
                            {
                                display: 'flex',
                                flexDirection: 'row',
                                alignItems: 'center',
                                fontSize:'18px',
                                gap: '10px'
                            } as CSSProperties
                        },
                        [
                            (tw.minutes_left != undefined && tw.minutes_left != 0) ? progress_bar(tw) : 
                                h(NIcon,
                                {
                                    style:{
                                        marginBottom: '2px',
                                        marginLeft: '3px'
                                    } as CSSProperties,
                                    size: '25',
                                    color: 'yellow',
                                    component: WarningOutline
                                }),
                            h('div',
                            {
                                style:
                                {
                                    fontWeight: '700',
                                    color: 'green'
                                } as CSSProperties

                            }, date_time_string(tw)),
                            h('div', tw.text) 
                        ]),
                    ])
                    : []
                })
            )
        }
        return {viewer}
    },
    render ()
    {
        return this.viewer()
    }
})

