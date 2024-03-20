import { TimeWarning, User } from '../models/user';
import { dateToString, parseDate, parseDateObj, parseDateObj2, timeToString } from '../services/date';
import Store from './abstract_store';

// const time_warnings = 
// ([
//     {id: 1, time: '09:50', weekDay: [1,3], text: "Отправить факс в спецсвязь, тесть COVID на Авдеева"},
//     {id: 2, time: '09:10' , text: "Передать строевую записку  239-29"},
//     {id: 3, date: new Date(2023, 11, 25), time: '09:10' , text: "Тестовое предупреждение на определенную дату"},
//     {id: 4, time: '06:10' , text: "Отзвонить дежурному УИС"},
// ])


/**
 * Хранилище состояний
 */
interface IGlobalState  extends Object 
{
  currentDate: Date;
  stringDate: string;
  weekDay: number;
  stringTime: string;
  usersInVacationCount: number;
  usersInDiseaseCount: number;
  timeWarnings: TimeWarning[];
}

class GlobalState  implements IGlobalState 
{
  stringDate = dateToString(new Date());
  stringTime = timeToString(new Date());
  weekDay = new Date().getDay();
  currentDate = new Date();
  usersInVacationCount = 0;
  usersInDiseaseCount = 0;
  timeWarnings: TimeWarning[] = [];

}

class GlobalStore extends Store<IGlobalState> 
{
  protected data(): IGlobalState 
  {
    let intervalId = setInterval(() => 
    {
      this.state.currentDate = new Date();
      this.state.stringDate = parseDateObj2(this.state.currentDate);
      this.state.stringTime = this.state.currentDate.getHours() + ":" + this.state.currentDate.getMinutes()
      this.state.weekDay = this.state.currentDate.getDay();
      console.log(this.state.weekDay);
    }, 60000)
    return new GlobalState ();
  }

  set vacationsCount(v: number)
  {
    this.state.usersInVacationCount = v;
    console.log("Количесво отпускников " + v)
  }
  set diseaseCount(v: number)
  {
    this.state.usersInDiseaseCount = v;
  }
 
}
const store = new GlobalStore();

export default store;

