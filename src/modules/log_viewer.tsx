import 
{
    h,
    defineComponent,
    defineAsyncComponent,
    CSSProperties,
  } from 'vue'

import { NSpin, NVirtualList} from 'naive-ui';
import { DateFormat, DateTime} from '../services/date.ts';
import { app_state_store } from '../store/index.ts';
import { StatusCard } from './status_card.tsx';
import { bell_ico, envelope_ico, error_ico } from '../services/svg.ts';
import { IPacket } from '../models/types.ts';
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
        // for (let index = 0; index < 100; index++) {
        //     app_state_store.add_packet(test_packet());
        //     app_state_store.add_packet(test_error_packet());
        // }
       
        //console.log(app_state_store.getState().current_log);
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
            let sb_color =  'rgba(22, 227, 84, 0.6)';
            let avatar = envelope_ico;
            let text_class = 'neon-blue';
            if(packet.document)
            {
                const parse_date = new DateTime(packet.document.parseTime);
                const sign_date = packet.document.signDate ? new DateTime(packet.document.signDate) : undefined;
                let description = (packet.document.organization ?? "") + " " + (sign_date?.to_string(DateFormat.DotDate) ?? "") + " " + (packet.document.number ?? "")
                if(packet.error)
                {
                    description = packet.error;
                    sb_color = '#f6848487';
                    avatar = error_ico;
                    text_class = 'neon-red'
                }
                return h(StatusCard,
                {
                    key: packet.document.parseTime,
                    avatar: avatar,
                    shadowbox_color: sb_color,
                    tooltip: packet.document.name
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
                            class: text_class

                        },
                        parse_date.to_string(DateFormat.DotDate) + " " + parse_date.to_string(DateFormat.Time) + " " + packet.document?.name),
                        h('div', description),
                    
                    ])
                })
            }
            else
            {
                if(packet.error)
                {
                    return h("span", "Получен неизвестный пакет! " + packet.error);
                }
                else
                {
                    return h("span", "Получен неизвестный пакет! " + packet.error);
                }
            }
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