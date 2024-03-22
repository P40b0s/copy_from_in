export interface IDocument
{
    organization?: string,
    docType?: string,
    number?: string,
    signDate?: string,
    name: string,
    parseTime: string
}
// для всех
// parseTime, name, organization, docType, number, signDate
export interface IPacket
{
    document?: IDocument,
    error?: string
}

export type Task = 
{
    name: string,
    source_dir: string,
    target_dir: string,
    timer: number,
    delete_after_copy: boolean,
    copy_modifier: 'CopyAll' | 'CopyOnly' | 'CopyExcept',
    is_active: boolean,
    filters: Filter
}

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
                source_dir: source.source_dir,
                target_dir: source.target_dir,
                timer: source.timer,
                delete_after_copy: source.delete_after_copy,
                copy_modifier: source.copy_modifier,
                is_active: source.is_active,
                filters: f
            } 
            return t;
        }
        else return undefined;
    }
}
export const taskClone = new TaskClone();