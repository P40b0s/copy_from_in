import { NIcon, SelectOption } from "naive-ui";
import { VNodeChild, h } from "vue";
import { Archive, AttachOutline, CodeSlashSharp, AtOutline, Help, Text } from '@vicons/ionicons5';
import emitter from "../../services/emit";
import { type SelectBaseOption, type SelectGroupOption } from "naive-ui/es/select/src/interface";
import { type IPacket, FilesRequest } from '../../models/types';
import { commands_packets } from "../../services/tauri/commands";

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


export type SelectedValue =                                    
{
  label: string,   
  value: string,
  ext: string,
  path: string
} & (SelectOption | SelectGroupOption);

/**Опшены для селектора */
export const options = async (packet: IPacket): Promise<SelectedValue[]>  =>
{
    let fs = {
        dir_name: packet.name,
        task_name: packet.task.name
    } as FilesRequest
    const files = await commands_packets.get_files_list(fs);
    if(files.error)
        return [];
    else
    {
        const f = files.get_value();
        return f.map(m =>
        {
            return {
                label: m.file_name,
                value: m.path,
                ext: m.file_type,
                path: m.path
            } as SelectedValue
        })
    }
}

export const on_update_val = (val: SelectedValue, option: SelectBaseOption|null) =>
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
    console.warn(option);
    
    const selected_path = val.toString();
    //emitter.emit('fileRequest', selected_path)
    console.log(val);
}

export const get_dir_type = (packet: IPacket) =>
{
    
    if(!packet?.packetInfo?.defaultPdf)
        return 'error'
    else
        return 'success'
}

export const fileSelectorLabel = (option: SelectedValue , selected: boolean): VNodeChild => 
{
    let icon = Help;
    const standart_color = "#b3ffba";
    let color = "#c23838"
    let description = "Выберите файл для просмотра";
    option.disabled = true;
    console.log(option)
    if (option.value.indexOf(".ltr") >=0)
    {
        icon = AtOutline
        color = standart_color;
        description = "Сопроводительный файл к транспортному пакету"
        option.disabled = false;
    }
    else if (option.value.indexOf(".rc") >=0)
    {
        icon = CodeSlashSharp
        color = standart_color;
        description = "Файл с реквизитами документа (загружен с АРМ)"
        option.disabled = false;
    }
    else if (option.value.indexOf(".xml") >=0)
    {
        icon = CodeSlashSharp;
        color = standart_color;
        description = "Файл с реквизитами документа, или параметрами вложения"
        option.disabled = false;
    }
    else if (option.value.indexOf(".pdf") >=0)
    {
        icon = AttachOutline;
        color = standart_color;
        description = "Документ в формате pdf"
        option.disabled = false;
    }
    else if (option.value.indexOf(".txt") >=0)
    {
        icon = Text;
        color = standart_color;
        description = "Тестовый файл с аннотацией к документу или текстом документа"
        option.disabled = false;
    }
    else if (option.value.indexOf(".zip") >=0)
    {
        icon = Archive;
        color = standart_color;
        description = "Вложение транспортного пакета"
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
            description),
            h('span',
            {
                style:
                {
                    fontSize: '10px',
                    color: standart_color
                }
            },
            option.path)
        ])
    ]);
    return onlyIcon
}