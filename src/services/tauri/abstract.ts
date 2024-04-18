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
    async post<I, T>(cmd: C, saved_obj: I) : Promise<Result<T>>
    {
        if (is_tauri())
        {
            try
            {
                const data = await invoke<T>(this.plugin + cmd, {payload: saved_obj});
                return {value: data} as Result<T>;
            }
            catch(e: unknown)
            {
                console.error(e);
                return new Promise<Result<T>>((resolve) => 
                {
                    resolve({error: String(e)} as Result<T>);
                });
            }
        }
        else
        {
            console.error("Tauri не заинжекчен, невозможно выполнить команду ", saved_obj);
            return new Promise<Result<T>>((resolve) => 
            {
                resolve({error: "Tauri не заинжекчен, невозможно выполнить команду"} as Result<T>);
            });
        }
    }
    /** Запуск команды таури, если таури не заинжекчен то undefined если тип string то значит пришла ошибка*/
    async get<T>(cmd: C, args?: InvokeArgs) : Promise<Result<T>>
    {
        if (is_tauri())
        {
            try
            {
                const data = await invoke<T>(this.plugin + cmd, args);
                return {value: data} as Result<T>;
            }
            catch(e: unknown)
            {
                console.error(e);
                return new Promise<Result<T>>((resolve) => 
                {
                    resolve({error: String(e)} as Result<T>);
                });
            }
        }
        else
        {
            console.error("Tauri не заинжекчен, невозможно выполнить команду");
            return new Promise<Result<T>>((resolve) => 
            {
                resolve({error: "Tauri не заинжекчен, невозможно выполнить команду"} as Result<T>);
            });
        }
    }
    functionGenerator = <T extends string, U = { [K in T]?: string }>(keys: T[]): U => 
    {
        return keys.reduce((oldType: any, type) => ({ ...oldType, [type]: type }), {})
    }
}

export class Result<T>
{
    value?: T
    error?: string;

    is_ok(): boolean
    {
        return this.value ? true : false
    }
    is_err(): boolean
    {
        return this.error ? true : false
    }
    get_value(): T
    {
        return this.value as T
    }
    get_error(): string
    {
        return this.error as string
    }
}