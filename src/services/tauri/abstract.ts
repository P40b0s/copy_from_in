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

export class Unlistener
{
    constructor(unlisten: event.UnlistenFn|undefined)
    {
        this.unlisten = unlisten;
    }
    unlisten: event.UnlistenFn|undefined
    unsubscribe()
    {
        if (this.unlisten)
            this.unlisten();
    }
    
}
export abstract class AbstractEvents<E extends string>
{
    /** Запуск команды таури, если таури не заинжекчен то undefined если тип string то значит пришла ошибка*/
    async subscribe<T>(event_name: E, func: (arg: event.Event<T>) => void) : Promise<Unlistener>
    {
        if(is_tauri())
        {
            return new Unlistener(await listen<T>(event_name, (event) => 
            {
                console.log(`Получен эвент ${event.event}`);
                func(event);
            }));
        }
        else
        {
            console.error("таури не заинжекчен!")
            return new Unlistener(undefined);
        }
    }
}

export abstract class Plugin<C extends string>
{
    protected abstract plugin: string;
    /** Запуск команды таури, если таури не заинжекчен то undefined если тип string то значит пришла ошибка*/
    async post<P, T>(cmd: C, saved_obj: P) : Promise<Result<T>>
    {
        if (is_tauri())
        {
            try
            {
                const data = await invoke<T>(this.plugin + cmd, {payload: saved_obj});
                return new Result<T>(data);
            }
            catch(e: unknown)
            {
                console.error(e);
                return new Promise<Result<T>>((resolve) => 
                {
                    resolve(new Result<T>(undefined, String(e)));
                });
            }
        }
        else
        {
            console.error("Tauri не заинжекчен, невозможно выполнить команду ", saved_obj);
            return new Promise<Result<T>>((resolve) => 
            {
                resolve(new Result<T>(undefined, "Tauri не заинжекчен, невозможно выполнить команду"));
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
                return new Result<T>(data)
            }
            catch(e: unknown)
            {
                console.error(e);
                return new Promise<Result<T>>((resolve) => 
                {
                    resolve(new Result<T>(undefined, String(e)));
                });
            }
        }
        else
        {
            console.error("Tauri не заинжекчен, невозможно выполнить команду");
            return new Promise<Result<T>>((resolve) => 
            {
                resolve(new Result<T>(undefined, "Tauri не заинжекчен, невозможно выполнить команду"));
            });
        }
    }
    /** Выполнение команды с нагрузкой*/
    async get_with_payload<T, P>(cmd: C, payload: P) : Promise<Result<T>>
    {
        if (is_tauri())
        {
            try
            {
                const data = await invoke<T>(this.plugin + cmd, {payload: payload});
                return new Result<T>(data)
            }
            catch(e: unknown)
            {
                console.error(e);
                return new Promise<Result<T>>((resolve) => resolve(new Result<T>(undefined, String(e))))
            }
        }
        else
        {
        console.error("Tauri не заинжекчен, невозможно выполнить команду");
        return new Promise<Result<T>>((resolve) => 
        {
            resolve(new Result<T>(undefined, "Tauri не заинжекчен, невозможно выполнить команду"));
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
    constructor(val?: T, err?: string)
    {
        this.value = val;
        this.error = err;
    }

    is_ok(): boolean
    {
        return this.value != undefined ? true : false
    }
    is_err(): boolean
    {
        return this.error != undefined ? true : false
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