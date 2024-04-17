import { event, invoke } from "@tauri-apps/api";
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


export abstract class AbstractEvents<E extends string>
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

export abstract class Plugin<C extends string>
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