import { IPacket, Task } from '../../models/types'; 
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

class Packets extends Plugin<'get_packets_list' | 'get_count'>
{
    plugin = "plugin:packets|";
    public async get_packets_list(limit: number, offset: number): Promise<Result<IPacket[]>>
    {
        return await this.get<IPacket[]>('get_packets_list', {pagination: {row: limit, offset: offset}});
    }
    public async get_count(): Promise<Result<number>>
    {
        return await this.get<number>('get_count');
    }
}
const commands_service = new Service();
const commands_settings = new Settings();
const commands_packets = new Packets();
export {commands_settings, commands_service, commands_packets}