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

import { NAvatar, NButton, NIcon, NPagination, NPopconfirm, NScrollbar, NSpin, NTooltip, NVirtualList, useNotification} from 'naive-ui';
import { DateFormat, DateTime} from '../services/date.ts';
import { app_state_store } from '../store/index.ts';
import { StatusCard } from './status_card.tsx';
import { background, envelope_ico, error_ico, image_ico, pdf_ico } from '../services/svg.ts';
import { Filter, IPacket, Task } from '../models/types.ts';
import { AlertOutline, CheckmarkDoneCircle, FlashOff, FolderOpen, MailSharp, MenuOutline, RefreshCircleSharp, SettingsSharp, TimeOutline, TrashBin } from '@vicons/ionicons5';
import { commands_packets, commands_service, commands_settings } from '../services/tauri/commands.ts';
import { naive_notify } from '../services/notification.ts';
import { events } from '../services/tauri/events.ts';
import { LiveSearch } from './live_search.tsx';
import { type Emitter, type Events } from "../services/emit";
import emitter from '../services/emit.ts';
import { sleepNow } from '../services/helpers.ts';
import { Senders } from '../models/senders.ts';
import { useSenders } from './Senders/senders.ts';

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
        const scrollbar_ref = ref();
        const search_value = ref("");
        //сейчас выполняется поиск
        const in_search = ref(false);
        //количество найденных значений
        const searched_count = ref(0);
        const { get_icon, get_senders, get_organization } = useSenders();
        const get_packets = async () =>
        {
            await get_senders();
            total_count.value = await get_pages_count();
            let r = await commands_packets.get_packets_list(items_on_page, current_offset);
            if(r.is_ok())
                packets.value = r.get_value();
            //console.log(packets.value);
        }
        
        const new_packet_event = events.packets_update(async (packet) => 
        {
            if (packet.payload.task.visible && !in_search.value)
            {
                const exist_index = packets.value.findIndex(f=>f.id == packet.payload.id);
                if (exist_index >= 0)
                {
                    naive_notify(notify, 'info', `Пакет ${packet.payload.name} задачи ${packet.payload.task} был заменен`, "После пересканирования пакет был заменен");
                    packets.value.splice(exist_index, 1, packet.payload);
                }
                else if(current_page.value == 1)
                {
                    packets.value.splice(0, 0, packet.payload);
                    if(packets.value.length > items_on_page)
                        packets.value.pop();
                    total_count.value++;
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
                    disabled: in_search.value,
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
                        scrollbar_ref.value.scrollTo({top: 0})
                    },
                    style:
                    {
                        marginTop: '5px',
                        marginBottom: '5px'
                    }
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
                    //backgroundImage: background
                }   as CSSProperties
            },
            [
                //live search block
                h('div',
                {
                    style:
                    {
                        display: 'flex',
                        flexDirection: 'row',
                        marginLeft: '5px',
                        //background: 'conic-gradient(from 116.56deg at calc(100%/3) 0   , #0000 90deg,#046D8B 0), conic-gradient(from -63.44deg at calc(200%/3) 100%,#0000 90deg,#046D8B 0)',
                        //backgroundImage: "linear-gradient(90deg, #303131ad 0%, #425b5c 51%, #61e1e5 100%)"
                        backgroundColor: "rgba(97, 225, 229, 0.85)",
                        padding: "2px",
                        background: "rgba(97, 225, 229, 0.85)",
                        boxShadow: "0 8px 32px 0 rgba( 31, 38, 135, 0.37 )",
                        backdropFilter: "blur( 8px )",
                        "-webkit-backdrop-filter": "blur( 8px )",
                        borderRadius: "10px",
                        border: "1px solid rgba( 255, 255, 255, 0.18 )"
                    } as CSSProperties
                },
                [
                    h(LiveSearch,
                    {
                        value: search_value.value,
                        "onUpdate:value": async (s: string) => 
                        {
                            if (s.length == 0)
                            {
                            packets.value = [];
                            await get_packets();
                            in_search.value = false;
                            searched_count.value = 0;
                            }
                            else
                            {
                                in_search.value = true;
                                packets.value = [];
                                const founded = await commands_packets.search_packets(s);
                                if(founded.is_ok())
                                {
                                    packets.value = founded.get_value()
                                    searched_count.value = packets.value.length;
                                } 
                            }
                            
                        },
                        style:
                        {
                            width: '50vw',
                            fontSize:"16px",
                            fontWeight: "100",
                            flexGrow: '2',
                            color: 'black !important!',
                            backgroundColor: "rgba(34, 96, 98, 0.86)",
                            padding: "2px",
                            background: "rgba(34, 96, 98, 0.85)",
                            boxShadow: "0 8px 32px 0 rgba( 31, 38, 135, 0.37 )",
                            backdropFilter: "blur( 8px )",
                            "-webkit-backdrop-filter": "blur( 8px )",
                            borderRadius: "10px",
                            border: "1px solid rgba(56, 194, 199, 0.85)"
                            
                        } as CSSProperties
                    }),
                
                    h('div', 
                    {
                        style:
                        {
                            visibility: in_search.value ? 'visible' : 'hidden',
                            alignSelf: 'center',
                            textAlign: 'left',
                            fontSize:"16px",
                            marginLeft: '5px',
                            flexGrow: '2',
                            color: 'black',
                            fontWeight: "700",
                        } as CSSProperties
                    },
                    "Найдено: "+ searched_count.value),
                    h('div', 
                    {
                        style:
                        {
                            visibility: in_search.value ? 'hidden' : 'visible',
                            alignSelf: 'center',
                            fontSize:"16px",
                            flexGrow: '1',
                            color: 'black',
                            fontWeight: "700",
                        } as CSSProperties
                    },
                    "Всего: "+ total_count.value),
                ]),
               
                h(NScrollbar,
                {
                    style:
                    {
                        maxHeight: '78vh',
                        marginTop: '5px'
                    } as CSSProperties,
                    ref: scrollbar_ref
                },
                {
                    default:() => packets.value.map(p =>
                    {
                        return doc_status(p);
                    })
                })
            
            ]);
        }
        

        const status_style = (packet: IPacket) =>
        {
            const copy_error = packet.copyStatus.some(s=>s.copyOk == false);
            const icon = () =>
            {
                if(packet.packetInfo?.error)
                    return error_ico;
                else return get_icon(packet);
            }
            const box_color = () =>
            {
                if(packet.packetInfo?.error || copy_error)
                    return '#f6848487';
                else return 'rgb(100, 165, 9)';
            }
            return {icon, box_color, copy_error}
        }



        const doc_status = (packet: IPacket) =>
        {
            const parse_date = new DateTime(packet.parseTime);
            const parse_time = parse_date.to_string(DateFormat.DotDate) + " " + parse_date.to_string(DateFormat.Time)
            const {icon, box_color} = status_style(packet);
            return h(StatusCard,
            {
                key: packet.id,
                avatar: icon(),
                task_color: packet.task.color,
                shadowbox_color: box_color(),
                files: packet.packetInfo?.files,
                onClick: () =>
                {
                    emitter.emit('openFileViewer', packet);
                },
                
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
                        padding: '5px'
                        //background: 'linear-gradient(0.25turn, #0000004a, 90%, '+ packet.task.color + ', #ebf8e100)',
                        //background: '#00000033',
                    } as CSSProperties,
                   
                },
                [
                    h('div',
                    {
                        style:
                        {
                            fontWeight: '500',
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
                                width: "inherit",
                                backgroundColor: packet.task.color,
                                padding: "2px",
                                background: "rgba(27, 126, 110, 0.35)",
                                boxShadow: "0 8px 32px 0 rgba( 31, 38, 135, 0.37 )",
                                backdropFilter: "blur( 8px )",
                                "-webkit-backdrop-filter": "blur( 8px )",
                                borderRadius: "10px",
                                border: "1px solid",
                                borderColor: packet.task.color
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
                                    paddingRight: '15px',
                                } as CSSProperties
                            },
                            [
                                h(NTooltip, null,
                                {
                                    trigger:() =>
                                    h(NIcon, 
                                    {
                                        component: TimeOutline,
                                        color: packet.task.color,
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
                                    paddingRight: '15px',
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
                        copy_error_field(packet),
                        packet_error_field(packet),
                        requisites(packet),
                        packet.packetInfo?.requisites?.annotation ?
                        h('div', 
                        {
                            style:
                            {
                                //marginLeft:'17px',
                                fontWeight: '300',
                                marginTop: '5px',
                                backgroundColor: packet.task.color,
                                background: "rgba(86, 189, 172, 0.07)",
                                boxShadow: "0 8px 32px 0 rgba( 31, 38, 135, 0.37 )",
                                padding: '2px',
                            } as CSSProperties
                        },
                        [
                            h(NIcon, 
                            {
                                component: MenuOutline,
                                color: packet.task.color,
                                style:
                                {
                                    marginRight: '2px'
                                } as CSSProperties,
                            }),
                            packet.packetInfo?.requisites?.annotation
                        ]): h('span'),
                        
                    ])
                ])
            })
        }
        const copy_error_field = (packet: IPacket) => 
        {
            const { copy_error } = status_style(packet);
            if(copy_error)
            {
                return h('div',
                    {
                        style:
                        {
                            //marginLeft:'17px',
                            fontWeight: '300',
                            borderRadius: '5px',
                            marginTop: '5px',
                            fontSize: '14px',
                            backgroundColor: packet.task.color,
                            background: "rgba(116, 32, 32, 0.85)",
                            boxShadow: "0 2px 8px 0 rgba(169, 30, 30, 0.85)",
                            padding: '2px',
                        } as CSSProperties
                    },
                    [
                        h('div', {style:{fontWeight: '700'}}, "При копировании файлов произошла ошибка, вы можете пересканировать данный пакет.")
                    ].concat(packet.copyStatus.map(c=>
                    {
                        if(c.copyOk)
                        {
                            return h('div', `🟢 Пакет ${packet.name} успешно скопирован в ${c.copyPath}`)
                        }
                        else
                        {
                            return h('div', `❌ Ошибка копирования пакета ${packet.name} в ${c.copyPath}`)
                        }
                    }))
                )
            }
            else
            {
                return [];
            }
        }
        

        const right_icons_panel = (packet: IPacket) =>
        {
           
            return  h('div',
            {
                style:
                {
                    flexGrow: 1,
                    justifyContent: 'end',
                    display: 'flex',
                    gap: '5px'
                } as CSSProperties
            },
            [
               
                open_file_viewer_button(packet),
                report_icon(packet),
                (packet.packetInfo?.error || packet.copyStatus.some(s=>s.copyOk == false)) ? rescan_item_button(packet) : h('span'),
                del_button(packet),
            ])
        }

        const rescan_item_button = (packet: IPacket) =>
        {
            const disabled = ref(false);
            return h(NTooltip, 
                {
                   
                },
                {
                    trigger:() =>
                    h(NButton,
                    {
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
                })
        }
        const open_file_viewer_button = (packet: IPacket) =>
        {
            return  h(NTooltip,{placement: 'top'},
            {
                trigger:() =>
                h(NAvatar,
                {
                    size: 18,
                    src: pdf_ico,
                    class: 'hover-button',
                    style:
                    {
                        backgroundColor: 'transparent',
                        cursor: 'pointer',
                        alignSelf: 'center'
                    }   as CSSProperties,
                    onClick: () =>
                    {
                        emitter.emit('openFileViewer', packet);
                    },
                    
                }),
                default:() => "Просмотр файлов пакета",
            })
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

        //даже если включена опция copy_only packet_info всеравно не пустой:
        // deliveryTime: "2024-10-17T16:15:26"

        // files: [] (0)

        // packetDirectory: "two (копия) copy"

        // updateKey: ""

        // visible: true

        // wrongEncoding: false
        const get_date = (packet: IPacket) =>
        {
            const sign_date = packet.packetInfo?.requisites?.signDate ? new DateTime(packet.packetInfo.requisites?.signDate) : undefined;
            if(sign_date)
            {
                return `от ${sign_date.to_string(DateFormat.DotDate)}`
            }
            else return ""
        }
        const get_number = (packet: IPacket) =>
        {
            const number = packet.packetInfo?.requisites?.documentNumber;
            if(number)
            {
                return `№ ${number}`
            }
            else return ""
        }
        const get_mj_requisites = (packet: IPacket) =>
        {
            const mj_date = packet.packetInfo?.requisites?.mj?.date ? new DateTime(packet.packetInfo.requisites.mj?.date) : undefined;
            const mj_number = packet.packetInfo?.requisites?.mj?.number;
            if(mj_date && mj_number)
            {
                return `(регистрация: №${mj_number} от ${mj_date.to_string(DateFormat.DotDate)})`
            }
            else return ""
        }
        const requisites = (packet: IPacket) =>
        {
            let description : string|undefined;
            if(packet.packetInfo && !packet.packetInfo.error && packet.packetInfo.requisites)
            {
                const organization = get_organization(packet);
                description = organization + " " + get_date(packet) + " " + get_number(packet) + " " + get_mj_requisites(packet);
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
                            component: MailSharp,
                            color: packet.task.color,
                            style:
                            {
                                marginRight: '2px'
                            } as CSSProperties,
                        }),
                        default:() => "Реквизиты документа"
                    }),
                    description,
                    
                ])
            }
            else return []
        }
        const packet_error_field = (packet: IPacket) => 
        {
            if(packet.packetInfo?.error)
            {
                return h('div',
                {
                    style:
                    {
                        //marginLeft:'17px',
                        fontWeight: '300',
                        borderRadius: '5px',
                        marginTop: '5px',
                        fontSize: '14px',
                        backgroundColor: packet.task.color,
                        background: "rgba(116, 32, 32, 0.85)",
                        boxShadow: "0 2px 8px 0 rgba(169, 30, 30, 0.85)",
                        padding: '2px',
                    } as CSSProperties
                },
                [
                    h('div', {style:{fontWeight: '700'}}, "Ошибка при разборе транспортного пакета"),
                    h('div', `❌ ${packet.packetInfo?.error}`)
                ])
            }
            else return [];
        }
        
        return {list, complex}
    },
    render ()
    {
        return this.complex()
    }
})