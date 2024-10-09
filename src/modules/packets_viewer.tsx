import 
{
    h,
    defineComponent,
    defineAsyncComponent,
    CSSProperties,
    ref,
    onMounted,
    onUnmounted,
  } from 'vue'

import { NButton, NIcon, NPagination, NScrollbar, NSpin, NTooltip, NVirtualList, useNotification} from 'naive-ui';
import { DateFormat, DateTime} from '../services/date.ts';
import { app_state_store } from '../store/index.ts';
import { StatusCard } from './status_card.tsx';
import { background, envelope_ico, error_ico } from '../services/svg.ts';
import { Filter, IPacket, Task } from '../models/types.ts';
import { AlertOutline, CheckmarkDoneCircle, FlashOff, FolderOpen, MailSharp, RefreshCircleSharp, SettingsSharp, TimeOutline } from '@vicons/ionicons5';
import { commands_packets, commands_service } from '../services/tauri/commands.ts';
import { naive_notify } from '../services/notification.ts';
import { events } from '../services/tauri/events.ts';


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
        sound: true,
        autocleaning: false,
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
        sound: true,
        autocleaning: false,
        clean_types: ["Квитанция"],
        color: '#0a8bb2',
        filters: f
    }
    return task;
}
//тестовые данные
// const test_packet1 = () =>
// {
    
//     const p : IPacket = {
//         document:
//         {
//             organization: "Совет Федерации Федерального Собрания Российской Федерации",
//             organizationUid: '92834908230948209348209384',
//             docUid: '123234r2342342342342',
//             sourceMedoAddressee: '123@123.MEDO',
//             docType: "Постановление Совета Федерации Федерального Собрания Российской Федерации",
//             number: "299-СФ",
//             signDate: "2023-06-21",
//         },
//         reportSended: false,
//         name: "ошибочное название директории",
//         parseTime: "2024-12-24T00:00:00",
//         task: task_1()
//     }
//     return p;
// }
// const test_packet2 = () =>
// {
    
//     const p : IPacket = {
//         document:
//         {
//             organization: "Правительство Российской Федерации",
//             docType: "Правительство Правительство Правительство Правительство Правительство Правительство Правительство Правительство Правительство",
//             organizationUid: '92834908230948209348209384',
//             docUid: '123234r2342342342342',
//             sourceMedoAddressee: '123@123.MEDO',
//             number: "299-РП",
//             signDate: "2023-06-21",
//         },
//         reportSended: true,
//         name: "ошибочное название директории",
//         parseTime: "2024-12-24T00:00:00",
//         task: task_2()
//     }
//     return p;
// }


export const PacketsViewerAsync = defineAsyncComponent({
    loader: () => import ('./packets_viewer.tsx'),
    loadingComponent: h(NSpin)
})

