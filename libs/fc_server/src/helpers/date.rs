use std::fmt::Display;

use chrono::{NaiveDateTime, Local, DateTime, NaiveDate, Datelike, NaiveTime, Timelike};
use logger::{error, backtrace};
use serde::{Deserialize, Serialize};
pub const FORMAT_SERIALIZE_DATE_TIME: &'static str = "%Y-%m-%dT%H:%M:%S";
pub const FORMAT_SERIALIZE_DATE_TIME_WS: &'static str = "%Y-%m-%d %H:%M:%S";
pub const FORMAT_DOT_DATE: &'static str = "%d.%m.%Y";
pub const FORMAT_DASH_DATE: &'static str = "%d-%m-%Y";

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
/// Объект хранящий дату время, пока без оффсета
pub struct Date
{
    pub val : String
}

impl Display for Date
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result 
    {
        f.write_str(&self.write(DateFormat::DotDate))
    }
}

pub enum DateFormat
{
    ///Формат сериализации данных %Y-%m-%dT%H:%M:%S
    Serialize,
    ///Формат даты %d-%m-%Y
    OnlyDate,
    ///Формат даты dd.MM.yyyy
    DotDate,
}

pub trait DateTimeFormat
{
    ///Формат парсинга - `dd-mm-YYTHH:MM:SS`, `dd.mm.yyyy` `dd mmmm yyyy`
    fn parse(date: &str) -> Result<Box<Self>, ()>;
    fn write(&self, format : DateFormat) -> String;
    fn now() -> Box<Self>;
    fn new(year : i32, month : u32, day: u32) -> Option<Box<Self>>;
    fn as_naive_datetime(&self) -> NaiveDateTime;
}

impl DateTimeFormat for Date 
{
    fn parse(date: &str) -> Result<Box<Date>, ()>
    {
        
        if let Ok(_) = NaiveDateTime::parse_from_str(date, FORMAT_SERIALIZE_DATE_TIME)
        {
            //Дата в нужном формате, просто присваиваем и все
            Ok(Box::new(Date{val : date.to_owned()}))
        }
        else if let Ok(d) = NaiveDateTime::parse_from_str(date, FORMAT_SERIALIZE_DATE_TIME_WS)
        {
            let str = to_serialized(d);
            Ok(Box::new(Date{val : str}))
        }
        else if let Ok(d2) = NaiveDate::parse_from_str(date, FORMAT_DOT_DATE)
        {
            let dt = NaiveDateTime::new(d2, NaiveTime::from_hms_opt(0, 0, 0).unwrap());
            let str = to_serialized(dt);
            Ok(Box::new(Date{val : str}))
        }
        else 
        {
            error!("Ошибка входного формата данных - {}. Поддерживаются форматы: {}, {}", date, FORMAT_DOT_DATE, FORMAT_SERIALIZE_DATE_TIME);
            Err(())
        }
    }
    fn now() -> Box<Date>
    {
        let now = from_date_time_to_naive_date_time(Local::now());
        Box::new(Date{val : to_serialized(now)})
    }
    fn new(year : i32, month : u32, day: u32) -> Option<Box<Self>>
    {
        if let Some(date) = NaiveDate::from_ymd_opt(year, month, day)
        {
            let time = NaiveTime::from_hms_opt(0,0,0).unwrap();
            let new = NaiveDateTime::new(date, time);
            return Some(Box::new(Date{val : to_serialized(new)}));
        }
        error!("Ошибка создания даты из входных параметров Д{} М{} Г{}", day, month, year);
        None
    }
    fn as_naive_datetime(&self) -> NaiveDateTime
    {
        NaiveDateTime::parse_from_str(&self.val, FORMAT_SERIALIZE_DATE_TIME).unwrap()
    }
    fn write(&self, format : DateFormat) -> String
    {
        let date = NaiveDateTime::parse_from_str(&self.val, FORMAT_SERIALIZE_DATE_TIME).unwrap();
        match format
        {
            DateFormat::Serialize => date.format(FORMAT_SERIALIZE_DATE_TIME).to_string(),
            DateFormat::OnlyDate => date.format(FORMAT_DASH_DATE).to_string(),
            DateFormat::DotDate => date.format(FORMAT_DOT_DATE).to_string(),
        }  
    }
}

fn from_date_time_to_naive_date_time(value: DateTime<Local>) -> NaiveDateTime
{
    let time = NaiveTime::from_hms_opt(value.hour(), value.minute(), value.second()).expect("Ошибка первода даты из формата DateTime<Local> в формат NaiveTime");
    let date = NaiveDate::from_ymd_opt(value.year(), value.month(), value.day()).expect("Ошибка первода даты из формата DateTime<Local> в формат NaiveDate");
    NaiveDateTime::new(date, time)
}

