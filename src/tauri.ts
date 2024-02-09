// import { InvokeArgs, invoke as inv,} from '@tauri-apps/api/tauri';
// import { Dictionary, Disease, DiseaseTest, DiseaseType, Journal, Ordered, Phones, User, Vacation, Vactination } from '../models/user';
// import { global_store } from '../store';
// import { parseDate } from './date';
// import { ref } from 'vue';
// import { emit, listen} from '@tauri-apps/api/event';
// import { AppState } from '../models/app_state';
// import { event } from '@tauri-apps/api';

// function is_tauri() : boolean
// {
//     if (window.__TAURI_IPC__)
//         return true;
//     else
//         return false;
// }


// type Document = 
// {
//     organization?: string,
//     docType?: string,
//     number?: string,
//     signDate?: string,
//     name: string,
//     parseTime: string
// }
// type Error = 
// {
//     error?: string,
// }
// type Packet = 
// {
//     document?: Document,
//     error?: Error
// }

// type Task = 
// {
//     name: string,
//     source_dir: string,
//     target_dir: string,
//     timer: number,
//     delete_after_copy: boolean,
//     copy_modifier: 'copy_all' | 'copy_only' | 'copy_except',
//     is_active: boolean,
//     filters: Filter
// }
// type Filter = 
// {
//     document_types: string[],
//     document_uids: string[]
// }

// export class TauriEvents
// {
//     static async new_document_event(func: (arg: event.Event<Packet>) => void)
//     {
//         if(is_tauri())
//             await listen<Packet>('new_document_event', (event) => 
//             {
//                 console.log(`Эвент new_document_event обновлен ${event.windowLabel}, payload: ${event.payload.document?.parseTime}`);
//                 func(event);
//             });
//         else
//         {
//             console.error("таури не заинжекчен!")
//         }
//     }
// }


// abstract class Plugin  
// {
//     abstract plugin: string;
//     abstract cmd_names: string[];
//     /** Запуск команды таури, если таури не заинжекчен то undefined если тип string то значит пришла ошибка*/
//     async save<I, O>(cmd: string, saved_obj: I) : Promise<O|undefined|string>
//     {
//         if (is_tauri())
//         {
//             try
//             {
//                 const data = await inv<O>(cmd, {payload: saved_obj});
//                 return data;
//             }
//             catch(e: unknown)
//             {
//                 console.error(e);
//                 return new Promise<string>((resolve) => 
//                 {
//                     resolve(String(e));
//                 });
//             }
//         }
//         else
//         {
//             console.error("Tauri не заинжекчен, невозможно сохранить ", saved_obj);
//             return new Promise<undefined>((resolve) => 
//             {
//                 resolve(undefined);
//             });
//         }
//     }
//     /** Запуск команды таури, если таури не заинжекчен то undefined если тип string то значит пришла ошибка*/
//     async get<T>(cmd: string, args?: InvokeArgs) : Promise<T|undefined|string>
//     {
//         if (is_tauri())
//         {
//             try
//             {
//                 const data = await inv<T>(cmd, args);
//                 return data;
//             }
//             catch(e: unknown)
//             {
//                 console.error(e);
//                 return new Promise<string>((resolve) => 
//                 {
//                     resolve(String(e));
//                 });
//             }
//         }
//         else
//         {
//             console.error("Tauri не заинжекчен, невозможно выполнить команду");
//             return new Promise<undefined>((resolve) => 
//             {
//                 resolve(undefined);
//             });
//         }
//     }
//     functionGenerator = <T extends string, U = { [K in T]?: string }>(keys: T[]): U => 
//     {
//         return keys.reduce((oldType: any, type) => ({ ...oldType, [type]: type }), {})
//     }
// }

// class Settings extends Plugin
// {
//     plugin = "plugin:settings|";
//     cmd_names = ['update', 'get'];
//     public async save_settings(types: Task[]): Promise<void|undefined|string>
//     {
//         return await this.save<Task[], void>(this.plugin + this.cmd_names[0], types);
//     }
//     public async load_settings(): Promise<Task[]|undefined|string>
//     {
//         return await this.get<Task[]>(this.plugin + this.cmd_names[1]);
//     }
// }

// const settings = new Settings();
// export {settings}

