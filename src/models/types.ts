import { RendererElement, RendererNode, VNode } from "vue";
import type { IPacketInfo } from './packet';

export interface IPacket
{
    id: string;
    //Директория пакета
    name: string,
    parseTime: string,
    packetInfo?: IPacketInfo,
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
    //Отображать ли результат обработки этого таска в списке пакетов
    visible: boolean,
    filters: Filter
}
export type CopyModifer = 'CopyAll' | 'CopyOnly' | 'CopyExcept';
export type Filter = 
{
    document_types: string[],
    document_uids: string[]
}

export interface Clone<T>
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
                visible: source.visible,
                filters: f
            } 
            return t;
        }
        else return undefined;
    }
}

export type File = 
{   
    file_name: string,
    file_type: string,
    path: string
}
/// Структура для запроса страницы файла или всего файла из API
export type FileRequest = 
{
    file: File,
    page_number?: number,
}

export type FilesRequest = 
{
    task_name: string,
    dir_name: string
}  


export type VN = VNode<RendererNode, RendererElement, {
    [key: string]: any;
}>
export const taskClone = new TaskClone();

export type Callback<T> = (val: T) => void;