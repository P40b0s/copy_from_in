import { InvokeArgs, invoke as inv,} from '@tauri-apps/api/tauri';
import { Dictionary, Disease, DiseaseTest, DiseaseType, Journal, Ordered, Phones, User, Vacation, Vactination } from '../models/user';
import { global_store } from '../store';
import { parseDate } from './date';
import { ref } from 'vue';
import { emit, listen} from '@tauri-apps/api/event';
import { AppState } from '../models/app_state';
import { event } from '@tauri-apps/api';

function is_tauri() : boolean
{
    if (window.__TAURI_IPC__)
        return true;
    else
        return false;
}


type Document = 
{
    organization?: string,
    docType?: string,
    number?: string,
    signDate?: string,
    name: string,
    parseTime: string
}
type Error = 
{
    error?: string,
}
type Packet = 
{
    document?: Document,
    error?: Error
}

type Task = 
{
    name: string,
    source_dir: string,
    target_dir: string,
    timer: number,
    delete_after_copy: boolean,
    copy_modifier: 'CopyAll' | 'CopyOnly' | 'CopyExcept',
    is_active: boolean,
    filters: Filter
}
type Filter = 
{
    document_types: string[],
    document_uids: string[]
}

export class TauriEvents
{
    static async new_document_event(func: (arg: event.Event<Packet>) => void)
    {
        if(is_tauri())
            await listen<Packet>('new_document_event', (event) => 
            {
                console.log(`Эвент new_document_event обновлен ${event.windowLabel}, payload: ${event.payload.document?.parseTime}`);
                func(event);
            });
        else
        {
            console.error("таури не заинжекчен!")
        }
    }
}

/**`C` необходим лительный тип всех команд 
 * @example
 * ```class Settings extends Plugin<'update' | 'get'>```
 */
abstract class Plugin<C extends string>  
{
    /**Имя плагина (жестко связано с кодом tauri)*/
    abstract plugin: string;
    //abstract cmd_names: C;
    /** Запуск команды таури, если таури не заинжекчен то undefined если тип string то значит пришла ошибка*/
    async save<I, O>(cmd: C, saved_obj: I) : Promise<O|undefined|string>
    {
        if (is_tauri())
        {
            try
            {
                const command = this.plugin + cmd;
                const data = await inv<O>(command, {payload: saved_obj});
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
            console.error("Инъекция Tauri не найдена, невозможно сохранить ", saved_obj);
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
                const command = this.plugin + cmd;
                const data = await inv<T>(command, args);
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
            console.error("Инъекция Tauri не найдена, невозможно выполнить команду");
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

class Settings extends Plugin<'update' | 'get'>
{
    plugin = "plugin:settings|";
    public async save_settings(types: Task[]): Promise<void|undefined|string>
    {
        return await this.save<Task[], void>('update', types);
    }
    public async load_settings(): Promise<Task[]|undefined|string>
    {
        return await this.get<Task[]>('get');
    }
}

class Service extends Plugin<'clear_dirs' | 'truncate_tasks_excepts'>
{
    plugin = "plugin:service|";
    /**очистка пакетов согласно фильтру  `clean_types = ["Квитанция"]` если фильтр пустой  
     * или отсуствует в настройках то очистка выполнятся не будет
     * @returns
     * `number` - колчиество удаленных директорий  
     * `undefined` - таури не инъекцирован  
     * `string` строка с ошибкой  
    */
    public async clear_dirs(): Promise<number|undefined|string>
    {
        return await this.get<number>('clear_dirs');
    }
    /**очистка списка исключений задач (файлы *.task)  
     * из файлов удаляются все директории которые отсутсвуют в данное время  
     * по путям указанным в настройках задач `source_dir` 
     *  
     * @returns  
     * `number` - колчиество удаленных записей  
     * `undefined` - таури не инъекцирован  
     * `string` строка с ошибкой (ошибку не возвращает, ошибки все в логах)
    */
    public async truncate_tasks_excepts(): Promise<number|undefined|string>
    {
        return await this.get<number>('truncate_tasks_excepts');
    }
}

const settings = new Settings();
const service = new Service();
export {settings, service}


// /** Запуск команды из бэкэнда, если таури не заинжекчен то undefined*/
// async function invoke<T>(cmd: string, args?: InvokeArgs) : Promise<T|undefined>
// {
//     if (is_tauri())
//     {
//         try
//         {
//             const data = await inv<T>(cmd, args);
//             return data;
//         }
//         catch(e: unknown)
//         {
//             console.error(e);
//             return new Promise<undefined>((resolve) => 
//             {
//                 resolve(undefined);
//             });
//         }
//     }
//     else
//     {
//         return new Promise<undefined>((resolve) => 
//         {
//             resolve(undefined);
//         });
//     }
// }

// async function save<I, O>(cmd: string, saved_obj: I) : Promise<undefined|O>
// {
//     if (is_tauri())
//     {
//         try
//         {
//             const data = await inv<O>(cmd, {payload: saved_obj});
//             return data;
//         }
//         catch(e: unknown)
//         {
//             console.error(e);
//             return new Promise<undefined>((resolve) => 
//             {
//                 resolve(undefined);
//             });
//         }
//     }
//     else
//     {
//         console.error("Tauri не заинжекчен, невозможно сохранить ", saved_obj);
//         return new Promise<undefined>((resolve) => 
//         {
//             resolve(undefined);
//         });
//     }
// }
