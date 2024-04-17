import { listen } from '@tauri-apps/api/event';
import { AppState } from '../models/app_state';
import { DateFormat, DateTime, dateToString, parseDate, parseDateObj, parseDateObj2, timeToString } from '../services/date';
import Store from './abstract_store';
import { events, service } from '../services/tauri/tauri-service';
import { IPacket } from '../models/types';

/**
 * Хранилище состояний
 */
interface IGlobalAppState extends Object 
{
  server_is_online: boolean;
  current_log: IPacket[];
}
class GlobalAppState implements IGlobalAppState
{
 current_log: IPacket[] = [];
 server_is_online = false;
}

class AppStateStore extends Store<IGlobalAppState> 
{
  protected data(): IGlobalAppState
  {
    this.listen_log();
    this.check_online();
    return new GlobalAppState();
  }
  
  check_online()
  {
    let intervalId = setInterval(async () => 
    {
      const result = await service.ws_server_online()
      if (typeof result === 'boolean')
      {
        this.state.server_is_online = result;
      }
    }, 7000)
  }
  async listen_log()
  {
    await events.packets_update((doc) => 
    {
      console.log(doc);
      this.add_packet(doc.payload);
    })
  }

  /**Добавляем пакет в начало списка, если список больше 5000 то удаляем последний в списке */
  add_packet(packet: IPacket)
  {
    this.state.current_log.splice(0,0, packet);
    if(this.state.current_log.length > 1000)
      this.state.current_log.pop();
  }
}
const store = new AppStateStore();

export default store;