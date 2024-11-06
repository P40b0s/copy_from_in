import { listen } from '@tauri-apps/api/event';
import { AppState } from '../models/app_state';
import { DateFormat, DateTime, dateToString, parseDate, parseDateObj, parseDateObj2, timeToString } from '../services/date';
import Store from './abstract_store';
import { events} from '../services/tauri/events';
import { commands_packets, commands_service, commands_settings } from '../services/tauri/commands'
import { IPacket } from '../models/types';
import { error_sound, new_packet_notify_sound } from '../services/sounds';
import { Senders } from '../models/senders';
import { event } from '@tauri-apps/api';
import { image_ico } from '../services/svg';

/**
 * Хранилище состояний
 */
interface IGlobalAppState extends Object 
{
  server_is_online: boolean;
  //senders: Senders[];
}
class GlobalAppState implements IGlobalAppState
{
 server_is_online = false;
 //senders = [];
}

class AppStateStore extends Store<IGlobalAppState> 
{
  protected data(): IGlobalAppState
  {
    this.check_online_intervally();
    //this.subscribe_update_senders();
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
  // async get_senders()
  // {
  //   if(this.state.senders.length == 0)
  //   {
  //     const snd = await commands_packets.get_senders();
  //     this.state.senders = snd.value ?? [];
  //   }
  // }
  // get_icon(packet: IPacket)
  // {
  //   const snd = this.state.senders.find(f=>f.id == packet.packetInfo?.senderInfo?.sourceGuid);
  //   return snd?.icon ?? image_ico
  // }
  // subscribe_update_senders()
  // {
  //   events.update_sender(s=> 
  //   {
  //     const sender = s.payload;
  //     const index = this.state.senders.findIndex(f=>f.id == sender.id);
  //     if(index > 0)
  //     {
  //       this.state.senders.splice(index, 1, sender);
  //     }
  //   });
  // }
}
const store = new AppStateStore();

export default store;