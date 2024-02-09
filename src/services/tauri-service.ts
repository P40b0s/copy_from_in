import { event, invoke } from "@tauri-apps/api";
import { IPacket, Task } from "../types/types"; 
import { listen } from "@tauri-apps/api/event";
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
    static async new_document_event(func: (arg: event.Event<IPacket>) => void)
    {
        if(is_tauri())
            await listen<IPacket>('new_packet_found', (event) => 
            {
                console.log(`Эвент new_packet_found обновлен ${event.windowLabel}, payload: ${event.payload.document?.parseTime}`);
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
    abstract plugin: string;
    abstract cmd_names: string[];
    /** Запуск команды таури, если таури не заинжекчен то undefined если тип unknown то значит пришла ошибка*/
    async save<I, O>(cmd: string, saved_obj: I) : Promise<O|undefined|unknown>
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
                return new Promise<unknown>((resolve) => 
                {
                    resolve(e);
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
    /** Запуск команды таури, если таури не заинжекчен то undefined если тип unknown то значит пришла ошибка*/
    async get<T>(cmd: string, args?: InvokeArgs) : Promise<T|undefined|unknown>
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
                return new Promise<unknown>((resolve) => 
                {
                    resolve(e);
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
    cmd_names = ['update', 'get'];
    public async save_settings(types: Task[]): Promise<void|undefined|unknown>
    {
        return await this.save<Task[], void>(this.plugin + this.cmd_names[0], types);
    }
    public async load_settings(): Promise<void|undefined|unknown>
    {
        return await this.get<Task[]>(this.plugin + this.cmd_names[1]);
    }
}

const settings = new Settings();
export {settings}