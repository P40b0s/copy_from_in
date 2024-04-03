import { match } from "ts-pattern";

/**преобразование строки в объект `Date`*/
const parseDate = (date: string): Date =>
{
    const splitted = date.split(".").map(m=> Number.parseInt(m));
    let d: Date = new Date(splitted[2], splitted[1]-1, splitted[0]);
    return d;
}
/**преобразование из даты в виде номера в строку */
const parseDateObj = (date: number): string =>
{
    let d: Date = new Date(date);
    return d.getDate() + "." + (d.getMonth()+1) + "." + d.getFullYear();
}
/**преобразование из даты в виде объекта даты `Date` в строку */
const parseDateObj2 = (date: Date): string =>
{
    return date.getDate() + "." + (date.getMonth() + 1) + "." + date.getFullYear();
}
const dateToString = (date: Date) => 
{
    return date.toLocaleString('ru-RU', 
    {
        year: "numeric",
        month: '2-digit',
        day: '2-digit',

    })
}
const timeToString = (date: Date) => 
{
    return date.toLocaleString('ru-RU', 
    {
        hour: '2-digit',
        minute: '2-digit',
        
    })
}

enum DateFormat
{
    /**"2024-01-23T23:54:00" */
    SerializedDateTime,
    /**"2024-01-23" */
    SerializedDate,
    /**"23:54" без секунд */
    Time,
    /**"24.05.1983" */
    DotDate,
    CalendarFormat = 'dd.MM.yyyy',
    
}

class DateTime
{
    public year: number = 0
    public mounth: number = 0
    public day: number = 0
    public hour: number = 0
    public minute: number = 0;
    public weekDay: number = 0;
    public date: Date = new Date();
    constructor(date?: string|Date|number)
    {
        if(!date)
        {
            this.date = new Date();
            this.weekDay = this.date.getDay();
            this.year = this.date.getFullYear();
            this.mounth = this.date.getMonth() + 1;
            this.day = this.date.getDate();
            this.hour = this.date.getHours();
            this.minute = this.date.getMinutes();
        }
        else if (typeof date === "string")
        {
            const serialized_format_splitted = date.split("T");
            if (serialized_format_splitted.length > 1)
            {
                const date = serialized_format_splitted[0];
                const time = serialized_format_splitted[1];
                this.split_date(date, "-", [0, 1, 2])
                this.split_time(time);
                this.date = new Date(this.year, this.mounth - 1, this.day, this.hour, this.minute);
                this.weekDay = this.date.getDay();
            } 
            else if (date.split(".").length > 1)
            {
                this.split_date(date, ".", [2, 1, 0]);
                this.date = new Date(this.year, this.mounth - 1, this.day, this.hour, this.minute);
            }
            else if (date.split("-").length > 1)
            {
                this.split_date(date, "-", [0, 1, 2]);
                this.date = new Date(this.year, this.mounth - 1, this.day, this.hour, this.minute);
            }
        }
        else if (typeof date === "number")
        {
            this.date = new Date(date);
            this.weekDay = this.date.getDay();
            this.year = this.date.getFullYear();
            this.mounth = this.date.getMonth() + 1;
            this.day = this.date.getDate();
            this.hour = this.date.getHours();
            this.minute = this.date.getMinutes();
        }
        else
        {
            this.date = date as Date;
            this.weekDay = this.date.getDay();
            this.year = this.date.getFullYear();
            this.mounth = this.date.getMonth() + 1;
            this.day = this.date.getDate();
            this.hour = this.date.getHours();
            this.minute = this.date.getMinutes();
        }
    }

    public set_date(date: number): DateTime
    {
        this.date.setDate(date);
        this.weekDay = this.date.getDay();
        this.year = this.date.getFullYear();
        this.mounth = this.date.getMonth() + 1;
        this.day = this.date.getDate();
        this.hour = this.date.getHours();
        this.minute = this.date.getMinutes();
        return this;
    }
    public set_hours(h: number, m: number): DateTime
    {
        this.date.setHours(h, m, 0, 0);
        this.hour = this.date.getHours();
        this.minute = this.date.getMinutes();
        return this;
    }
    public gt(date: DateTime): boolean
    {
        return this.as_date() > date.as_date()
    }
    public lt(date: DateTime): boolean
    {
        return this.as_date() < date.as_date()
    }
    public eq(date: DateTime): boolean
    {
        return this.as_date() == date.as_date()
    }
    public greater_or_equal_then(date: DateTime): boolean
    {
        return this.gt(date) || this.eq(date); 
    }
    public lower_or_equal_then(date: DateTime): boolean
    {
        return this.lt(date) || this.eq(date); 
    }
    public as_date(): Date
    {
        return this.date;
    }

