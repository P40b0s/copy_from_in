import 
{
    h,
    defineComponent,
    defineAsyncComponent,
    CSSProperties,
  } from 'vue'

import { NButton, NIcon, NSpin, NTooltip, NVirtualList, useNotification} from 'naive-ui';
import { DateFormat, DateTime} from '../services/date.ts';
import { app_state_store } from '../store/index.ts';
import { StatusCard } from './status_card.tsx';
import { background, envelope_ico, error_ico } from '../services/svg.ts';
import { Filter, IPacket, Task } from '../models/types.ts';
import { AlertOutline, CheckmarkDoneCircle, FlashOff, FolderOpen, MailSharp, RefreshCircleSharp, SettingsSharp, TimeOutline } from '@vicons/ionicons5';
import { service, settings } from '../services/tauri/commands.ts';
import { naive_notify } from '../services/notification.ts';


const task_1 = (): Task => 
{
    const f : Filter = 
    {
        document_types: [],
        document_uids: []
    }
    const task: Task = {
        name: "task_1",
        description: "",
        source_dir: "",
        target_dir: "",
        report_dir: "",
        timer: 120000,
        delete_after_copy: false,
        copy_modifier: "CopyAll",
        is_active: true,
        generate_exclude_file: true,
        clean_types: ["Квитанция"],
        color: '#4f46',
        filters: f
    }
    return task;
}
const task_2 = (): Task => 
{
    const f : Filter = 
    {
        document_types: [],
        document_uids: []
    }
    const task: Task = {
        name: "task_2",
        description: "",
        source_dir: "",
        target_dir: "",
        report_dir: "/sds",
        timer: 120000,
        delete_after_copy: false,
        copy_modifier: "CopyAll",
        is_active: true,
        generate_exclude_file: true,
        clean_types: ["Квитанция"],
        color: '#0a8bb2',
        filters: f
    }
    return task;
}
//тестовые данные
const test_packet1 = () =>
{
    
    const p : IPacket = {
        document:
        {
            organization: "Совет Федерации Федерального Собрания Российской Федерации",
            organizationUid: '92834908230948209348209384',
            docUid: '123234r2342342342342',
            sourceMedoAddressee: '123@123.MEDO',
            docType: "Постановление Совета Федерации Федерального Собрания Российской Федерации",
            number: "299-СФ",
            signDate: "2023-06-21",
        },
        report_sended: false,
        name: "ошибочное название директории",
        parseTime: "2024-12-24T00:00:00",
        task: task_1()
    }
    return p;
}
const test_packet2 = () =>
{
    
    const p : IPacket = {
        document:
        {
            organization: "Правительство Российской Федерации",
            docType: "Правительство Правительство Правительство Правительство Правительство Правительство Правительство Правительство Правительство",
            organizationUid: '92834908230948209348209384',
            docUid: '123234r2342342342342',
            sourceMedoAddressee: '123@123.MEDO',
            number: "299-РП",
            signDate: "2023-06-21",
        },
        report_sended: true,
        name: "ошибочное название директории",
        parseTime: "2024-12-24T00:00:00",
        task: task_2()
    }
    return p;
}
const test_error_packet = () =>
{
    const p : IPacket = {
        name: "123error_packet",
        parseTime: "2024-12-24T00:00:00",
        error: "Ошибка распознавания пакета йв3242342!",
        report_sended: false,
        task: task_1()
    }
    return p;
}
const test_error_packet2 = () =>
{
    const p : IPacket = {
        name: "err_packet",
        parseTime: "2024-12-24T00:00:00",
        report_sended: false,
        error: "Ошибка распознавания пакета! Ошибка распознавания пакета! Ошибка распознавания пакета! Ошибка распознавания пакета! Ошибка распознавания пакета! Ошибка распознавания пакета! Ошибка распознавания пакета!",
        task: task_2()
    }
    return p;
}


export const PacketsViewerAsync = defineAsyncComponent({
    loader: () => import ('./packets_viewer.tsx'),
    loadingComponent: h(NSpin)
})



