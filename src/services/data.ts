import { Disease, DiseaseTest, DiseaseType, Id, Journal, JournalItem, Dictionary, Phones, TimeWarning, User,  Vactination, Status, UserStatusType } from '../models/user';
import { global_store } from '../store';
import { DateFormat, DateTime, dateToString, getDaysDiff, parseDate, parseDateObj, timeLeft } from './date';
import { Ref, onMounted, ref, watch, watchEffect } from 'vue';
import { v4 as uuidv4, v4 } from 'uuid';
import { diseases_list, ordered, test_clinics, test_data, test_departments, test_disease_types, test_journal, test_posts, test_ranks, trip, vacation_list, vactionation_list } from './test_data';
import { clinics } from './dictionaries';
import emitter from './emit';
import { TauriCommands } from './tauri';

class TypesBuilder
{
    public static build_status(user_id: string, status_type: UserStatusType, startDate : DateTime, daysCount : number = 1) : Status
    {
        const endDate = startDate.calc_end_date(daysCount, DateFormat.SerializedDateTime);
        let inst: Status =  
        {
            userId: user_id,
            id: v4(),
            place: "",
            startDate: startDate.to_string(DateFormat.SerializedDateTime),
            daysProgress: { days: 1, daysLeft: 1, progress: 100},
            statusType: status_type,
            endDate: endDate
        }
        return inst;
    }
    public static build_user(): User
    {
        return {
            id: v4(),
            name1: "",
            name2: "",
            surname: "",
            phones: [],
            diseases: [],
            vactinations: [],
            statuses: [],
            sanTicketNumber: "",
            tests: [],
            post: {id: "", name: ""},
            department: {id: "", name: ""},
            rank:  {id: "", name: ""},
            livePlace: ""
        } as User
    }
    public static build_disease(user_id: string): Disease
    {
        const date = new Date();
        const dis = {
            id: v4(),
            userId: user_id,
            diseaseType: {id: '', name: "", needReference: false} as DiseaseType,
            dateOfIllness: new DateTime().to_string(DateFormat.SerializedDateTime),
            clinic: {id: '', name: ""}
        }
        return dis;
    }

    public static build_vactination(user_id: string): Vactination
    {
        const date = new Date();
        const vac: Vactination = {
            id: v4(),
            userId: user_id,
            type: '',
            date: new DateTime().to_string(DateFormat.SerializedDateTime),
            note: ""
        };
        return vac;
    }
}


// const calcEndDate = (v: Vacation): string =>
// {
//     const dc = new DateTime(v.startDate).date.getDate() + (v.daysCount - 1);
//     const ed = new DateTime(v.startDate).set_date(dc);
//     return ed.to_string(DateFormat.CalendarFormat);
// }

// const overallVacations = () => 
// {
//     const now = global_store.getState().currentDate;
//     const vac = vacations.value.filter(v=> (now >= parseDate(v.startDate)) && (now <= parseDate(v.endDate)));
//     global_store.vacationsCount = vac.length;
//     return vac.length;
// }
// const overallDiseases = () => 
// {
//     const d = diseases.value.filter(v=> v.dateOfRecovery == undefined);
//     global_store.diseaseCount = d.length;
//     return d.length;
// }
// const updateCounts = () =>
// {
//     overallDiseases();
//     overallVacations();
// }
// watch(() => global_store.getState().stringDate, (new_date, olddate) => 
// {
//     //updateCounts();
// })

const updateUser = async (u: User) =>
{
    console.log(u);
    const updated = await TauriCommands.Users.add_or_update_users(u);
    if(updated)
    {
        emitter.emit('userUpdated');
    }
    else
    {
        console.error("Ошибка при сохранении пользователя " + u.surname);
    }
}

//как правильно использовать функию reduse
//list.push(new Person("Adult", 25));
//var oldest_person = list.reduce( (a, b) => a.Age > b.Age ? a : b );
const updateDiseases = async (dis: Disease[]) =>
{
    // const dis_upd = await TauriCommands.Statuses.update_diseases(dis);
    // if (dis_upd)
    // {
    //     emitter.emit('userUpdated');
    // }
    // else
    // {
    //     console.error("Ошибка при сохранении списка заболеваний для юзера  " + dis[0].userId);
    // }
}
// const updateVactinations = (vac: Vactination[]) =>
// {
//     vactinations.value = sortedVactinations(vac);
// }

const sortedDiseases = (dis: Disease[]): Disease[] =>
{
    dis.sort((a, b)=> 
    {
        if(a.dateOfRecovery == undefined && b.dateOfRecovery == undefined)
            return 0;
        if(a.dateOfRecovery == undefined)
            return 1;
        if(b.dateOfRecovery == undefined)
            return 1;
        if(parseDate(a.dateOfRecovery) > parseDate(b.dateOfRecovery))
            return -1;
        else
            return 1;
    });
    return dis;
}
const sortedVactinations = (vac: Vactination[]): Vactination[] =>
{
    vac.sort((a, b)=> 
    {
        if(parseDate(a.date) > parseDate(b.date))
            return -1;
        else
            return 1;
    });
    return vac;
}
// const sortedVacations = (vac: Vacation[]): Vacation[] =>
// {
//     vac.sort((a, b)=> 
//     {
//         if(parseDate(calcEndDate(a)) > parseDate(calcEndDate(b)))
//             return -1;
//         else
//             return 1;
//     });
//     return vac;
// }
// const updateVacations = (vac: Vacation[]) =>
// {
//     vacations.value = vac;
// }
// const updateOrdered = (ordered: Ordered[]) =>
// {
//     ord.value = ordered;
// }
// const updateBuisnesTrip = (ord: Ordered[]) =>
// {
//     buisnessTrip.value = ord;
// }