    public to_string(format: DateFormat): string
    {
        const day = this.day < 10 ? "0"+this.day.toString() : this.day.toString();
        const month = this.mounth < 10 ? "0"+this.mounth.toString() : this.mounth.toString();
        const hour = this.hour < 10 ? "0"+this.hour.toString() : this.hour.toString();
        const minute = this.minute < 10 ? "0"+this.minute.toString() : this.minute.toString();
        const sec = "00";
        const f = match(format)
        .returnType<string>()
        .with(DateFormat.SerializedDateTime, () => `${this.year}-${month}-${day}T${hour}:${minute}:${sec}`)
        .with(DateFormat.SerializedDate, () => `${this.year}-${month}-${day}`)
        .with(DateFormat.DotDate, () => `${day}.${month}.${this.year}`)
        .with(DateFormat.Time, () => `${hour}:${minute}`)
        .with(DateFormat.CalendarFormat, () => `${day}.${month}.${this.year}`)
        .otherwise(()=> `${this.year}-${month}-${day}T${hour}:${minute}:${sec}`);
        return f;
    }
    /**год - 0 месяц - 1 день - 2 */
    split_date(date: string, splitter: string, ord: number[])
    {
        const splitted_date = date.split(splitter).map(m=>Number.parseInt(m));
        this.year = splitted_date[ord[0]];
        this.mounth = splitted_date[ord[1]];
        this.day = splitted_date[ord[2]];
    }
    split_time(time: string)
    {
        const splitted_time = time.split(":").map(m=>Number.parseInt(m));
        if (splitted_time.length > 1)
        {
            this.hour = splitted_time[0];
            this.minute = splitted_time[1]
        }
    }

    public calc_end_date = (days_count : number = 1, format: DateFormat): string =>
    {
        const dc = this.date.getDate() + days_count;
        const ed = this.set_date(dc);
        return ed.to_string(format);
    }
}

type DateProgress = 
{
    /**Текущий процесс в процентах */
    progress: number,
    /**Количество в единицах сколько осталось */
    left: number,
    /**В единицах сколько между первой единицей и второй единицей */
    overall: number
}
type TimeProgress = 
{
    /**Текущий процесс в процентах */
    progress: number,
    /**Количество в минутах сколько осталось */
    minutes_left: number,
    /**Количество в часах сколько осталось */
    hours: number,
    minutes: number
}

const getDaysDiff = (start_date: Date, end_date: Date) : DateProgress =>
{
    const date_now = new Date().setHours(0, 0, 0);
    const oneDay = 24 * 60 * 60 * 1000; // hours*minutes*seconds*milliseconds
    const diffFullVacation = Math.round(Math.abs((end_date.getTime() - start_date.getTime()) / oneDay)) + 1;
    //console.log(start_date, end_date);
    const diffFromNow = Math.round(Math.abs((end_date.getTime() - date_now) / oneDay)) + 1;
    //console.log(diffFullVacation, diffFromNow, Math.round((diffFromNow / diffFullVacation) * 100))
    return {
        progress: Math.abs((100 - Math.round((diffFromNow / diffFullVacation) * 100))),
        left: diffFromNow,
        overall: diffFullVacation
    };
}

const timeLeft = (target_time: number|undefined): TimeProgress | undefined =>
{
    if (target_time)
    {
        const date_now = new Date();
        const oneMinute = 60 * 1000; // hours*minutes*seconds*milliseconds
        const diffFullVacation = Math.round(Math.abs((target_time - date_now.getTime()) / oneMinute)) + 1;
        //console.log(target_time, diffFullVacation);
        const diffFromNow = Math.round(Math.abs((date_now.getTime() - date_now.setHours(0, 0, 0, 0)) / oneMinute)) + 1;
        const process = Math.trunc((diffFullVacation/diffFromNow )*100);
        //console.log(process, diffFromNow, Math.trunc((diffFromNow /diffFullVacation )*100))
        return {
            progress: process,
            minutes_left: diffFullVacation,
            hours: Math.trunc(diffFullVacation / 60),
            minutes: diffFullVacation % 60,
        }
    }
}


export { DateTime, DateFormat, parseDate, parseDateObj, parseDateObj2, timeToString, dateToString, getDaysDiff, timeLeft, type TimeProgress}