export const PacketsViewer =  defineComponent({
    setup () 
    {
        const notify = useNotification();
        //для тестирования
        for (let index = 0; index < 100; index++) {
            app_state_store.add_packet(test_packet1());
            app_state_store.add_packet(test_error_packet());
            app_state_store.add_packet(test_error_packet2());
            app_state_store.add_packet(test_packet2());
        }
       
        const list = () =>
        {
            return h('div',
            {
                style:
                {
                    display: 'flex',
                    flexDirection: 'column',
                    alignItems: 'left',
                    width: '100%',
                    //background: 'conic-gradient(from 116.56deg at calc(100%/3) 0   , #0000 90deg,#046D8B 0), conic-gradient(from -63.44deg at calc(200%/3) 100%,#0000 90deg,#046D8B 0)',
                    //backgroundSize: '50px 50px'
                    backgroundImage: background
                }   as CSSProperties
            },
            virtual_list()
            );
        }
        const doc_status = (packet: IPacket) =>
        {
            const parse_date = new DateTime(packet.parseTime);
            const parse_time = parse_date.to_string(DateFormat.DotDate) + " " + parse_date.to_string(DateFormat.Time)
            return h(StatusCard,
            {
                key: parse_date.to_string(DateFormat.SerializedDateTime),
                avatar: packet.error ? error_ico : envelope_ico,
                task_color: packet.task.color,
                shadowbox_color: packet.error ? '#f6848487' : 'rgb(100, 165, 9)',
                tooltip: packet.name
            },
            {
                default:() =>
                h('div', 
                {
                    style: {
                        width: '100%',
                        display: 'flex',
                        flexDirection: 'column',
                        textAlign: 'left',
                        //background: 'linear-gradient(0.25turn, #0000004a, 90%, '+ packet.task.color + ', #ebf8e100)',
                        //background: '#00000033',
                    } as CSSProperties
                },
                [
                    h('div',
                    {
                        style:
                        {
                            fontWeight: '700',
                            fontSize: '16px'
                        } as CSSProperties,
                        //class: packet.error ? 'standart-red' : 'standart-green'
                    },
                    [
                        h('div',
                        {
                            style:
                            {
                                display: 'flex',
                                flexDirection: 'row',
                                justifyItems: 'center',
                                alignItems: 'center',
                            } as CSSProperties,
                        },
                        [
                            h('div',
                            {
                                style:
                                {
                                    display: 'flex',
                                    flexDirection: 'row',
                                    alignItems: 'center',
                                    paddingRight: '3px',
                                    borderBottom: '2px solid ' + packet.task.color,
                                    borderRight: '2px solid ' + packet.task.color,
                                } as CSSProperties
                            },
                            [
                                h(NTooltip, null,
                                {
                                    trigger:() =>
                                    h(NIcon, 
                                    {
                                        component: TimeOutline,
                                        color: 'rgb(100, 165, 9)',
                                        size:'large',
                                        style:
                                        {
                                            marginRight: '2px',
                                        } as CSSProperties,
                                    }),
                                    default:() => "Время обработки пакета"
                                }),
                                parse_time,
                            ]),
                            h('div',
                            {
                                style:
                                {
                                    display: 'flex',
                                    flexDirection: 'row',
                                    alignItems: 'center',
                                    borderBottom: '2px solid ' + packet.task.color,
                                    paddingRight: '3px',
                                    borderRight: '2px solid ' + packet.task.color,
                                } as CSSProperties
                            },
                            [
                                h(NTooltip, null,
                                {
                                    trigger:() =>
                                    h(NIcon, 
                                    {
                                        component: SettingsSharp,
                                        color: packet.task.color,
                                        style:
                                        {
                                            marginLeft: '5px',
                                            marginRight: '2px'
                                        } as CSSProperties,
                                    }),
                                    default:() => "Наименование задачи"
                                }),
                                packet.task?.name,
                            ]),
                            h('div',
                            {
                                style:
                                {
                                    display: 'flex',
                                    flexDirection: 'row',
                                    alignItems: 'center',
                                    borderBottom: '2px solid ' + packet.task.color,
                                   
                                } as CSSProperties
                            },
                            [
                                h(NTooltip, null,
                                {
                                    trigger:() =>
                                    h(NIcon, 
                                    {
                                        component: FolderOpen,
                                        color: 'rgb(241, 229, 95)',
                                        style:
                                        {
                                            marginLeft: '5px',
                                            marginRight: '2px'
                                        } as CSSProperties,
                                    }),
                                    default:() => "Наименование директории пакета"
                                }),
                                packet.name,
                            ]),
                            report_icon(packet),
                            rescan_icon(packet)
                        ]),
                        requisites_or_error(packet)
                    ])
                ])
            })
        }

        const rescan_icon = (packet: IPacket) =>
        {
            return  h('div',
            {
                style:
                {
                    flexGrow: 1,
                    justifyContent: 'end',
                    display: 'flex',
                    visibility: packet.error ? 'visible' : 'collapse'


                } as CSSProperties
            },
            h(NTooltip, 
                {
                   
                },
                {
                    trigger:() =>
                    h(NButton,
                    {
                        text: true,
                        size: 'large',
                        onClick: async (e) =>
                        {
                            const res = await service.rescan_packet(packet)
                            if (res.is_err())
                                naive_notify(notify, 'error', "Ошибка запроса раскана пакета", res.get_error());
                        },
                    },
                    {
                        icon:() =>
                        h(NIcon, 
                            {
                                component: RefreshCircleSharp,
                                color: 'rgb(221, 245, 41)',
                            }),
                    }),
                    default:() => "Пересканировать текущий пакет"
                }),
            ) 
        }

        const report_icon = (packet: IPacket) =>
        {
            return packet.task.report_dir != "" ?
            h('div',
            {
                style:
                {
                    position: 'absolute',
                    left: '15px'
                } as CSSProperties
            },
                packet.report_sended ?
                h(NTooltip, null,
                {
                    trigger:() =>
                    h(NIcon, 
                    {
                        component: CheckmarkDoneCircle,
                        color: 'rgb(100, 165, 9)',
                        size:'large',
                        style:
                        {
                            marginRight: '2px',
                        } as CSSProperties,
                    }),
                    default:() => "Уведомление успешно отправлено"
                }) :
                h(NTooltip, null,
                {
                    trigger:() =>
                    h(NIcon, 
                    {
                        component: FlashOff,
                        color: 'rgb(165, 11, 9)',
                        size:'large',
                        style:
                        {
                            marginRight: '2px',
                        } as CSSProperties,
                    }),
                    default:() => "Ошибка отправки уведомления"
                }),
            ) : []
        }


        const requisites_or_error = (packet: IPacket) =>
        {
            let description : string|undefined;
            if(packet.document)
            {
                const sign_date = packet.document.signDate ? new DateTime(packet.document.signDate) : undefined;
                description = (packet.document.organization ?? "") + " " + (sign_date?.to_string(DateFormat.DotDate) ?? "") + " " + (packet.document.number ?? "")
                return h('div',
                {
                    style:
                    {
                        display: 'flex',
                        flexDirection: 'row',
                        alignItems: 'center',
                        //border: '1px solid ' + packet.task?.color ?? 'rgb(100, 165, 9)',
                    } as CSSProperties
                },
                [
                    h(NTooltip, null,
                    {
                        trigger:() =>
                        h(NIcon,
                        {
                            component: MailSharp,
                            color: packet.task.color,
                            style:
                            {
                                marginRight: '2px'
                            } as CSSProperties,
                        }),
                        default:() => "Реквизиты документа"
                    }),
                    (packet.document.organization ?? "") + " " + (sign_date?.to_string(DateFormat.DotDate) ?? "") + " " + (packet.document.number ?? "")
                ])
            }
            else
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
                    h(NTooltip, null,
                    {
                        trigger:() =>
                        h(NIcon, 
                        {
                            component: AlertOutline,
                            color: 'rgb(239, 67, 67)',
                            style:
                            {
                                marginRight: '2px'
                            } as CSSProperties,
                        }),
                        default:() => "Ошибка парсинга пакета"
                    }),
                    packet.error
                ])
            }
        }
        const virtual_list = () =>
        {
            return h(NVirtualList,
                {
                    style:
                    {
                        maxHeight: "600px",
                        minHeight: '600px',
                        padding: '10px'
                    } as CSSProperties,
                    trigger: 'hover',
                    itemSize: 70,
                    items: app_state_store.getState().current_log
                },
                {
                    default:({ item }: {item: IPacket}) => 
                    {
                        return doc_status(item);
                    }
                })
            }
        return {list}
    },
    render ()
    {
        return this.list()
    }
})