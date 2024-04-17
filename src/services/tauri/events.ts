import { event} from "@tauri-apps/api";
import { IPacket, Task } from "../../models/types"; 
import { UnlistenFn, listen } from "@tauri-apps/api/event";
import { AbstractEvents, Plugin } from "./abstract";

/**
 * задаем дженерик в виде литеральных типов, и создаем перечень эвентов
 */
export class TauriEvents extends AbstractEvents<'packets_update' | 'error' | 'task_updated' | 'task_deleted'>
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
const events = new TauriEvents();
export {events}