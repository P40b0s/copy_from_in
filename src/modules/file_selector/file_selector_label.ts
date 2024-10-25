import { NIcon, SelectOption } from "naive-ui";
import { VNodeChild, h } from "vue";
import { Archive, AttachOutline, CodeSlashSharp, AtOutline, Help, Text } from '@vicons/ionicons5';
import { PacketInfo } from "../../models/backend/document";
import emitter from "../../services/emit";
import { type SelectBaseOption, type SelectGroupOption } from "naive-ui/es/select/src/interface";

const valueStyle =
{
    display: "flex",
    flexDirection: "row",
    alignItems: 'center',
}
const selectedValueStyle =
{
    display: "flex",
    flexDirection: "row",
    alignItems: 'center',
    color: '#03e203',
}


type select_val =                                    
{
  label: string,   
  value: string
}

/**Опшены для селектора */
export const options = (packet: PacketInfo): select_val[]  =>
{
    const packet_dir = packet?.packetDirectory;
    if(packet_dir && packet?.files)
    {
        return packet.files.map(m=> 
            { 
                return {
                    value: packet_dir + "/" + m,
                    label: m
                } as select_val
            }
        );
    }
    else
    {
        return [];
    }
}

export const on_update_val = (val: string, option: SelectBaseOption|null) =>
{
    // const selected_path = val.toString();
    // if(val.toString().indexOf(".pdf") >=0)
    // {
    //     emitter.emit('pdfSelectedForView', selected_path)
    // }
    // else
    // {
    //     emitter.emit('fileRequest', selected_path)
    // }
    console.log(option?.value);
    const selected_path = val.toString();
    emitter.emit('fileRequest', selected_path)
    console.log(val);
}

export const get_dir_type = (packet: PacketInfo) =>
{
    
    if(!packet?.defaultPdf)
        return 'error'
    else
        return 'success'
}

export const fileSelectorLabel = (option: SelectOption | SelectGroupOption, selected: boolean): VNodeChild => 
{
    let icon = Help;
    const standart_color = "#b3ffba";
    let color = "#c23838"
    let description = "Формат не поддерживается просмотрщиком";
    option.disabled = true;
    if(option.value)
    { 
        const val = option.value as string;
        if (val.indexOf(".ltr") >=0)
        {
            icon = AtOutline
            color = standart_color;
            description = "Сопроводительный файл к транспортному пакету"
            option.disabled = false;
        }
        if (val.indexOf(".rc") >=0)
        {
            icon = CodeSlashSharp
            color = standart_color;
            description = "Файл с реквизитами документа (загружен с АРМ)"
            option.disabled = false;
        }
        if (val.indexOf(".xml") >=0)
        {
            icon = CodeSlashSharp;
            color = standart_color;
            description = "Файл с реквизитами документа, или параметрами вложения"
            option.disabled = false;
        }
        if (val.indexOf(".pdf") >=0)
        {
            icon = AttachOutline;
            color = standart_color;
            description = "Документ в формате pdf"
            option.disabled = false;
        }
        if (val.indexOf(".txt") >=0)
        {
            icon = Text;
            color = standart_color;
            description = "Тестовый файл с аннотацией к документу или текстом документа"
            option.disabled = false;
        }
        if (val.indexOf(".zip") >=0)
        {
            icon = Archive;
            color = standart_color;
            description = "Вложение транспортного пакета"
        }
    }

    const onlyIcon = 
    h('div', 
    selected ? {style: selectedValueStyle} : {style: valueStyle}, 
    [
        h(NIcon,
            {
                style: 
                {
                    verticalAlign: '-0.15em',
                    marginRight: '5px',
                    
                },
                color: color,
                size: 20

            },
            {
                default: () => h(icon)
            }
        ),
        h('div',
        {
            style: 
            {
                display: "flex",
                flexDirection: "column",
                justifyItems: 'center'
            }
        },
        [
            h('span',
            {
                style:
                {
                    fontWeight: '700',
                }
            },
            option.label as string),
            h('span',
            {
                style:
                {
                    fontSize: '10px',
                    color: standart_color
                }
            },
            description)
        ])
    ]);
    // if(tooltip != "")
    // return [
    //     h(NTooltip,
    //     {
    //         placement: 'left'
    //     },
    //     {
    //         trigger: () => onlyIcon,
    //         default: () => tooltip
    //     }),
    // ]
    // else
    // return  onlyIcon
    return onlyIcon
}