fn to_serialized(date: NaiveDateTime) -> String
{
    date.format(FORMAT_SERIALIZE_DATE_TIME).to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DaysProgress
{
    ///количество дней между начальной и конечной датой
    pub days: i64,
    ///количество оставшихся дней от сегодняшней даты
    pub days_left: i64,
    ///процент для прогрессбара 0-100% (количество оставшихся дней в процентах)
    pub progress: i64

}
impl Default for DaysProgress
{
    fn default() -> Self 
    {
        Self { days: 0, days_left: 0, progress: 100 }
    }
}
impl DaysProgress
{
    ///на вход принимаент начальную дату и конечную дату <br>
    /// 1-количество дней между начальной и конечной датой<br>
    /// 2-количество оставшихся дней от сегодняшней даты<br>
    /// 3-процент для прогрессбара 0-100% (количество оставшихся дней в процентах)
    pub fn days_diff(start_date: &str, end_date: &str) -> Option<Self>
    {
        let start_date = *Date::parse(start_date).ok()?;
        let end_date = *Date::parse(end_date).ok()?;
        let date_now = *Date::now();
        let one_day = 86400; // секунд с сутках
        let end_start_timestramp_diff = end_date.as_naive_datetime().timestamp() - start_date.as_naive_datetime().timestamp();
        let diff_full_vacation = if end_start_timestramp_diff > 0
        {
            (end_start_timestramp_diff / one_day) + 1
        }
        else
        {
            0
        };
        let end_now_timestramp_diff = end_date.as_naive_datetime().with_hour(23).unwrap().with_minute(59).unwrap().timestamp() - date_now.as_naive_datetime().timestamp();
        let diff_from_now = if end_now_timestramp_diff > 0
        {
            (end_now_timestramp_diff / one_day) + 1
        }
        else
        {
            0
        };
        let process = (100.0f64 - ((diff_from_now as f64 / diff_full_vacation as f64) * 100.0f64)).floor() as i64;
        logger::info!("{} {}, {}%  {}",diff_full_vacation, diff_from_now, process, backtrace!());
        Some(Self { days: diff_full_vacation, days_left: diff_from_now, progress: process})
    }
}


//     const diffFromNow = ((end_date - date_now) / one_day) + 1;
   
//     return {
//         progress: Math.abs((100 - Math.round((diffFromNow / diffFullVacation) * 100))),
//         left: diffFromNow,
//         overall: diffFullVacation
//     };
// }

// type DateProgress = 
// {
//     /**Текущий процесс в процентах */
//     progress: number,
//     /**Количество в единицах сколько осталось */
//     left: number,
//     /**В единицах сколько между первой единицей и второй единицей */
//     overall: number
// }

fn floor()
{
    let r = 4/5;
    println!("{}",r);
}

#[cfg(test)]
mod test
{
    use logger::{StructLogger, debug};
    
    
    use super::
    {
        Date,
        DateFormat,
        DateTimeFormat
    };

    #[test]
    pub fn date_output() 
    {
        logger::StructLogger::initialize_logger();
        let date = Date::parse("26-10-2022T13:23:52").unwrap();
        debug!("Парсинг 26-10-2022T13:23:52 - {} ", date.write(DateFormat::DotDate));
        let date2 = Date::parse("26 октября 2020 г.").unwrap();
        debug!("Парсинг 26 октября 2020 г. - {} ", date2.write(DateFormat::DotDate));
        assert_eq!(date.write(DateFormat::DotDate), "26.10.2022".to_owned());
        assert_eq!(date.write(DateFormat::Serialize), "26-10-2022T13:23:52".to_owned());
        assert_eq!(date.write(DateFormat::OnlyDate), "26-10-2022".to_owned());
        debug!("Вывод в формате DotDate: {}", date.write(DateFormat::DotDate));
        debug!("Вывод в формате Serialize: {}", date.write(DateFormat::Serialize));
        debug!("Вывод в формате OnlyDate: {}", date.write(DateFormat::OnlyDate));
        debug!("Тукущее время: {}", Date::now().to_string());
        debug!("Дата 12 12 2056: {}", Date::new(2056, 12, 12).unwrap().to_string());
        
    }

    #[test]
    pub fn round() 
    {
        let start_date = "2024-01-01 00:00:00";
        let end_date = "2024-01-02- 00:00:00";
        super::DaysProgress::days_diff(start_date, end_date);
    }

}