// export class TauriCommands
// {
//     static Settings = class extends Plugin
//     {
//         plugin = "plugin:settings|";
//         public static async save_settings(types: Task[]): Promise<DiseaseType[]|undefined>
//         {
//             return await save_cmd<DiseaseType[]>(TauriCommands.Dictionaries.dictionary_plugin + 'update', types);
//         }
//         public static async get_diseases_types(): Promise<DiseaseType[]|undefined>
//         {
//             return await invoke<DiseaseType[]>(TauriCommands.Dictionaries.dictionary_plugin + 'get_diseases_types');
//         }
//         public static async get_departments(): Promise<Dictionary[]|undefined>
//         {
//             return await invoke<Dictionary[]>(TauriCommands.Dictionaries.dictionary_plugin + 'get_departments');
//         }
//         public static async save_departments(posts: Dictionary[]): Promise<Dictionary[]|undefined>
//         {
//             return await save_cmd<Dictionary[]>(TauriCommands.Dictionaries.dictionary_plugin + 'save_departments', posts);
//         }
//         public static async get_posts(): Promise<Dictionary[]|undefined>
//         {
//             return await invoke<Dictionary[]>(TauriCommands.Dictionaries.dictionary_plugin + 'get_posts');
//         }
//         public static async save_posts(posts: Dictionary[]): Promise<Dictionary[]|undefined>
//         {
//             return await save_cmd<Dictionary[]>(TauriCommands.Dictionaries.dictionary_plugin + 'save_posts', posts);
//         }
//         public static async get_clinics(): Promise<Dictionary[]|undefined>
//         {
//             return await invoke<Dictionary[]>(TauriCommands.Dictionaries.dictionary_plugin + 'get_clinics');
//         }
//         public static async save_clinics(posts: Dictionary[]): Promise<Dictionary[]|undefined>
//         {
//             return await save_cmd<Dictionary[]>(TauriCommands.Dictionaries.dictionary_plugin + 'save_clinics', posts);
//         }
//         public static async get_ranks(): Promise<Dictionary[]|undefined>
//         {
//             return await invoke<Dictionary[]>(TauriCommands.Dictionaries.dictionary_plugin + 'get_ranks');
//         }
//         public static async save_ranks(posts: Dictionary[]): Promise<Dictionary[]|undefined>
//         {
//             return await save_cmd<Dictionary[]>(TauriCommands.Dictionaries.dictionary_plugin + 'save_ranks', posts);
//         }
//     } 
//     static Users = class
//     {
//         static users_plugin = "plugin:users|"
//         public static async get_users<R extends User[]>(rows: number, offset: number): Promise<R|undefined>
//         {
//             return await invoke<R>(TauriCommands.Users.users_plugin + 'get', {pagination: {row: rows, offset: offset}});
//         }
//         public static async add_or_update_users(user: User): Promise<User|undefined>
//         {
//             return await save_cmd<User>(TauriCommands.Users.users_plugin + 'add_or_update', user);
//         }
//     }
//     static Statuses = class
//     {
//         static plugin = "plugin:statuses|"
//         public static async update_diseases<R extends AppState>(dis: Disease[], userId: string): Promise<R|undefined>
//         {
//             return await invoke<R>(TauriCommands.Statuses.plugin + 'update_diseases', {payload: dis, userId: userId});
//         }
//         public static async update_statuses<R extends AppState>(ord: Ordered[], userId: string): Promise<R|undefined>
//         {
//             return await invoke<R>(TauriCommands.Statuses.plugin + 'update_ordered', {payload: ord, userId: userId});
//         }
//         public static async update_vacations<R extends AppState>(vac: Vacation[], userId: string): Promise<R|undefined>
//         {
//             return await invoke<R>(TauriCommands.Statuses.plugin + 'update_vacations', {payload: vac, userId: userId});
//         }
//     }
    
//     static Helpers = class
//     {
//         static date_plugin = "plugin:date|"
//         public static async get_date_now<R extends string>(): Promise<R|undefined>
//         {
//             return await invoke<R>(TauriCommands.Helpers.date_plugin + 'get_date_now');
//         }
//         public static async initialize_app_state<R extends AppState>(): Promise<R|undefined>
//         {
//             return await invoke<R>('initialize_app_state');
//         }
//     }
// }



// class DictionariesCommands
// {
//     static dictionary_plugin = "plugin:dictionaries|";

