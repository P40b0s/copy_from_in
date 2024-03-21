import { listen } from '@tauri-apps/api/event';
import { AppState } from '../models/app_state';
import { DateFormat, DateTime, dateToString, parseDate, parseDateObj, parseDateObj2, timeToString } from '../services/date';
import Store from './abstract_store';
import { TauriEvents } from '../services/tauri-service';
import { IPacket } from '../models/types';

/**
 * Хранилище состояний
 */
interface IGlobalAppState extends Object 
{
  current_log: IPacket[];
}
class GlobalAppState implements IGlobalAppState
{
 current_log: IPacket[] = [];
}

class AppStateStore extends Store<IGlobalAppState> 
{
  protected data(): IGlobalAppState
  {
    this.listen_log();
    return new GlobalAppState();
  }

  async listen_log()
  {
    await TauriEvents.new_document_event((doc) => 
    {
      console.log(doc);
      const pl = doc.payload
      this.add_packet(doc.payload);
      // if(pl.error)
      // {
      //   this.state.current_log.push(pl.error);
      // }
      // if(pl.document)
      // {
      //   this.state.current_log.push(pl.document.name);
      // }
    })
  }

  /**Добавляем пакет в начало списка, если список больше 5000 то удаляем последний в списке */
  add_packet(packet: IPacket)
  {
    this.state.current_log.splice(0,0, packet);
    if(this.state.current_log.length > 5000)
      this.state.current_log.pop();
  }
}
const store = new AppStateStore();

export default store;