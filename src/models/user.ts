import { RendererElement, RendererNode, VNode } from "vue";
import { TimeProgress, timeLeft } from "../services/date";
import {v4} from 'uuid'
import { UserStatus } from '../modules/user_status';
import { number } from "ts-pattern/dist/patterns";
type User = 
{
    name1: string,
    name2: string,
    surname: string,
    post: Dictionary,
    department: Dictionary,
    sanTicketNumber: String,
    rank: Dictionary,
    /**место жительства */
    livePlace: string,
    phones: Phones[]
    /**тесты на заболевание (фактически только для ковид но дальше хз) */
    tests: DiseaseTest[],
    diseases: Disease[],
    statuses: Status[],
} & Id

type Id = 
{
    id: string;
}
type Phones = 
{
    phoneType: string,
    phoneNumber: string,
    isMain: boolean
}

type Disease = Id & UserIdLink &
{
    /**тип болезни */
    diseaseType: DiseaseType,
    /**дата заболевания*/
    dateOfIllness: string
    /**дата выздоровления*/
    dateOfRecovery?: string
    /**к какой поликлиннике приписан */
    clinic: Dictionary,
    note?: string
}
const get_active_dis = (d: Disease[]): DiseaseType|undefined => 
{
    return d.find(f=>!f.dateOfRecovery)?.diseaseType
}


type DiseaseTest =
{
    /**положительный или отрицательный тест */
    isActive: boolean,
    /**дата тестирования */
    date: string
}
type Vactination =
{
    /**от какого типа болезни была вакцинация */
    type: string,
    /**дата вакцинации */
    date: string,
     /**дополнительная информация, отвод от вакцинации итд */
    note: string
} & Id & UserIdLink

type DaysProgress = 
{
     ///количество дней между начальной и конечной датой
    days: number,
     ///количество оставшихся дней от сегодняшней даты
    daysLeft: number,
     ///процент для прогрессбара 0-100% (количество оставшихся дней в процентах)
    progress: number
}
/**командировка или распоряжение или отпуск  или еще чего, почему нет на рабочем месте */
type Status = 
{
    /**дата начала отпуска */
    startDate: string,
    /**дата окончания отпуска */
    endDate: string,
    daysProgress: DaysProgress,
    statusType: UserStatusType,
    /**место проведения отпуска */
    place: string
} & Id & UserIdLink

enum UserStatusType
{
    Vacation = 0,
    Ordered = 1,
    Trip = 2
}

/**Должность пока обернем в тип, возможно потребуется другое склонение на потом */

type UserIdLink = 
{
    userId : string
}
type Dictionary =
{
    name: string
} & Id
/**тип для отображения варнинга привязанного ко времени если выбран день недели то будет предупреждение только в этот день недели
 * если выбрана дата и время то будет предупреждение только в эту дату, если выбрано только время то это будет повторяющееся событие
*/
// type TimeWarning =
// {
//     weekDay?: number[],
//     date?: Date,
//     time?: string,
//     time2?: Date,
//     text: string,
//     showNotify: boolean,
//     isVisible: boolean,
//     progress:() => number,
//     minutesLeft:() => number
// } & Id

class TimeWarning
{
    _notifyBeforeTargetTime: number;
    _time?: string;
    constructor(text: string, time?: string, weekDay?: number[], date?: Date, notifyBeforeTargetTime: number = 30)
    {
        this.id = v4();
        this._notifyBeforeTargetTime = notifyBeforeTargetTime;
        this.time = time;
        this.date = date;
        this.text = text;
        this.weekDay = weekDay;
    }
    id: string;
    weekDay?: number[];
    date?: Date;
    get time(): string|undefined
    {
        return this._time
    }    
    set time(t: string|undefined)
    {
        if(t)
        {
            const warn_time = t.split(":");
            this.warningTime = new Date().setHours((parseInt(warn_time[0])), (parseInt(warn_time[1])), 0, 0)
            this.startNotifyTime = new Date().setHours((parseInt(warn_time[0])), (parseInt(warn_time[1]) - this._notifyBeforeTargetTime), 0, 0)
            console.log(t);
            this._time = t;
        }
    }
    warningTime?: number;
    startNotifyTime?: number;
    text: string = "";
    showNotify: boolean = true;
    isVisible: boolean = false;
    //методы класса не клонируются как положено, вроде изза того что надо делать Object.Assign потому что клонирование идет через JSON сериализацию
    // progress(): TimeProgress
    // {
    //     const t = timeLeft(this.warningTime);
    //     if(t)
    //         return t
    //     else
    //         return {progress: 0, minutes_left: 0, hours: 0, minutes: 0}
    // }
    //minutesLeft:() => number;
}

type JournalItem = 
{
    time: string,
    note: string
}
type Journal = 
{
    date: string,
    items: JournalItem[]
}

type DiseaseType = Dictionary &
{
    needReference: boolean
}

function enumKeys<O extends object, K extends keyof O = keyof O>(obj: O): K[] 
{
    return Object.keys(obj).filter(k => !Number.isNaN(k)) as K[]
}

const get_dict_value = (d: Dictionary[], id: string): Dictionary =>
{
    const r = d.find(f=>f.id == id)
    return r as Dictionary;
}
type EditorType = 'edit' | 'new';
type DictionaryEditorType = 'departments' | 'posts' | 'clinic' | 'disease' | 'ranks';
type ComponentType = VNode<RendererNode, RendererElement, { [key: string] : any; }>;
export type {User, DiseaseType, Phones, Disease,  DiseaseTest,  JournalItem, Status, Vactination, Journal, ComponentType, EditorType, Id, Dictionary, DictionaryEditorType}
export {enumKeys, TimeWarning, get_dict_value, get_active_dis, UserStatusType }