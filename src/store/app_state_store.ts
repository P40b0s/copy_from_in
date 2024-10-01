import { listen } from '@tauri-apps/api/event';
import { AppState } from '../models/app_state';
import { DateFormat, DateTime, dateToString, parseDate, parseDateObj, parseDateObj2, timeToString } from '../services/date';
import Store from './abstract_store';
import { events} from '../services/tauri/events';
import { commands_service, commands_settings } from '../services/tauri/commands'
import { IPacket } from '../models/types';
import { error_sound, new_packet_notify_sound } from '../services/sounds';

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
    //this.get_packets_log();
    this.listen_log();
    this.check_online_intervally();
    return new GlobalAppState();
  }
  check_online_intervally()
  {
    this.check_online();
    let intervalId = setInterval(() => 
    {
     this.check_online();
    }, 7000)
  }

  async check_online()
  {
    const result = await commands_service.ws_server_online()
    if (result.is_ok())
    {
      this.state.server_is_online = result.get_value();
    }
  }

  // async get_packets_log()
  // {
  //   const packets = await commands_settings.get_packets_list();
  //   if (packets.is_err())
  //   {
  //     console.error("Ошибка получения лога пакетов с сервера " + packets.get_error());
  //     this.state.current_log = [];
  //   }
  //   else
  //   {
  //     this.state.current_log = packets.get_value();
  //   }
  // }
  async listen_log()
  {
    await events.packets_update((doc) => 
    {
      const pl = doc.payload;
      if(pl.task.sound)
      {
        if(pl.error != undefined)
          error_sound();
        
        else
          new_packet_notify_sound();
      }
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