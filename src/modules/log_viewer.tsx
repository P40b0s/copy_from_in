import 
{
    h,
    defineComponent,
    defineAsyncComponent,
    CSSProperties,
  } from 'vue'

import { NIcon, NSpin, NVirtualList} from 'naive-ui';
import { DateFormat, DateTime} from '../services/date.ts';
import { app_state_store } from '../store/index.ts';
import { StatusCard } from './status_card.tsx';
import { envelope_ico, error_ico } from '../services/svg.ts';
import { IPacket } from '../models/types.ts';
import { FolderOpen, TimeOutline } from '@vicons/ionicons5';
export const LogViewerAsync = defineAsyncComponent({
    loader: () => import ('./log_viewer.tsx'),
    loadingComponent: h(NSpin)
})

export const LogViewer =  defineComponent({
    setup () 
    {
        //тестовые данные
        // const test_packet = () =>
        // {
        //     const p : IPacket = {
        //         document:
        //         {
        //         name: "название_директории",
        //         organization: "Совет Федерации Федерального Собрания Российской Федерации",
        //         docType: "Постановление Совета Федерации Федерального Собрания Российской Федерации",
        //         number: "299-СФ",
        //         signDate: "2023-06-21",
        //         parseTime: "2024-03-20T16:52:51"
        //         }
        //     }
        //     return p;
        // }
        // const test_error_packet = () =>
        // {
        //     const p : IPacket = {
        //         document:
        //         {
        //             name: "ошибочное название директории",
        //             parseTime: "2024-12-24T00:00:00"
        //         },
        //         error: "Ошибка распознавания пакета б!"
        //     }
        //     return p;
        // }
        // const test_error_packet2 = () =>
        // {
        //     const p : IPacket = {
        //         error: "Ошибка распознавания пакета!"
        //     }
        //     return p;
        // }
        // for (let index = 0; index < 100; index++) {
        //     app_state_store.add_packet(test_packet());
        //     app_state_store.add_packet(test_error_packet());
        //     app_state_store.add_packet(test_error_packet2());
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
            let description = "";
            let parse_time = new DateTime().to_string(DateFormat.DotDate) + " " + new DateTime().to_string(DateFormat.Time)
            if(packet.document)
            {
                const parse_date = new DateTime(packet.document.parseTime);
                parse_time = parse_date.to_string(DateFormat.DotDate) + " " + parse_date.to_string(DateFormat.Time)
                const sign_date = packet.document.signDate ? new DateTime(packet.document.signDate) : undefined;
                description = (packet.document.organization ?? "") + " " + (sign_date?.to_string(DateFormat.DotDate) ?? "") + " " + (packet.document.number ?? "")
            }
            return h(StatusCard,
                {
                    key: packet.document?.parseTime ?? parse_time,
                    avatar: packet.error ? error_ico : envelope_ico,
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
                            textAlign: 'left'
                        } as CSSProperties
                    },
                    [
                        h('div',
                        {
                            style:
                            {
                                fontWeight: '700',
                            } as CSSProperties,
                            class: packet.error ? 'standart-red' : 'standart-green'

                        },
                        [
                            h('div',
                            {
                                style:
                                {
                                    display: 'flex',
                                    flexDirection: 'row',
                                    justifyItems: 'center',
                                    alignItems: 'center'
                                } as CSSProperties,
                            },
                            [
                                h(NIcon, 
                                {
                                    component: TimeOutline,
                                    color: 'rgb(100, 165, 9)',
                                    style:
                                    {
                                        marginRight: '5px'
                                    } as CSSProperties,
                                }),
                                parse_time,
                                packet.document ? h(NIcon, 
                                {
                                    component: FolderOpen,
                                    color: 'rgb(241, 229, 95)',
                                    style:
                                    {
                                        marginLeft: '5px',
                                        marginRight: '5px'
                                    } as CSSProperties,
                                }) : [],
                                packet.document?.name,
                            ]),
                            packet.document ? h('div', description) : [],
                            packet.error ? h('div', packet.error) : [],
                        ])
                    ])
                })
        }

        const virtual_list = () =>
        {
            return h(NVirtualList,
                {
                    style:
                    {
                        maxHeight: "600px",
                        minHeight: "300px"
                    } as CSSProperties,
                    itemSize: 60,
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