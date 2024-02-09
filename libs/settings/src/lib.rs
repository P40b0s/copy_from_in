mod io;
mod task;
mod validation_error;
use logger::error;
mod file_methods;
pub use file_methods::FileMethods;
use once_cell::sync::OnceCell;
use std::{borrow::Cow, fmt::{Display, Write}, path::{Path, PathBuf}, sync::{Arc, Mutex, RwLock}, time::Duration};
pub use io::{serialize, deserialize, Serializer};
use serde::{Serialize, Deserialize};
extern crate toml;
extern crate blake2;
mod dates;
mod settings;
pub use settings::Settings;
pub use task::{Task, Filter, CopyModifier};
pub use validation_error::ValidationError;
pub use dates::*;
use hashbrown::hash_map::HashMap;
pub static EXCLUDES: OnceCell<Mutex<hashbrown::hash_map::HashMap<String, Vec<String>>>> = OnceCell::new();





// #[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
// //#[serde(rename_all = "camelCase")]
// pub struct MedoSettings
// {
//     #[serde(default="default_api_port")]
//     pub api_port: String,
//     #[serde(default="default_websocket_port")]
//     pub websocket_port: String,
//     pub paths: Paths,
//     #[serde(default="is_default")]
//     pub is_default: bool,
//     #[serde(default="default_organs")]
//     pub organs: Vec<Organ>,
//     pub filters: Vec<filter::Filter>,
//     ///Период сканирования директорий в секундах <br>
//     /// по умолчанию установлено на 2 минуты
//     #[serde(default="default_scan_interval")]
//     pub scan_interval: u32,
//     #[serde(default="default_date_format")]
//     pub date_format: String,
//     #[serde(default="default_time_format")]
//     pub time_format: String
// }
// impl MedoSettings
// {
//     pub fn update(&mut self, settings: MedoSettings)
//     {
//         self.paths = settings.paths;
//         self.is_default = false;
//         self.organs = settings.organs;
//         self.filters = settings.filters;
//     }
// }



#[cfg(test)]
mod test
{
    use serde::Deserialize;
    use crate::{file_methods::FileMethods, Settings, EXCLUDES};

    #[test]
    fn test_serialize_medo()
    {
        let medo: Settings = Settings::default();
        medo.save(true, crate::io::Serializer::Toml);
    }

    #[test]
    fn test_deserialize_medo()
    {
        logger::StructLogger::initialize_logger();
        let settings = Settings::load(true, crate::io::Serializer::Toml).unwrap();
        Settings::add_to_exclude("TASK", &"5555555".to_owned());
        Settings::add_to_exclude("TASK", &"4555555".to_owned());
        Settings::add_to_exclude("TASK", &"3555555".to_owned());
        Settings::add_to_exclude("TASK", &"2555555".to_owned());
        logger::info!("{:?}", EXCLUDES.get().unwrap().lock().unwrap());
        Settings::save_exclude("TASK");
        //let adm_prez = settings.organs.iter().find(|s|s.internal_id == OrganInternalId::AdmPrez);
        //assert_eq!(adm_prez.unwrap().source_uid, String::from("0b21bba1-f44d-4216-b465-147665360c06"));
    }

    #[test]
    fn test_del_one_exclude()
    {
        logger::StructLogger::initialize_logger();
        let settings = Settings::load(true, crate::io::Serializer::Toml).unwrap();
        Settings::del_exclude(settings.tasks.first().as_ref().unwrap(), "38773995_1");
        //let adm_prez = settings.organs.iter().find(|s|s.internal_id == OrganInternalId::AdmPrez);
        //assert_eq!(adm_prez.unwrap().source_uid, String::from("0b21bba1-f44d-4216-b465-147665360c06"));
    }
}