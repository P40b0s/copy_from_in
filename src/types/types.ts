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
    copy_modifier: 'copy_all' | 'copy_only' | 'copy_except',
    is_active: boolean,
    filters: Filter
}
export type Filter = 
{
    document_types: string[],
    document_uids: string[]
}