const sortUsers = () => 
{
    users.value.sort((u1, u2)=> 
    {
        if (u1.surname > u2.surname) {
            return 1;
        }
        if (u1.surname < u2.surname) {
            return -1;
        }
        return 0;
    });
}


const filterUsers = (filter: string) => 
{
    if(filter.length > 0)
    {
        users.value = users_backup.filter(f=>
            f.name1.toLowerCase().includes(filter) ||
            f.name2.toLowerCase().includes(filter) ||
            f.surname.toLowerCase().includes(filter)
        )
    }
    else
    {
        users.value = users_backup;
    }
}



const users = ref<User[]>([]);
const users_backup = users.value;
//const diseases = ref<Disease[]>(sortedDiseases(diseases_list()));
//const vactinations = ref<Vactination[]>(vactionation_list());
//const vacations = ref<Vacation[]>(sortedVacations(vacation_list()));
//const buisnessTrip = ref<Ordered[]>([trip]);
//const ord = ref<Ordered[]>([ordered]);





const time_warnings = ref<TimeWarning[]>
([
    new TimeWarning("Тестовое предупреждение на определенную дату 1", "17:40", [1,2], new Date(2023, 11, 28)),
    new TimeWarning("Отправить факс в спецсвязь, тест COVID на Авдеева", "10:00", [0,2]),
    new TimeWarning("Отзвонить дежурному УИС", "06:10"),
    new TimeWarning("Отзвонить дежурному Спецсвязи - 239-29", "06:15"),
    new TimeWarning("Проверка тревожной кнопки - 224-88", "07:45"),
    new TimeWarning("Передать строевую записку  239-29", "09:10"),
    new TimeWarning("Передать в дежурную службу эпидемиологическую справку", "17:00", [1,2,3,4]),
    new TimeWarning("Передать в дежурную службу эпидемиологическую справку", "16:00", [5,6,0]),
    new TimeWarning("Передать в дежурную службу противопожарный акт", "16:00", [5]),
    new TimeWarning("Тестовое предупреждение на определенную дату 1", "15:25", [1,2,3,4,5],),
    new TimeWarning("Тестовое предупреждение на определенную дату 1", "01:30", [1,2,3,4,5,6,],),
    new TimeWarning("Тестовое предупреждение на определенную дату 2", undefined, undefined, new Date(2023, 11, 29),),
    // {id: 30, date: new Date(2023, 11, 28), time: '17:40' , text: "Тестовое предупреждение на определенную дату 1", isVisible: false, showNotify: true},
    // {id: 31, date: new Date(2023, 11, 26), time: '02:10' , text: "Тестовое предупреждение на определенную дату 2", isVisible: false, showNotify: true},
    // {id: 32, date: new Date(2023, 11, 26), time: '02:03' , text: "Тестовое предупреждение на определенную дату 3", isVisible: false, showNotify: true},
    // {id: 1, time: '10:00', weekDay: [0,1,2], text: "Отправить факс в спецсвязь, тест COVID на Авдеева", isVisible: false, showNotify: true},
    // {id: 4, time: '06:10' , text: "Отзвонить дежурному УИС", isVisible: false, showNotify: true},
    // {id: 5, time: '06:15' , text: "Отзвонить дежурному Спецсвязи - 239-29", isVisible: false, showNotify: true},
    // {id: 6, time: '07:45' , text: "Проверка тревожной кнопки - 224-88", isVisible: false, showNotify: true},
    // {id: 2, time: '09:10' , text: "Передать строевую записку  239-29", isVisible: false, showNotify: true},
    // {id: 38, weekDay: [0,1,2,3],time: '17:00' , text: "Передать в дежурную службу эпидемиологическую справку", isVisible: false, showNotify: true},
    // {id: 36, weekDay: [4,5,6],time: '16:00' , text: "Передать в дежурную службу эпидемиологическую справку", isVisible: false, showNotify: true},
    // {id: 37, weekDay: [4], time: '16:00' , text: "Передать в дежурную службу противопожарный акт", isVisible: false, showNotify: true},
    // {id: 7, weekDay: [0], text: "тестовое событие только по понидельникам", isVisible: false, showNotify: true},
]) as Ref<TimeWarning[]>

const updateTimeWarnings = (tw: TimeWarning[]) =>
{
    time_warnings.value = sortTimeWarnings(tw);
}




const sortTimeWarnings = (tw: TimeWarning[]): TimeWarning[] =>
{
    tw.sort((a, b)=> 
    {
        if(a.startNotifyTime == undefined && b.startNotifyTime == undefined)
            return -1;
        if(a.startNotifyTime == undefined)
            return -1;
        if(b.startNotifyTime == undefined)
            return -1;
        //if(new Date().setHours(parseInt(a.time.split(":")[0]), parseInt(a.time.split(":")[1])) > new Date().setHours(parseInt(b.time.split(":")[0]), parseInt(b.time.split(":")[1])))
        if(a.startNotifyTime > b.startNotifyTime)
            return 1;
        else
            return -1;
    });
    return tw;
}


//updateCounts();
//sortUsers();


export {test_data as users_test_data,  sortTimeWarnings, updateTimeWarnings, time_warnings, ordered, users, TypesBuilder, updateDiseases,  filterUsers,  updateUser}