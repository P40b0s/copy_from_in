mod io;
mod task;
mod validation_error;
mod file_methods;
pub use file_methods::FileMethods;
use once_cell::sync::OnceCell;
use std::sync::Mutex;
pub use io::{serialize, deserialize, Serializer};
extern crate toml;
extern crate blake2;
mod settings;
pub use settings::Settings;
pub use task::{Task, Filter, CopyModifier};
pub use validation_error::ValidationError;
//pub static EXCLUDES: OnceCell<Mutex<hashbrown::hash_map::HashMap<String, Vec<String>>>> = OnceCell::new();





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
    use crate::{file_methods::FileMethods, CopyModifier, Settings};

    #[test]
    fn test_serialize_medo()
    {
        let medo: Settings = Settings::default();
        medo.save(crate::io::Serializer::Toml);
    }
    #[test]
    fn test_from_toml_to_json()
    {
        logger::StructLogger::new_default();
        let settings = Settings::load(crate::io::Serializer::Toml).unwrap();
        settings.save(crate::io::Serializer::Json);
    }
    #[test]
    fn test_from_json_to_toml()
    {
        logger::StructLogger::new_default();
        let settings = Settings::load(crate::io::Serializer::Json).unwrap();
        settings.save(crate::io::Serializer::Toml);
    }
    #[test]
    fn test_serialize_settings_json()
    {
        let settings: Settings = Settings::default();
        settings.save(crate::io::Serializer::Json);
    }
    #[test]
    fn test_deserialize_settings_json()
    {
        logger::StructLogger::new_default();
        let settings = Settings::load(crate::io::Serializer::Json).unwrap();
        assert_eq!(settings.tasks[0].copy_modifier, CopyModifier::CopyAll);
        assert_eq!(settings.tasks[1].copy_modifier, CopyModifier::CopyOnly);
    }
    #[test]
    fn test_deserialize_settings_toml()
    {
        logger::StructLogger::new_default();
        let settings = Settings::load(crate::io::Serializer::Toml).unwrap();
        assert_eq!(settings.tasks[0].copy_modifier, CopyModifier::CopyAll);
        assert_eq!(settings.tasks[1].copy_modifier, CopyModifier::CopyOnly);
    }
}