//Если компонент активен, то поддерживается добавление нового найденого пакета в список
export const PacketsViewer =  defineComponent({
    async setup() 
    {
        const notify = useNotification();
        const current_page = ref(1);
        const items_on_page = 20;
        const total_count = ref(0);
        let current_offset = 0;
        const packets = ref<IPacket[]>([]);
        // onMounted(async ()=>
        // {
        //     total_count.value = await get_pages_count();
        //     let r = await commands_packets.get_packets_list(items_on_page, current_offset);
        //     if(r.is_ok())
        //         packets.value = r.get_value();
                    
        // })
        const get_packets = async () =>
        {
            total_count.value = await get_pages_count();
            let r = await commands_packets.get_packets_list(items_on_page, current_offset);
            if(r.is_ok())
                packets.value = r.get_value();
        }
        const new_packet_event = events.packets_update(async (packet) => 
        {
            const exist_index = packets.value.findIndex(f=>f.id == packet.payload.id);
            if (exist_index >= 0)
            {
                packets.value.splice(exist_index, 1, packet.payload);
            }
            //куда то пропадают из списка 
            //сначала появляется 4 а на его месте 5
            else if(current_page.value == 1)
            {
                packets.value.splice(0, 0, packet.payload);
                if(packets.value.length > items_on_page)
                    packets.value.pop();
            }
        })
        const update_packets_event = events.need_packets_refresh(async () => 
        {
            await get_packets();
        })
        onUnmounted(()=>
        {
            new_packet_event.then(u=> u.unsubscribe());
            update_packets_event.then(u=> u.unsubscribe())
        })
        const get_pages_count = async () : Promise<number> =>
        {
            const c = await commands_packets.get_count();
            if (c.is_ok())
            {
                return c.get_value();
            }
            else
            {
                console.log(c.get_error());
                return 0;
            }
        }

        await get_packets();
        const complex = () =>
        {
            return h('div',
            [
                h(list),
                h(NPagination,
                {
                    itemCount: total_count.value,
                    pageSizes: [items_on_page],
                    showSizePicker: false,
                    simple: true,
                    page: current_page.value,
                    onUpdatePage: async (page) => 
                    {
                        current_page.value = page;
                        current_offset = (page - 1) * items_on_page;
                        total_count.value = await get_pages_count();
                        let r = await commands_packets.get_packets_list(items_on_page, current_offset);
                        if(r.is_ok())
                            packets.value = r.get_value();
                    },
                },
                {
                    
                })

            ])
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
                    height: '100%',
                    //background: 'conic-gradient(from 116.56deg at calc(100%/3) 0   , #0000 90deg,#046D8B 0), conic-gradient(from -63.44deg at calc(200%/3) 100%,#0000 90deg,#046D8B 0)',
                    //backgroundSize: '50px 50px'
                    backgroundImage: background
                }   as CSSProperties
            },
            h(NScrollbar,
                {
                   style:{
                    maxHeight: '78vh',
                   } as CSSProperties
                },
                {
                    default:() => packets.value.map(p =>
                    {
                        return doc_status(p);
                    })
                })
            );
        }
        const doc_status = (packet: IPacket) =>
        {
            const parse_date = new DateTime(packet.parseTime);
            const parse_time = parse_date.to_string(DateFormat.DotDate) + " " + parse_date.to_string(DateFormat.Time)
            return h(StatusCard,
            {
                key: parse_date.to_string(DateFormat.SerializedDateTime),
                avatar: packet.packetInfo?.error ? error_ico : envelope_ico,
                task_color: packet.task.color,
                shadowbox_color: packet.packetInfo?.error ? '#f6848487' : 'rgb(100, 165, 9)',
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
                                    default:() => "Время доставки пакета"
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
                                    default:() => packet.task.description
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
                            right_icons_panel(packet)
                        ]),
                        requisites_or_error(packet)
                    ])
                ])
            })
        }

        const right_icons_panel = (packet: IPacket) =>
        {
            const disabled = ref(false);
            return  h('div',
            {
                style:
                {
                    flexGrow: 1,
                    justifyContent: 'end',
                    display: 'flex',
                } as CSSProperties
            },
            [
                h(NTooltip, 
                {
                   
                },
                {
                    trigger:() =>
                    h(NButton,
                    {
                        style:
                        {
                            visibility: packet.packetInfo?.error ? 'visible' : 'collapse',
                            marginRight: '2px',
                            
                        } as CSSProperties,
                        disabled: disabled.value,
                        text: true,
                        size: 'large',
                        onClick: async (e) =>
                        {
                            const res = await commands_service.rescan_packet(packet)
                            if (res.is_err())
                            {
                                naive_notify(notify, 'error', "Ошибка запроса пересканирования пакета " + packet.name, res.get_error());
                            }
                            else
                            {
                                disabled.value = true;
                                //const index = packets.value.findIndex(f=> f.packetInfo?.headerGuid == packet.packetInfo?.headerGuid);
                                //packets.value.splice(index, 0);
                            }
                                

                        },
                    },
                    {
                        icon:() =>
                        h(NIcon, 
                            {
                                component: RefreshCircleSharp,
                                color: 'rgb(67, 237, 29)',
                            }),
                    }),
                    default:() => "Пересканировать текущий пакет"
                }),
                report_icon(packet)
            ])
        }

        const report_icon = (packet: IPacket) =>
        {
            const report_sended_icon = () =>
            {
                return h(NTooltip, null,
                    {
                        trigger:() =>
                        h(NIcon, 
                        {
                            component: CheckmarkDoneCircle,
                            color: 'rgb(67, 237, 29)',
                            size: 'large',
                            style:
                            {
                                marginRight: '2px',
                            } as CSSProperties,
                        }),
                        default:() => "Уведомление успешно отправлено"
                    });
            }
            const report_not_sended_icon = () =>
            {
                return h(NTooltip, null,
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
                })
            }
            const report_sended = packet.task.report_dir != "" && packet.reportSended && packet.packetInfo != undefined;
            const error_sended = packet.task.report_dir != "" && !packet.reportSended && packet.packetInfo != undefined;

            const icon = () =>
            {
                if (report_sended)
                {
                    return report_sended_icon();
                } 
                if (error_sended)
                {
                    return report_not_sended_icon();
                }
                if(packet.task.copy_modifier == 'CopyAll')
                {
                    return []
                }
                //это значит что модификатор copyall
                return [];
            }

            return h('div',
            {
                style:
                {
                    display: 'flex',
                    flexDirection: 'row',
                    alignItems: 'center',
                    paddingRight: '3px',
                } as CSSProperties
            },
            icon())
        }


        const requisites_or_error = (packet: IPacket) =>
        {
            let description : string|undefined;
            if(packet.packetInfo && !packet.packetInfo.error)
            {
                const sign_date = packet.packetInfo.requisites?.signDate ? new DateTime(packet.packetInfo.requisites?.signDate) : undefined;
                description = (packet.packetInfo.senderInfo?.organization ?? "") + " " + (sign_date?.to_string(DateFormat.DotDate) ?? "") + " " + (packet.packetInfo.requisites?.documentNumber ?? "")
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
                    (packet.packetInfo.senderInfo?.organization ?? "") + " " + (sign_date?.to_string(DateFormat.DotDate) ?? "") + " " + (packet.packetInfo.requisites?.documentNumber ?? "")
                ])
            }
            else if (packet.packetInfo?.error)
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
                    packet.packetInfo?.error
                ])
            }
            //если нет ошибок и документа значит документ копируется с опцией CopyAll
            //в этом случае ошибки парсинга пакета  не имеют значения 
            else
            {
                return [];
            }
        }
        const virtual_list = () =>
        {
            return h(NVirtualList,
                {
                    style:
                    {
                        maxHeight: "600px",
                        minHeight: '900px',
                        padding: '10px'
                    } as CSSProperties,
                    trigger: 'hover',
                    itemSize: 80,
                    items: packets.value
                },
                {
                    default:({ item }: {item: IPacket}) => 
                    {
                        console.error(item);
                        return doc_status(item);
                    }
                })
            }
            const packets_list = () =>
            {
                
                return h(NVirtualList,
                    {
                        style:
                        {
                            maxHeight: "600px",
                            minHeight: '900px',
                            padding: '10px'
                        } as CSSProperties,
                        trigger: 'hover',
                        itemSize: 80,
                        items: packets.value
                    },
                    {
                        default:({ item }: {item: IPacket}) => 
                        {
                            console.error(item);
                            return doc_status(item);
                        }
                    })
                }
        return {list, complex}
    },
    render ()
    {
        return this.complex()
    }
})