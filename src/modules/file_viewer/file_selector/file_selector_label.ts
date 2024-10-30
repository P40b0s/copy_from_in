import { NIcon, SelectOption } from "naive-ui";
import { VNodeChild, h } from "vue";
import { Archive, AttachOutline,Image, CodeSlashSharp, AtOutline, Help, Text } from '@vicons/ionicons5';
import emitter from "../../../services/emit";
import { type SelectBaseOption, type SelectGroupOption } from "naive-ui/es/select/src/interface";
import { type IPacket, FilesRequest } from '../../../models/types';
import { commands_packets } from "../../../services/tauri/commands";

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



export type FileType = 
{
    extension: string,
    highlighting_lang?: string,
    description: string,
    color: string,
    disabled: boolean,
    icon: typeof Help,
    type: FileTypeEnum

}
export enum FileTypeEnum
{
    /**Файл который можно прочитать как строку */
    File,
    /**Pdf файл */
    Pdf,
    /**Изображение */
    Image,
    /**Архив */
    Archive,
    /**Документ (doc, docx итд) */
    Document,
    /**Неподдерживаемый тип файла */
    NotSupported
}
class SupportedFiles
{
    standart_color = "#b3ffba";
    default = 
    {
        extension: "",
        description: "Выберите файл для просмотра",
        color: "#c23838",
        disabled: true,
        icon: Help,
        type: FileTypeEnum.NotSupported
    } as FileType
    not_supported = 
    {
        extension: "",
        description: "Просмотр данного файла не поддерживается",
        color: "#c23838",
        disabled: true,
        icon: Help,
        type: FileTypeEnum.NotSupported
    } as FileType
    files = 
    [
        {
            extension: "xml",
            highlighting_lang: "xml",
            description: "Файл с реквизитами документа, или параметрами вложения",
            color: this.standart_color,
            disabled: false,
            icon: CodeSlashSharp,
            type: FileTypeEnum.File
        },
        {
            extension: "txt",
            description: "Тестовый файл с аннотацией к документу или текстом документа",
            color: this.standart_color,
            disabled: false,
            icon: Text,
            type: FileTypeEnum.File
        },
        {
            extension: "ltr",
            description: "Сопроводительный файл к транспортному пакету",
            highlighting_lang: "ini",
            color: this.standart_color,
            disabled: false,
            icon: AtOutline,
            type: FileTypeEnum.File
        },
        {
            extension: "rc",
            description: "Файл с реквизитами документа (загружен с АРМ)",
            highlighting_lang: "xml",
            color: this.standart_color,
            disabled: false,
            icon: CodeSlashSharp,
            type: FileTypeEnum.File
        },
        {
            extension: "zip",
            description: "Zip архив с вложением транспортного пакета",
            color: this.standart_color,
            disabled: true,
            icon: Archive,
            type: FileTypeEnum.Archive
        },
        {
            extension: "pdf",
            description: "Документ в формате pdf",
            color: this.standart_color,
            disabled: false,
            icon: AttachOutline,
            type: FileTypeEnum.Pdf
        },
        {
            extension: "png",
            description: "Изображение в формате png",
            color: this.standart_color,
            disabled: false,
            icon: Image,
            type: FileTypeEnum.Image
        },
        {
            extension: "jpg",
            description: "Изображение в формате jpg",
            color: this.standart_color,
            disabled: false,
            icon: Image,
            type: FileTypeEnum.Image
        },
    ] as FileType[]
    /**
     * 
     * @param ext Расширение файла
     */
    get_type(ext: string| undefined): FileType| undefined
    {
        return this.files.find(f=>f.extension == ext);
    }
}

export const supported_files = new SupportedFiles();

export const fileSelectorLabel = (option: SelectedValue , selected: boolean): VNodeChild => 
{
    // let icon = Help;
    // const standart_color = "#b3ffba";
    // let color = "#c23838"
    // let description = "Выберите файл для просмотра";
    // option.disabled = true;
    // if (option.value.indexOf(".ltr") >=0)
    // {
    //     icon = AtOutline
    //     color = standart_color;
    //     description = "Сопроводительный файл к транспортному пакету"
    //     option.disabled = false;
    // }
    // else if (option.value.indexOf(".rc") >=0)
    // {
    //     icon = CodeSlashSharp
    //     color = standart_color;
    //     description = "Файл с реквизитами документа (загружен с АРМ)"
    //     option.disabled = false;
    // }
    // else if (option.value.indexOf(".xml") >=0)
    // {
    //     icon = CodeSlashSharp;
    //     color = standart_color;
    //     description = "Файл с реквизитами документа, или параметрами вложения"
    //     option.disabled = false;
    // }
    // else if (option.value.indexOf(".pdf") >=0)
    // {
    //     icon = AttachOutline;
    //     color = standart_color;
    //     description = "Документ в формате pdf"
    //     option.disabled = false;
    // }
    // else if (option.value.indexOf(".txt") >=0)
    // {
    //     icon = Text;
    //     color = standart_color;
    //     description = "Тестовый файл с аннотацией к документу или текстом документа"
    //     option.disabled = false;
    // }
    // else if (option.value.indexOf(".zip") >=0)
    // {
    //     icon = Archive;
    //     color = standart_color;
    //     description = "Вложение транспортного пакета"
    // }
    let ext = option.value.split(".")
    let file_type = supported_files.get_type(ext[ext.length -1])
    if(file_type == undefined)
        file_type = supported_files.default;
    option.disabled = file_type.disabled;
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
                color: file_type.color,
                size: 20

            },
            {
                default: () => h(file_type.icon)
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
                    color: supported_files.standart_color
                }
            },
            file_type.description),
            h('span',
            {
                style:
                {
                    fontSize: '10px',
                    color: supported_files.standart_color
                }
            },
            option.path)
        ])
    ]);
    return onlyIcon
}