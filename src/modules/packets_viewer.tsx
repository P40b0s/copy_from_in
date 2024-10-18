import 
{
    h,
    defineComponent,
    defineAsyncComponent,
    CSSProperties,
    ref,
    onMounted,
    onUnmounted,
    watch,
    watchEffect,
  } from 'vue'

import { NButton, NIcon, NPagination, NPopconfirm, NScrollbar, NSpin, NTooltip, NVirtualList, useNotification} from 'naive-ui';
import { DateFormat, DateTime} from '../services/date.ts';
import { app_state_store } from '../store/index.ts';
import { StatusCard } from './status_card.tsx';
import { background, envelope_ico, error_ico } from '../services/svg.ts';
import { Filter, IPacket, Task } from '../models/types.ts';
import { AlertOutline, CheckmarkDoneCircle, FlashOff, FolderOpen, MailSharp, RefreshCircleSharp, SettingsSharp, TimeOutline, TrashBin } from '@vicons/ionicons5';
import { commands_packets, commands_service, commands_settings } from '../services/tauri/commands.ts';
import { naive_notify } from '../services/notification.ts';
import { events } from '../services/tauri/events.ts';


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
        const state = app_state_store.getState();
        const packets = ref<IPacket[]>([]);

        const get_packets = async () =>
        {
            total_count.value = await get_pages_count();
            let r = await commands_packets.get_packets_list(items_on_page, current_offset);
            if(r.is_ok())
                packets.value = r.get_value();
        }
        const new_packet_event = events.packets_update(async (packet) => 
        {
            if (packet.payload.task.visible)
            {
                const exist_index = packets.value.findIndex(f=>f.id == packet.payload.id);
                if (exist_index >= 0)
                {
                    naive_notify(notify, 'info', `Пакет ${packet.payload.name}-${packet.payload.id} был заменен`, "После пересканирования пакет был заменен");
                    packets.value.splice(exist_index, 1, packet.payload);
                }
                else if(current_page.value == 1)
                {
                    packets.value.splice(0, 0, packet.payload);
                    if(packets.value.length > items_on_page)
                        packets.value.pop();
                }
            }
        })
        watch(() => state.server_is_online, async (new_state, old_state) => 
        {   
            if(!old_state && new_state && packets.value.length > 0)
            {
                await get_packets();
            }
        })
        const update_packets_event = events.need_packets_refresh(async () => 
        {
            await get_packets();
        })
        const vis_change = async () =>
        {
            if (!document.hidden) 
            {
                await get_packets();
            }
        }
        document.addEventListener("visibilitychange", () => vis_change());
        onUnmounted(()=>
        {
            new_packet_event.then(u=> u.unsubscribe());
            update_packets_event.then(u=> u.unsubscribe())
            document.removeEventListener('visibilitychange', () => vis_change());
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
            return h('div', {
                style:
                {
                    display: 'flex',
                    flexDirection: 'column',
                    width: '100%',
                    height: '100%',
                    justifyContent: 'space-between'
                }   as CSSProperties
            },
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
                del_button(packet),
                report_icon(packet)
            ])
        }

        const del_button =(packet: IPacket) =>
        {
            return h(NPopconfirm,
            {
                style:
                {
                    
                } as CSSProperties,
                positiveText: "Удалить",
                onPositiveClick: async () => 
                {
                    let dl = await commands_service.delete_packet(packet)
                    if (dl.is_err())
                    {
                        naive_notify(notify, 'error', "Ошибка удаления пакета " + packet.name, () => 
                        {
                            return h('div', 
                            {
                                style:
                                {
                                    color: 'red'
                                } as CSSProperties,
                            },
                            dl.get_error()
                            );
                        });
                    }
                    else
                    {
                        const index = packets.value.findIndex(i=> i.id == packet.id);
                        packets.value.splice(index, 1);
                    }
                }
            },
            {
                trigger:() =>  h(NTooltip,null,
                {
                    trigger:() =>  h(NButton,
                    {
                        type: 'error',
                        quaternary: true,
                        circle: true,
                        color: "#d90d0d",
                        style:
                        {
                            marginLeft: '5px',
                        }    as CSSProperties,
                    },
                    {
                        icon:() => h(NIcon, {component: TrashBin})
                    }),
                    default:() => "Удалить пакет"
                }),
                default:() => `При удалении пакет ${packet.name} будет удален физически с диска!`
            })
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
            const report_not_needed = () =>
            {
                return h(NTooltip, null,
                    {
                        trigger:() =>
                        h(NIcon, 
                        {
                            component: CheckmarkDoneCircle,
                            color: '#878787',
                            size: 'large',
                            style:
                            {
                                marginRight: '2px',
                            } as CSSProperties,
                        }),
                        default:() => "Для данного типа задания уведомление не требуется"
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
                if(packet.task.copy_modifier == 'CopyAll')
                {
                    return report_not_needed();
                }
                else if (report_sended)
                {
                    return report_sended_icon();
                } 
                else if (error_sended)
                {
                    return report_not_sended_icon();
                }
                return report_not_needed();
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