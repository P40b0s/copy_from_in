import { RendererElement, RendererNode, VNode } from "vue";
import type { IPacketInfo } from './packet';
// export interface IPacketInfo
// {
//     // organization?: string,
//     // organizationUid: string,
//     // docType?: string,
//     // sourceMedoAddressee?: string
//     // docUid?: string
//     // number?: string,
//     // signDate?: string,
//     header_guid?: string,
//     packet_directory: string,
//     packet_type?: string,
//     ///Время создания локальной директории
//     ///(фактически когда пакет пришел к нам)
//     ///зависит от времени на сервере, тому что берет локальное время создания
//     delivery_time : string,
//     wrong_encoding: boolean,
//     error?: string,
//     files: string[],
//     requisites?: Requisites,
//     sender_info?: SenderInfo,
//     default_pdf?: string,
//     pdf_hash?: string,
//     acknowledgment?: Ack,
//     trace_message?: string,
//     visible: boolean,
   
// }
// для всех
// parseTime, name, organization, docType, number, signDate
export interface IPacket
{
    name: string,
    parseTime: string,
    packetInfo?: IPacketInfo,
    error?: string,
    task: Task,
    reportSended: boolean,
}

export type Task = 
{
    name: string,
    description: string,
    source_dir: string,
    target_dir: string,
    report_dir: string,
    timer: number,
    delete_after_copy: boolean,
    copy_modifier: CopyModifer,
    is_active: boolean,
    generate_exclude_file: boolean,
    color: string,
    sound: boolean,
    clean_types: string[],
    autocleaning: boolean,
    filters: Filter
}
export type CopyModifer = 'CopyAll' | 'CopyOnly' | 'CopyExcept';
export type Filter = 
{
    document_types: string[],
    document_uids: string[]
}

interface Clone<T>
{
    clone(source: T|undefined): T|undefined
}

class TaskClone implements Clone<Task>
{
    clone(source: Task|undefined): Task|undefined
    {
        if(source)
        {
            const f : Filter = 
            {
                document_types: source.filters.document_types,
                document_uids: source.filters.document_uids
            }
            const t : Task =
            {
                name: source.name,
                description: source.description,
                source_dir: source.source_dir,
                target_dir: source.target_dir,
                report_dir: source.report_dir,
                timer: source.timer,
                delete_after_copy: source.delete_after_copy,
                copy_modifier: source.copy_modifier,
                is_active: source.is_active,
                color: source.color,
                sound: source.sound,
                clean_types: source.clean_types,
                generate_exclude_file: source.generate_exclude_file,
                autocleaning: source.autocleaning,
                filters: f
            } 
            return t;
        }
        else return undefined;
    }
}

export type VN = VNode<RendererNode, RendererElement, {
    [key: string]: any;
}>
export const taskClone = new TaskClone();