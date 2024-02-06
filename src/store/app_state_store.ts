import { listen } from '@tauri-apps/api/event';
import { AppState } from '../models/app_state';
import { TimeWarning, User } from '../models/user';
import { DateFormat, DateTime, dateToString, parseDate, parseDateObj, parseDateObj2, timeToString } from '../services/date';
import Store from './abstract_store';
import { TauriCommands, TauriEvents } from '../services/tauri';

/**
 * Хранилище состояний
 */
const initialize_date = new DateTime();
const initialize_state = await TauriCommands.Helpers.initialize_app_state();
interface IGlobalAppState extends Object 
{
  stringDate: string;
  stringTime: string;
  weekDay: number;
  appState: AppState;
}
const date = await TauriCommands.Helpers.get_date_now();
class GlobalAppState implements IGlobalAppState
{
  stringDate = initialize_date.to_string(DateFormat.SerializedDate);
  stringTime = initialize_date.to_string(DateFormat.Time);
  weekDay = initialize_date.weekDay;
  appState: AppState = initialize_state ?? {
    current_date: initialize_date.to_string(DateFormat.SerializedDateTime),
    diseases_count: 0,
    vacations_count: 0,
    buisness_trip_count: 0,
    users_count: 0,
    ordered_count: 0,
    current_disease_users: [],
    users_with_statuses: []
  }
}

class AppStateStore extends Store<IGlobalAppState> 
{
  protected data(): IGlobalAppState
  {
    this.listen_time();
    return new GlobalAppState();
  }

  async listen_time()
  {
    // if(is_tauri())
    // await listen<AppState>('update_state', (event) => 
    // {
    //   console.log(`Got refresh event ${event.windowLabel}, payload: ${event.payload.current_date}`);
    //   this.state.appState = event.payload;
    // });
    await TauriEvents.update_state_event((e)=>
    {
      this.state.appState = e.payload;
    })
  }
  public get_app_state(): AppState
  {
    return this.state.appState
  }

  public set set_app_state(a: AppState)
  {
    this.state.appState = a;
  }
}
const store = new AppStateStore();

export default store;

