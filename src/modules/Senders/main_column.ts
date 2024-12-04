import { NAlert, NAvatar, NGradientText, NIcon, NTag, NButton, NTooltip, NHighlight } from 'naive-ui';
import { RendererElement, RendererNode, VNode, h, reactive, ref, watchEffect } from 'vue';
import { ThumbsUpOutline, SettingsOutline, List} from '@vicons/ionicons5';
//import NoPhoto from  '../../assets/rastr/no-ico.png';
//import ContactsIcon from  '../../assets/svg/contacts.svg';
//import EditIcon from  '../../assets/svg/edit.svg';
import emitter, { Emitter, type Events } from '../../services/emit';
import OrganIcon from './OrganIcon.tsx';
import { type TableColumn } from 'naive-ui/es/data-table/src/interface';
import { ContactInfo, Senders } from '../../models/senders.ts';
import { edit_ico, envelope_ico, info_ico } from '../../services/svg.ts';
import { Callback } from '../../models/types.ts';
interface FilterOptions
{
  label?: string;
  value?: string;
}
const icons_width = () => '30px';
export const organ_column = (s: Callback<Senders>, c: Callback<Senders>) =>
{
        // const expand = 
    // reactive({
    //     type: 'expand',
    //     expandable: (rowData:Senders) => rowData.icon != null,
    //     renderExpand: (rowData:Senders) => 
    //     {
    //       return `АЙДИ ${rowData.id}`
    //     }
    // });
    
    const organColumn : TableColumn<Senders> =
    reactive({
    title: 'Отправители',
    key: 'organization',
    className: 'senders',
    sortOrder: 'ascend',
    sorter (rowA:Senders, rowB:Senders) 
    {
        return 0 - (rowA.organization > rowB.organization ? -1 : 1);
    },
    filterOptionValue: "",
    filter (value, row) 
    {
        if(!value)
            return true;
        if(value.toString().length == 0)
            return true;
        if(row.medo_addresse)
        {
            const addr_index = row.medo_addresse?.toLowerCase().indexOf(value.toString()) as number;
            //console.log(addr_index);
            if(addr_index >= 0)
                return true;
        }
        for (let index = 0; index < row.contact_info.length; index++) 
        {
            const element = row.contact_info[index];
            const organization = element.organization?.toLowerCase().indexOf(value.toString()) as number;
            const person = element.person?.toLowerCase().indexOf(value.toString()) as number;
            const post = element.post?.toLowerCase().indexOf(value.toString()) as number;
            if(organization >=0 || person >= 0 || post >= 0)
                return true;
        }
        const org_index = row.organization.toLowerCase().indexOf(value.toString());
        if(org_index >= 0)
            return true;
        return false; 
    },
    render (row: Senders) 
    {
        return h('div', 
        {
            style: 
                {
                    display: 'flex',
                    justifyItems: 'center',
                    alignItems: 'center',
                }
        },
        [
            functions_panel(row),
            h(OrganIcon, {icon: row.icon}),
            h('div',
            {
            style: 
            {
                display: 'flex',
                flexDirection: 'column',
                //minHeight: '50px',
                alignItems: 'left',
                marginLeft: '6px',
                marginRight: '6px',
                width: '100%'
            }
            },
            [
                h(
                    NTag,
                    {
                    style: 
                    {
                    },
                    type: 'info',
                    bordered: false
                    },
                    {
                    default: () =>  [
                    h(NGradientText,
                    {
                        style:
                        {
                        fontSize: '14px',
                        fontWeight: '600'
                        },
                        type: "info",
                    },
                    {
                        default: () => row.organization
                    }),
                    ]
                    },
                ),
                h(
                    NTag,
                    {
                    style: 
                    {
                    },
                    type: 'info',
                    bordered: false
                    },
                    {
                    default: () =>
                    [
                        h(NGradientText,
                        {
                            style:
                            {
                                fontSize: '14px',
                                fontWeight: '600'
                            },
                            type: "info",
                        },
                        {
                            default: () => row.medo_addresse ?? ""
                        }),
                    ]
                    }
                ),
                //notification_render(row),
            ]
            )
        ])
        }
    })

    // const notification_render = (row: Senders) =>
    // {
    //     return [
    //     h(
    //         NTooltip,
    //         {
    //             placement: 'top-start',
    //             alignSelf: 'left'
    //         },
    //         {
    //             trigger: () =>
    //             h('div',
    //             {
    //                 style:
    //                 {
    //                     display: 'flex',
    //                     flexDirection: 'column'
    //                 }
    //             },
    //             adresses_list_render(row)),
    //             default: () => "Адресаты для отправки уведомлений об опубликовании"
    //         },
    //     ),
    //     ]
    // }

    // const adresses_list_render = (row: Senders) : VNode<RendererNode, RendererElement, { [key: string]: any; }>[] =>
    // {
    //     const arr: VNode<RendererNode, RendererElement, { [key: string]: any; }>[] = [];
    //     for (let index = 0; index < row.notifications_sources_medo_addresses.length; index++) 
    //     {
    //         arr.push(h(
    //             NTag,
    //             {
    //                 style:
    //                 {
    //                     fontSize: '14px',
    //                     width: 'fit-content',
    //                 },
    //                 type: "success",
    //             },
    //             {
    //                 default: () => 
    //                 {
    //                     return row.notifications_sources_medo_addresses[index];
    //                 },
    //                 icon: () =>
    //                     h(NIcon,
    //                     {
    //                         component : List
    //                     }
    //                 ),
    //             },
    //         ));
    //     }
    //     return arr;      
    // }

    const functions_panel = (row: Senders) =>
    {
        return h('div',
        {
            style: 
            {
                marginRight: '7px',
                display: 'flex',
                flexDirection: 'column',
                //justifyContent: 'space-between',
                alignItems: 'center',
                alignSelf: 'normal',
                gap: '10px'

            },
        },
        [
            sender_card_editor_icon_render(row),
            contacts_editor_icon_render(row)
        ])
    }

    const sender_card_editor_icon_render = (row: Senders) =>
    {
        return [  
            h(
            NTooltip,
            {
                style:
                {
                    fontSize: '14px',
                },
                placement: 'right'
            },
            {
                default: () => 
                {
                    return "Редактировать карточку отправителя";
                },
                trigger: () =>
                h('img',
                {
                    style:
                    {
                        cursor: 'pointer',
                        width: icons_width()
                    },
                    onClick: (() => 
                    {
                        s(row);
                        //emitter.emit('startEditSender', row);
                    }),
                    src: envelope_ico
                }),
            })
        ]
    }

    const contacts_editor_icon_render = (row: Senders) =>
    {
        return [  
            h(
            NTooltip,
            {
                style:
                {
                    fontSize: '14px',
                },
                placement: 'right'
            },
            {
                default: () => 
                {
                    return "Редактировать контакты";
                },
                trigger: () =>
                h('img',
                {
                    style:
                    {
                        cursor: 'pointer',
                        width: icons_width()
                    },
                    onClick: (() => 
                    {
                        c(row);
                        //emitter.emit('startEditContacts', row);
                    }),
                    src: edit_ico
                }),
            })
        ]
    }

    function uniqByMap<T>(array: T[]): T[] 
    {
        const map = new Map();
        for (const item of array) 
        {
            //undefined нам не нужен
            if(item)
                map.set(item, item);
        }
        return Array.from(map.values());
    }

    return { organColumn}
}