import { RendererElement, RendererNode, VNode } from "vue";

export interface IDocument
{
    organization?: string,
    organizationUid: string,
    docType?: string,
    sourceMedoAddressee?: string
    docUid?: string
    number?: string,
    signDate?: string,
   
}
// для всех
// parseTime, name, organization, docType, number, signDate
export interface IPacket
{
    name: string,
    parseTime: string,
    document?: IDocument,
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
    clean_types: string[],
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
                clean_types: source.clean_types,
                generate_exclude_file: source.generate_exclude_file,
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