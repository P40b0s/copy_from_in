import { listen } from '@tauri-apps/api/event';
import { AppState } from '../models/app_state';
import { DateFormat, DateTime, dateToString, parseDate, parseDateObj, parseDateObj2, timeToString } from '../services/date';
import Store from './abstract_store';
import { events} from '../services/tauri/events';
import { service, settings } from '../services/tauri/commands'
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
    this.get_packets_log();
    this.listen_log();
    this.check_online();
    return new GlobalAppState();
  }
  check_online()
  {
    let intervalId = setInterval(async () => 
    {
      const result = await service.ws_server_online()
      console.log(result)
      if (result.is_ok())
      {
        this.state.server_is_online = result.get_value();
        console.log(result.get_value());
      }
    }, 7000)
  }
  async get_packets_log()
  {
    const packets = await settings.get_packets_list();
    if (packets.is_err())
    {
      console.error("Ошибка получения лога пакетов с сервера " + packets.get_error());
      this.state.current_log = [];
    }
    else
    {
      this.state.current_log = packets.get_value();
    }
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

const get_packets_log = async (): Promise<IPacket[]> =>
{
  const packets = await settings.get_packets_list();
  if (packets.is_err())
  {
    console.error("Ошибка получения лога пакетов с сервера " + packets.get_error());
    return [];
  }
  else
  {
    return packets.get_value();
  }
}