import 
{
    h,
    defineComponent,
    defineAsyncComponent,
    inject,
    onMounted,
    CSSProperties,
    PropType,
    toRefs,
    toRef,
    ref,
    VNode,
    RendererNode,
    RendererElement,
    toRaw,
    watch
  } from 'vue'

import { NAvatar, NButton, NCard, NCheckbox, NConfigProvider, NDatePicker, NDynamicInput, NForm, NFormItem, NInput, NInputGroup, NInputNumber, NModal, NScrollbar, NSelect, NSpin, NTab, NTabPane, NTable, NTabs, NTooltip, SelectGroupOption, SelectOption } from 'naive-ui';
import { DateFormat, DateTime, dateToString, parseDate, parseDateObj, timeToString } from '../services/date.ts';
import { AddCircleOutline, Close, Home, RemoveOutline } from '@vicons/ionicons5';
import { match } from 'ts-pattern';
import { ruRU, dateRuRU } from 'naive-ui'
import { TauriEvents } from '../services/tauri-service.ts';
import { app_state_store } from '../store/index.ts';
import { StatusCard } from './status_card.tsx';
import { bell_ico } from '../services/svg.ts';
import { IDocument, IPacket } from '../models/types.ts';
export const LogViewerAsync = defineAsyncComponent({
    loader: () => import ('./log_viewer.tsx'),
    loadingComponent: h(NSpin)
})

export const LogViewer =  defineComponent({
    setup () 
    {
        const test_packet = () =>
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
                }
            }
            return p;
        }
        app_state_store.add_packet(test_packet());
        // const log = ref<string[]>(["123 34 2234 234"]);
        //  onMounted(async ()=> 
        //     await TauriEvents.new_document_event((doc) => 
        //     {
        //         console.log("EVENT!");
        //         const pl = doc.payload
        //         if(pl.error)
        //         {
        //             log.value.push(pl.error);
        //         }
        //         if(pl.document)
        //         {
        //             log.value.push(pl.document.name);
        //         }
        //     })
        // );
       
        console.log(app_state_store.getState().current_log);
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
                app_state_store.getState().current_log.map(p=>
                {
                    if (p.document)
                        return doc_status(p.document);
                    else if (p.error)
                        return err_status(p.error);
                })
            );
        }


        const doc_status = (doc: IDocument) =>
        {
            const parse_date = new DateTime(doc.parseTime);
            return h(StatusCard,
                {
                    key: doc.parseTime,
                    avatar: bell_ico,
                    shadowbox_color: 'rgba(22, 227, 84, 0.6)',
                    tooltip: doc.name
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
                        h('div', parse_date.to_string(DateFormat.DotDate) + " " + parse_date.to_string(DateFormat.Time) + " " + doc.name),
                        h('div', (doc.organization ?? "") + " " + (doc.signDate ?? "") + " " + (doc.number ?? "")),
                       
                    ])
                })
        }

        const err_status = (err: string) =>
        {
            return h(StatusCard,
                {
                    key: err,
                    avatar: bell_ico,
                    shadowbox_color: 'rgba(22, 227, 84, 0.6)',
                    tooltip: err
                },
                {
                    default:() =>
                    h('div', 
                    {
                        style: {
                            width: '100%',
                        } as CSSProperties
                    },
                    h('div', err),
                    )
                })
        }

        return {list}
    },
    render ()
    {
        return this.list()
    }
})