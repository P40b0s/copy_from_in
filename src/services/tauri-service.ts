import { event, invoke } from "@tauri-apps/api";
import { IPacket, Task } from "../models/types"; 
import { UnlistenFn, listen } from "@tauri-apps/api/event";
import { InvokeArgs } from "@tauri-apps/api/tauri";

function is_tauri() : boolean
{
    // @ts-ignore
    if (window.__TAURI_IPC__)
        return true;
    else
        return false;
}

export class TauriEvents
{
    static async new_document_event(func: (arg: event.Event<IPacket>) => void) : Promise<UnlistenFn|undefined>
    {
        if(is_tauri())
            return await listen<IPacket>('new_packet_found', (event) => 
            {
                console.log(`Эвент new_packet_found обновлен ${event.windowLabel}, payload: ${event.payload.parseTime}`);
                func(event);
            });
        else
        {
            console.error("таури не заинжекчен!")
        }
    }
    static async settings_updated(func: (arg: event.Event<Task>) => void) : Promise<UnlistenFn|undefined>
    {
        if(is_tauri())
            return await listen<Task>('settings/tasks/update', (event) => 
            {
                console.log(`Получен эвент что настройки сервера были обновлены ${event.windowLabel}`);
                func(event);
            });
        else
        {
            console.error("таури не заинжекчен!")
        }
    }
}

abstract class AbstractEvents<E extends string>
{
    /** Запуск команды таури, если таури не заинжекчен то undefined если тип string то значит пришла ошибка*/
    async subscribe<T>(event_name: E, func: (arg: event.Event<T>) => void) : Promise<UnlistenFn|undefined>
    {
        if(is_tauri())
            return await listen<T>(event_name, (event) => 
            {
                console.log(`Получен эвент ${event.windowLabel}`);
                func(event);
            });
        else
        {
            console.error("таури не заинжекчен!")
        }
    }
    public async unsubscribe(event: Promise<UnlistenFn|undefined>)
    {
        event.then(f => 
        {
            if (f)
            f()
        });
    }
}
/**
 * задаем дженерик в виде литеральных типов, и создаем перечень эвентов
 */
class Events extends AbstractEvents<'packets_update' | 'error' | 'task_updated' | 'task_deleted'>
{
    public async packets_update(func: (arg: event.Event<IPacket>) => void): Promise<UnlistenFn|undefined>
    {
        return await this.subscribe('packets_update', func)
    }
    public async error(func: (arg: event.Event<string>) => void): Promise<UnlistenFn|undefined>
    {
        return await this.subscribe('error', func)
    }
    public async task_updated(func: (arg: event.Event<Task>) => void): Promise<UnlistenFn|undefined>
    {
        return await this.subscribe('task_updated', func)
    }
    public async task_deleted(func: (arg: event.Event<Task>) => void): Promise<UnlistenFn|undefined>
    {
        return await this.subscribe('task_deleted', func)
    }
}




abstract class Plugin<C extends string>
{
    protected abstract plugin: string;
    /** Запуск команды таури, если таури не заинжекчен то undefined если тип string то значит пришла ошибка*/
    async save<I, O>(cmd: C, saved_obj: I) : Promise<O|undefined|string>
    {
        if (is_tauri())
        {
            try
            {
                const data = await invoke<O>(this.plugin + cmd, {payload: saved_obj});
                return data;
            }
            catch(e: unknown)
            {
                console.error(e);
                return new Promise<string>((resolve) => 
                {
                    resolve(String(e));
                });
            }
        }
        else
        {
            console.error("Tauri не заинжекчен, невозможно сохранить ", saved_obj);
            return new Promise<undefined>((resolve) => 
            {
                resolve(undefined);
            });
        }
    }
    /** Запуск команды таури, если таури не заинжекчен то undefined если тип string то значит пришла ошибка*/
    async get<T>(cmd: C, args?: InvokeArgs) : Promise<T|undefined|string>
    {
        if (is_tauri())
        {
            try
            {
                const data = await invoke<T>(this.plugin + cmd, args);
                return data;
            }
            catch(e: unknown)
            {
                console.error(e);
                return new Promise<string>((resolve) => 
                {
                    resolve(String(e));
                });
            }
        }
        else
        {
            console.error("Tauri не заинжекчен, невозможно выполнить команду");
            return new Promise<undefined>((resolve) => 
            {
                resolve(undefined);
            });
        }
    }
    functionGenerator = <T extends string, U = { [K in T]?: string }>(keys: T[]): U => 
    {
        return keys.reduce((oldType: any, type) => ({ ...oldType, [type]: type }), {})
    }
}

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
const events = new Events();
export {settings, service, events}