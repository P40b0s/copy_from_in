import { Senders } from '../../models/senders';
import { type IPacket, type Task, type File, type FilesRequest, type FileRequest } from '../../models/types'; 
import { Plugin, Result } from "./abstract";


class Settings extends Plugin<'update' | 'get' | 'delete'>
{
    plugin = "plugin:settings|";
    public async save_task(types: Task): Promise<Result<void>>
    {
        return await this.post<Task, void>('update', types);
    }
    public async load_settings(): Promise<Result<Task[]>>
    {
        return await this.get<Task[]>('get');
    }
    public async delete_task(types: Task): Promise<Result<void>>
    {
        return await this.post<Task, void>('delete', types);
    }
}

class Service extends Plugin<'truncate_tasks_excepts' | 'clear_dirs' | 'ws_server_online' | 'rescan_packet' | 'delete_packet'>
{
    plugin = "plugin:service|";
    public async truncate_tasks_excepts<R extends number>(): Promise<Result<R>>
    {
        return await this.get<R>('truncate_tasks_excepts');
    }
    public async clean_dirs<R extends void>(): Promise<Result<R>>
    {
        return await this.get<R>('clear_dirs');
    }
    public async ws_server_online<R extends boolean>(): Promise<Result<R>>
    {
        return await this.get<R>('ws_server_online');
    }
    public async rescan_packet<R extends IPacket>(packet: R): Promise<Result<void>>
    {
        return await this.post('rescan_packet', packet);
    }
    public async delete_packet<R extends IPacket>(packet: R): Promise<Result<void>>
    {
        return await this.post('delete_packet', packet);
    }
}

class Packets extends Plugin<
   'get_packets_list'
 | 'get_count' 
 | 'search_packets' 
 | 'get_files_list' 
 | 'get_pdf_pages_count' 
 | 'get_pdf_page' 
 | 'get_file_body'
 | 'get_senders'>
{
    plugin = "plugin:packets|";
    public async get_packets_list(limit: number, offset: number): Promise<Result<IPacket[]>>
    {
        return await this.get<IPacket[]>('get_packets_list', {pagination: {row: limit, offset: offset}});
    }
    public async search_packets(search_string: string): Promise<Result<IPacket[]>>
    {
        return await this.get<IPacket[]>('search_packets', {payload: search_string});
    }
    public async get_count(): Promise<Result<number>>
    {
        return await this.get<number>('get_count');
    }
    public async get_files_list(fr: FilesRequest): Promise<Result<File[]>>
    {
        return await this.get<File[]>('get_files_list', {filesRequest: {dir_name: fr.dir_name, task_name: fr.task_name}});
    }
    public async get_pdf_pages_count(fr: FileRequest): Promise<Result<number>>
    {
        return await this.get<number>('get_pdf_pages_count', {fileRequest: { file: { file_name: fr.file.file_name, file_type: fr.file.file_type, path: fr.file.path }} as FileRequest});
    }
    public async get_pdf_page<T extends string>(fr: FileRequest): Promise<Result<T>>
    {
        return await this.get<T>('get_pdf_page', {fileRequest: { file: { file_name: fr.file.file_name, file_type: fr.file.file_type, path: fr.file.path }, page_number: fr.page_number} as FileRequest});
    }
    public async get_file_body<T extends string>(fr: FileRequest): Promise<Result<T>>
    {
        return await this.get<T>('get_file_body', {fileRequest: { file: { file_name: fr.file.file_name, file_type: fr.file.file_type, path: fr.file.path }} as FileRequest});
    }
    public async get_senders<T extends Senders[]>(): Promise<Result<T>>
    {
        return await this.get<T>('get_senders');
    }
    
    
}
const commands_service = new Service();
const commands_settings = new Settings();
const commands_packets = new Packets();
export {commands_settings, commands_service, commands_packets}