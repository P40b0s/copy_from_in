use logger::error;
use once_cell::sync::Lazy;
use regex::Regex;
use utilites::{Date, DateFormat};
use uuid::Uuid;
use time::{OffsetDateTime, format_description, UtcOffset, Duration};
use std::time::SystemTime;

use crate::MedoParserError;

pub static RC_DATE_REGEX: Lazy<Regex> = Lazy::new(|| 
{
    regex::Regex::new("(\\d{2}).(\\d{2}).(\\d{4})").unwrap()
});
pub static MJ_DATE_REGEX: Lazy<Regex> = Lazy::new(|| 
{
    regex::Regex::new("(\\d{5,})\\s*от\\s*(\\d{2}).(\\d{2}).(\\d{4})").unwrap()
});
pub static GUID_REGEX: Lazy<Regex> = Lazy::new(|| 
{
    regex::Regex::new("^[{(]?([0-9A-F]{8}[-]?(?:[0-9A-F]{4}[-]?){3}[0-9A-F]{12})[)}]?$").unwrap()
});
const DTFORMAT: &'static str = "[year]-[month]-[day]T[hour]:[minute]:[second]";

pub trait DatesHelper
{
    
    
    // fn convert_dot_date(date: &String) -> Option<String>
    // {
    //     for cap in RC_DATE_REGEX.captures_iter(date)
    //     {
    //         let day = cap.get(1).unwrap();
    //         let month = cap.get(2).unwrap();
    //         let year = cap.get(3).unwrap();
    //         let day: u32 = day.as_str().parse().unwrap();
    //         let month: u32 = month.as_str().parse().unwrap();
    //         let year: u32 = year.as_str().parse().unwrap();
    //         let date = Date::new_date(day, month, year);
    //         return Some(date.format(DateFormat::Serialize));
    //     }
    //     logger::error!("Ошибка разбора даты {date}, неверный формат");
    //     return None;
    // }
    ///72097 от 23.01.2023
    fn extract_mj_requisites(annotation: &String) -> Option<(String, String)>
    {
        for cap in MJ_DATE_REGEX.captures_iter(annotation)
        {
            let number = cap.get(1).unwrap().as_str().to_owned();
            let dot_date = Date::parse(annotation).unwrap();
            logger::info!("Обнаружены данные министерства юстиции  {} {}", &number, &dot_date);
            return Some((number, dot_date.format(DateFormat::Serialize)));
        }   
        return None;
    }
    // fn convert_system_time(dt: SystemTime) -> Option<String>
    // {
    //     let mut offset: OffsetDateTime = dt.into();
    //     let dur = Duration::hours(3);
    //     if let Ok(utc_offset_result) = UtcOffset::from_whole_seconds(dur.as_seconds_f32().round() as i32)
    //     {
    //         offset = offset.to_offset(utc_offset_result);
    //     }
    //     let dt_format = DTFORMAT;
    //     let format = format_description::parse(
    //         &dt_format,
    //     ).ok()?;
    //     let off = offset.format(&format);
    //     match off
    //     {
    //         Ok(off) => {
    //             return Some(off);
    //         },
    //         Err(e) => 
    //         {
    //             error!("Ошибка преобразования даты: {:?}, {}", dt, e.to_string());
    //             return None;
    //         }
    //     };
    // }
}

impl DatesHelper for Date{}

pub struct Guid;
impl Guid
{
    pub fn parse(guid: &String) ->  Result<String, MedoParserError>
    {
        let guid = Uuid::parse_str(guid);
        if let Ok(g) = guid
        {
            Ok(g.to_string())
        }
        else
        {
            return Err(MedoParserError::ParseError(format!("uid {} имеет неверный формат!", guid.err().unwrap().to_string())));
        }
    }
}

#[cfg(test)]
mod tests
{
    use uuid::Uuid;

    use crate::DatesHelper;

    const TEST_GUID: &'static str = "78c44684-1a53-4b7f-bd02-6917ae14cf47";
    #[test]
    fn test_guid()
    {
        let g = String::from("{78C44684-1A53-4B7F-BD02-6917AE14CF47}");
        let g = super::Guid::parse(&g);
        println!("{}", g.unwrap());
    }

    #[test]
    fn test_guid2()
    {
       let uuid = Uuid::new_v4();
        println!("{}", uuid);
    }
    #[test]
    fn test_split_guid()
    {
        let g = String::from("{78C446841A534B7FBD026917AE14CF47}");
        let g = super::Guid::parse(&g);
        assert_eq!(g.as_ref().unwrap(), TEST_GUID);
        assert_eq!(g.as_ref().unwrap(), "78c44684-1a53-4b7f-bd02-6917ae14cf47");
        assert_eq!(*g.as_ref().unwrap(), String::from("78c44684-1a53-4b7f-bd02-6917ae14cf47"));
    }
    #[test]
    fn test_mj_data()
    {
        let mj = String::from("72097 от 23.01.2023");
        let res = super::Date::extract_mj_requisites(&mj).unwrap();
        println!("№ {} дата: {}", res.0, res.1);
    }
}