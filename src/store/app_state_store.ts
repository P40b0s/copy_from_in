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
}
class GlobalAppState implements IGlobalAppState
{
 server_is_online = false;
}

class AppStateStore extends Store<IGlobalAppState> 
{
  protected data(): IGlobalAppState
  {
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
 
}
const store = new AppStateStore();

export default store;