//     public static async save_diseases_types(types: DiseaseType[]): Promise<DiseaseType[]|undefined>
//     {
//         return await save_cmd<DiseaseType[]>(DictionariesCommands.dictionary_plugin + 'save_diseases_types', types);
//     }
//     public static async get_diseases_types(): Promise<DiseaseType[]|undefined>
//     {
//         return await invoke<DiseaseType[]>(DictionariesCommands.dictionary_plugin + 'get_diseases_types');
//     }
//     public static async get_departments(): Promise<Dictionary[]|undefined>
//     {
//         return await invoke<Dictionary[]>(DictionariesCommands.dictionary_plugin + 'get_departments');
//     }
//     public static async save_departments(posts: Dictionary[]): Promise<Dictionary[]|undefined>
//     {
//         return await save_cmd<Dictionary[]>(DictionariesCommands.dictionary_plugin + 'save_departments', posts);
//     }
//     public static async get_posts(): Promise<Dictionary[]|undefined>
//     {
//         return await invoke<Dictionary[]>(DictionariesCommands.dictionary_plugin + 'get_posts');
//     }
//     public static async save_posts(posts: Dictionary[]): Promise<Dictionary[]|undefined>
//     {
//         return await save_cmd<Dictionary[]>(DictionariesCommands.dictionary_plugin + 'save_posts', posts);
//     }
//     public static async get_clinics(): Promise<Dictionary[]|undefined>
//     {
//         return await invoke<Dictionary[]>(DictionariesCommands.dictionary_plugin + 'get_clinics');
//     }
//     public static async save_clinics(posts: Dictionary[]): Promise<Dictionary[]|undefined>
//     {
//         return await save_cmd<Dictionary[]>(DictionariesCommands.dictionary_plugin + 'save_clinics', posts);
//     }
//     public static async get_ranks(): Promise<Dictionary[]|undefined>
//     {
//         return await invoke<Dictionary[]>(DictionariesCommands.dictionary_plugin + 'get_ranks');
//     }
//     public static async save_ranks(posts: Dictionary[]): Promise<Dictionary[]|undefined>
//     {
//         return await save_cmd<Dictionary[]>(DictionariesCommands.dictionary_plugin + 'save_ranks', posts);
//     }
// }
// class UserCommands
// {
//     static users_plugin = "plugin:users|"
//     public static async get_users<R extends User[]>(rows: number, offset: number): Promise<R|undefined>
//     {
//         return await invoke<R>(UserCommands.users_plugin + 'get', {pagination: {row: rows, offset: offset}});
//     }
//     public static async add_or_update_users(user: User): Promise<User|undefined>
//     {
//         return await save_cmd<User>(UserCommands.users_plugin + 'add_or_update', user);
//     }

// }
// class HelpersCommands
// {
//     static date_plugin = "plugin:date|"
//     public static async get_date_now<R extends string>(): Promise<R|undefined>
//     {
//         return await invoke<R>(HelpersCommands.date_plugin + 'get_date_now');
//     }
//     public static async initialize_app_state<R extends AppState>(): Promise<R|undefined>
//     {
//         return await invoke<R>('initialize_app_state');
//     }
// }



// enum TauriGetCmd
// {
//     /**возвращает дату string в формате 2023-01-23 12:54:23  */
//     DateNow = 'get_date_now_command',
//     /**возвращает DiseaseType[] */
//     DiseasesTypes = 'get_diseases_types_command',
//      /**возвращает Dictionary[] */
//     Clinics = 'get_clinics',
//     /**возвращает Dictionary[] */
//     Ranks = 'get_ranks',
//     /**возвращает Journal за сегодняшнее число по умолчанию, если добавить дату в формате сериализации то возвратит за эту дату */
//     Journal = 'get_journal',
//      /**возвращает Dictionary[] */
//     Posts = 'get_posts',
//      /**возвращает Dictionary[] */
//     Departments = 'get_departments',
//     /**возвращает состояние которое обновляется в менеджере таури*/
//     AppState = 'initialize_app_state_command',
//     GetUsers = 'get_users',
    
// }
// /**Команды для сохранения объектов */
// enum TauriSaveCmd
// {
//     Journal = "save_journal",
//     DiseasesTypes = 'save_diseases_types',
//     AddOrUpdateUser = 'add_or_update_user_command',
//     UpdateDiseseTypes = 'save_diseases_types_command',
//     UpdatePosts = 'save_posts_command',
// }

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
// /** Запуск команды из бэкэнда, если таури не заинжекчен то вернуть аказанное дефолтное значение*/
// async function invoke_or_default<T>(cmd: TauriGetCmd, default_value: T, args?: InvokeArgs) : Promise<T>
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
//             return new Promise<T>((resolve) => 
//             {
//                 resolve(default_value);
//             });
//         }
//     }
//     else
//     {
//         return new Promise<T>((resolve) => 
//         {
//             resolve(default_value);
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
