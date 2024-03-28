import 
{
    h,
    defineComponent,
    defineAsyncComponent,
    CSSProperties,
  } from 'vue'

import { NIcon, NSpin, NTooltip, NVirtualList} from 'naive-ui';
import { DateFormat, DateTime} from '../services/date.ts';
import { app_state_store } from '../store/index.ts';
import { StatusCard } from './status_card.tsx';
import { envelope_ico, error_ico } from '../services/svg.ts';
import { Filter, IPacket, Task } from '../models/types.ts';
import { AlertOutline, FolderOpen, MailSharp, SettingsSharp, TimeOutline } from '@vicons/ionicons5';


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
        timer: 120000,
        delete_after_copy: false,
        copy_modifier: "CopyAll",
        is_active: true,
        generate_exclude_file: true,
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
        timer: 120000,
        delete_after_copy: false,
        copy_modifier: "CopyAll",
        is_active: true,
        generate_exclude_file: true,
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
            name: "название_директории",
            organization: "Совет Федерации Федерального Собрания Российской Федерации",
            docType: "Постановление Совета Федерации Федерального Собрания Российской Федерации",
            number: "299-СФ",
            signDate: "2023-06-21",
            parseTime: "2024-03-20T16:52:51"
        },
        task: task_1()
    }
    return p;
}
const test_packet2 = () =>
{
    
    const p : IPacket = {
        document:
        {
            name: "название_директории2",
            organization: "Правительство Российской Федерации",
            docType: "Правительство",
            number: "299-РП",
            signDate: "2023-06-21",
            parseTime: "2024-03-20T16:52:51"
        },
        task: task_2()
    }
    return p;
}
const test_error_packet = () =>
{
    const p : IPacket = {
        document:
        {
            name: "ошибочное название директории",
            parseTime: "2024-12-24T00:00:00"
        },
        error: "Ошибка распознавания пакета йв3242342!",
        task: task_1()
    }
    return p;
}
const test_error_packet2 = () =>
{
    const p : IPacket = {
        error: "Ошибка распознавания пакета!",
        task: task_2()
    }
    return p;
}


export const LogViewerAsync = defineAsyncComponent({
    loader: () => import ('./log_viewer.tsx'),
    loadingComponent: h(NSpin)
})



export const LogViewer =  defineComponent({
    setup () 
    {
        //для тестирования
        // for (let index = 0; index < 100; index++) {
        //     app_state_store.add_packet(test_packet1());
        //     app_state_store.add_packet(test_error_packet());
        //     app_state_store.add_packet(test_error_packet2());
        //     app_state_store.add_packet(test_packet2());
        // }

        const list = () =>
        {
            return h('div',
            {
                style:
                {
                    display: 'flex',
                    flexDirection: 'column',
                    alignItems: 'left',
                    width: '100%'
                }   as CSSProperties
            },
            virtual_list()
            );
        }
        const doc_status = (packet: IPacket) =>
        {
            const parse_date = new DateTime(packet.document?.parseTime);
            const parse_time = parse_date.to_string(DateFormat.DotDate) + " " + parse_date.to_string(DateFormat.Time)
            return h(StatusCard,
            {
                key: packet.document?.parseTime ?? parse_date.to_string(DateFormat.SerializedDateTime),
                avatar: packet.error ? error_ico : envelope_ico,
                task_color: packet.task?.color,
                shadowbox_color: packet.error ? '#f6848487' : 'rgb(100, 165, 9)',
                tooltip: packet.document?.name || packet.error || "Неизвестный пакет"
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
                        background: 'linear-gradient(0.25turn, #00000033, 90%, '+ packet.task?.color + ', #ebf8e100)',
                        //background: '#00000033',
                    } as CSSProperties
                },
                [
                    h('div',
                    {
                        style:
                        {
                            fontWeight: '700',
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
                                    borderBottom: '1px solid ' + packet.task?.color ?? 'rgb(100, 165, 9)',
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
                                    borderBottom: '1px solid ' + packet.task?.color ?? 'rgb(100, 165, 9)',
                                } as CSSProperties
                            },
                            [
                                h(NTooltip, null,
                                {
                                    trigger:() =>
                                    h(NIcon, 
                                    {
                                        component: SettingsSharp,
                                        color: packet.task?.color ?? 'rgb(100, 165, 9)',
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
                            packet.document ? 
                            h('div',
                            {
                                style:
                                {
                                    display: 'flex',
                                    flexDirection: 'row',
                                    alignItems: 'center',
                                    borderBottom: '1px solid ' + packet.task?.color ?? 'rgb(100, 165, 9)',
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
                                packet.document?.name,
                            ]): []
                        ]),
                        requisites_or_error(packet)
                    ])
                ])
            })
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
                            color: packet.task?.color ?? 'rgb(100, 165, 9)',
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