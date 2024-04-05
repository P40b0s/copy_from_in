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

abstract class Plugin  
{
    protected abstract plugin: string;
    protected abstract cmd_names: string[];
    /** Запуск команды таури, если таури не заинжекчен то undefined если тип string то значит пришла ошибка*/
    async save<I, O>(cmd: string, saved_obj: I) : Promise<O|undefined|string>
    {
        if (is_tauri())
        {
            try
            {
                const data = await invoke<O>(cmd, {payload: saved_obj});
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
    async get<T>(cmd: string, args?: InvokeArgs) : Promise<T|undefined|string>
    {
        if (is_tauri())
        {
            try
            {
                const data = await invoke<T>(cmd, args);
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

class Settings extends Plugin
{
    plugin = "plugin:settings|";
    cmd_names = ['update', 'get', 'delete'];
    public async save_task(types: Task): Promise<void|undefined|string>
    {
        return await this.save<Task, void>(this.plugin + this.cmd_names[0], types);
    }
    public async load_settings(): Promise<Task[]|undefined|string>
    {
        return await this.get<Task[]>(this.plugin + this.cmd_names[1]);
    }
    public async delete_task(types: Task): Promise<void|undefined|string>
    {
        return await this.save<Task, void>(this.plugin + this.cmd_names[2], types);
    }
}

class Service extends Plugin
{
    plugin = "plugin:service|";
    cmd_names = ['truncate_tasks_excepts', 'clear_dirs'];
    public async truncate_tasks_excepts<R extends number>(): Promise<R|undefined|string>
    {
        return await this.get<R>(this.plugin + this.cmd_names[0]);
    }
    public async clean_dirs<R extends number>(): Promise<R|undefined|string>
    {
        return await this.get<R>(this.plugin + this.cmd_names[1]);
    }
}
const service = new Service();
const settings = new Settings(); 
export {settings, service}