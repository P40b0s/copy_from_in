import { IPacket, Task } from "../../models/types"; 
import { Plugin } from "./abstract";

class Settings extends Plugin<'update' | 'get' | 'delete'>
{
    plugin = "plugin:settings|";
    public async save_task(types: Task): Promise<void|undefined|string>
    {
        return await this.save<Task, void>('update', types);
    }
    public async load_settings(): Promise<Task[]|undefined|string>
    {
        return await this.get<Task[]>('get');
    }
    public async delete_task(types: Task): Promise<void|undefined|string>
    {
        return await this.save<Task, void>('delete', types);
    }
}

class Service extends Plugin<'truncate_tasks_excepts' | 'clear_dirs' | 'ws_server_online' | 'rescan_packet'>
{
    plugin = "plugin:service|";
    public async truncate_tasks_excepts<R extends number>(): Promise<R|undefined|string>
    {
        return await this.get<R>('truncate_tasks_excepts');
    }
    public async clean_dirs<R extends number>(): Promise<R|undefined|string>
    {
        return await this.get<R>('clear_dirs');
    }
    public async ws_server_online<R extends boolean>(): Promise<R|undefined|string>
    {
        return await this.get<R>('ws_server_online');
    }
    public async rescan_packet<R extends IPacket>(packet: R): Promise<void|undefined|string>
    {
        return await this.save('rescan_packet', packet);
    }
}
const service = new Service();
const settings = new Settings();
export {settings, service}