import { NAvatar, NDivider, NIcon, SelectOption } from "naive-ui";
import { CSSProperties, VNodeChild, h } from "vue";
import { Archive, AttachOutline,Image, CodeSlashSharp, AtOutline, Help, Text } from '@vicons/ionicons5';
import emitter from "../../../services/emit";
import { type SelectBaseOption, type SelectGroupOption } from "naive-ui/es/select/src/interface";
import { type IPacket, FilesRequest } from '../../../models/types';
import { commands_packets } from "../../../services/tauri/commands";
import { archive_ico, certificate_ico, docx_ico, envelope_ico, file_ico, image_ico, journal_ico, pdf_ico, xml_ico } from "../../../services/svg";
import { supported_files } from "../../../models/file_types";

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
        }).sort((a, b) => supported_files.sorting_order(a.ext) - supported_files.sorting_order(b.ext))
    }
}





export const fileSelectorLabel = (option: SelectedValue , selected: boolean): VNodeChild => 
{
    let file_type = supported_files.get_type_by_filename(option.value);
    if(file_type == undefined)
        file_type = supported_files.not_supported;
    option.disabled = file_type.disabled;
    const onlyIcon = 
    h('div', 
    selected ? {style: selectedValueStyle} : {style: valueStyle}, 
    [
        h(NAvatar,
        {
            style: 
            {
                verticalAlign: '-0.15em',
                marginRight: '5px', 
                minWidth:'10px'
            },
            color: 'transparent',
            src: file_type.icon,
            size: 30

        }),
        h('div',
        {
            style: 
            {
                display: "flex",
                flexDirection: "column",
                justifyItems: 'center',
                flexWrap: 'wrap',
                maxWidth:'480px'
            } as CSSProperties
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
                    fontSize: '14px',
                    color: file_type.color
                }
            },
            file_type.description),
            h('span',
            {
                style:
                {
                    flexWrap: 'wrap',
                    fontSize: '10px',
                    textWrap: 'wrap',
                    color: supported_files.path_color
                } as CSSProperties
            },
            option.path)
        ])
    ]);
    return onlyIcon
}