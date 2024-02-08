mod io;
use logger::error;
mod file_methods;
pub use file_methods::FileMethods;
use std::{borrow::Cow, collections::HashMap, fmt::{Display, Write}, path::{Path, PathBuf}, sync::{Arc, Mutex, RwLock}, time::Duration};
use io::{serialize, deserialize};
use serde::{Serialize, Deserialize};
extern crate toml;
extern crate blake2;
mod dates;
pub use dates::*;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
//#[serde(rename_all = "camelCase")]
pub struct Task
{
    pub name: String,
    #[serde(default="def_dirs")]
    pub source_dir: PathBuf,
    #[serde(default="def_dirs")]
    pub target_dir: PathBuf,
    #[serde(default="def_timer")]
    pub timer: u64,
    #[serde(default="is_default")]
    pub delete_after_copy: bool,
    #[serde(default="def_copy_mod")]
    #[serde(deserialize_with="deserialize_copy_modifier")]
    pub copy_modifier: CopyModifier,
    #[serde(default="is_default")]
    pub is_active: bool,
    pub filters: Filter
    
}
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
//#[serde(rename_all = "camelCase")]
pub struct Filter
{
    #[serde(default="empty_doc_types")]
    pub document_types: Vec<String>,
    #[serde(default="empty_doc_types")]
    pub document_uids: Vec<String>
}
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
//#[serde(rename_all = "camelCase")]
pub struct ValidationError
{
    pub field_name: Option<String>,
    pub error: String
}
impl ValidationError
{
    pub fn new(field_name: Option<String>, error: String)-> Self
    {
        Self 
        { 
            field_name, 
            error 
        }
    }
    pub fn new_from_str(field_name: Option<String>, error: &str)-> Self
    {
        Self 
        { 
            field_name, 
            error: error.to_owned()
        }
    }
}
impl Display for ValidationError
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result 
    {
        if let Some(field) = self.field_name.as_ref()
        {   let msg = ["Ошибка настроек в поле ", field, " -> ", &self.error].concat();
            f.write_str(&msg)
        }   
        else
        {
            f.write_str(&self.error)
        }
    }
}
fn def_timer() -> u64
{
    200000
}
fn def_copy_mod() -> CopyModifier
{
    CopyModifier::CopyAll
}
fn empty_doc_types() -> Vec<String>
{
    Vec::with_capacity(0)
}
fn def_dirs() -> PathBuf
{
    PathBuf::from("---")
}

impl Default for Task
{
    fn default() -> Self 
    {
        Task 
        {
            source_dir: PathBuf::from("in"),
            target_dir: PathBuf::from("out"),
            timer: 20000,
            name: "default_task".to_owned(),
            copy_modifier: CopyModifier::CopyAll,
            delete_after_copy: false,
            is_active: false,
            filters: Filter
            {
                document_types: vec![],
                document_uids: vec![]
            }
            
        }
    }
}

impl Task
{
    pub fn get_task_name(&self) -> &str
    {
        &self.name
    }
    pub fn get_source_dir(&self) -> &PathBuf
    {
        &self.source_dir
    }
    pub fn get_target_dir(&self) -> &PathBuf
    {
        &self.target_dir
    }
    pub fn get_task_delay(&self) -> Duration
    {
        std::time::Duration::from_millis(self.timer)
    }
}


#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub enum CopyModifier
{
    CopyAll,
    CopyOnly,
    CopyExcept
}
impl Display for CopyModifier
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result 
    {
        write!(f, "{}", match self 
        {
            CopyModifier::CopyAll => "copy_all",
            CopyModifier::CopyOnly => "copy_only",
            CopyModifier::CopyExcept => "copy_except"
        })
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Settings
{
    pub tasks: Vec<Task>
}
impl Default for Settings
{
    fn default() -> Self 
    {
        Settings 
        { 
            tasks: vec![Task::default()],
        }
    }
}
impl FileMethods for Settings
{
    const FILE_NAME: &'static str = "settings.toml";
    const FILE_PATH: &'static str = "";
    fn validate(&self) -> Result<(), Vec<ValidationError>>
    {
        let mut errors: Vec<ValidationError> = vec![];
        for task in &self.tasks
        {
            //Проверяем директории только если есть активный фильтр
            if task.is_active
            {
                if let Ok(e) = task.source_dir.try_exists()
                {
                    if !e
                    {
                        let err = ["Директория", &task.source_dir.to_str().unwrap_or("***"), " в задаче ", &task.name, " не существует!"].concat();
                        errors.push(ValidationError::new(Some("source_directory".to_owned()), err));
                    }
                }
                if let Ok(e) = task.target_dir.try_exists()
                {
                    if !e
                    {
                        let err = ["Директория ", &task.target_dir.to_str().unwrap_or("***"), " в задаче ", &task.name, " не существует!"].concat();
                        errors.push(ValidationError::new(Some("target_directory".to_owned()), err));
                    }
                }
                if task.copy_modifier != CopyModifier::CopyAll
                && task.filters.document_types.len() == 0
                && task.filters.document_uids.len() == 0
                {
                    let err = ["Для копирования выбран модификатор ", &task.copy_modifier.to_string(), " но не определены фильтры, для данного модификатора необходимо добавить хоть один фильтр"].concat();
                    errors.push(ValidationError::new(Some("filters".to_owned()), err));
                }
            }
        }
        if errors.len() > 0
        {
            Err(errors)
        }
        else 
        {
            Ok(())
        }
    }
}

// impl Settings
// {
//     pub fn validate(&self) -> Vec<(String, String)>
//     {
//         let mut errors :Vec<(String, String)> = vec![];
//         for task in &self.tasks
//         {
//             //Проверяем директории только если есть активный фильтр
//             if task.is_active
//             {
//                 if let Ok(e) = task.source_dir.try_exists()
//                 {
//                     if !e
//                     {
//                         let err = ["Директория", &task.source_dir.to_str().unwrap_or("***"), " в задаче ", &task.name, " не существует!"].concat();
//                         errors.push(("source_directory".to_owned(), err));
//                     }
//                 }
//                 if let Ok(e) = task.target_dir.try_exists()
//                 {
//                     if !e
//                     {
//                         let err = ["Директория ", &task.target_dir.to_str().unwrap_or("***"), " в задаче ", &task.name, " не существует!"].concat();
//                         errors.push(("target_directory".to_owned(), err));
//                     }
//                 }
//                 if task.copy_modifier != CopyModifier::CopyAll
//                 && task.document_types.len() == 0
//                 && task.document_uids.len() == 0
//                 {
//                     let err = ["Для копирования выбран модификатор ", &task.copy_modifier.to_string(), " но не определены фильтры, для данного модификатора необходимо добавить хоть один фильтр"].concat();
//                     errors.push(("filters".to_owned(), err));
//                 }
//             }
//         }
//         errors
//     }
// }

fn deserialize_copy_modifier<'de, D>(deserializer: D) -> Result<CopyModifier, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    let s: String = serde::de::Deserialize::deserialize(deserializer)?;
    match s.as_str()
    {
        "copy_only" => Ok(CopyModifier::CopyOnly),
        "copy_all" => Ok(CopyModifier::CopyAll),
        "copy_except" => Ok(CopyModifier::CopyExcept),
        _ => Err(serde::de::Error::custom("Модификатор может быть только: copy_only, copy_all, copy_except"))
    }
}

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


fn is_default() -> bool
{
    false
}
#[cfg(test)]
mod test
{
    use serde::Deserialize;
    use crate::{Settings,  file_methods::FileMethods};

    #[test]
    fn test_serialize_medo()
    {
        let medo: Settings = Settings::default();
        medo.save();
    }

    #[test]
    fn test_deserialize_medo()
    {
        let settings = Settings::load();
        println!("{:?}", settings.err().unwrap())
        //let adm_prez = settings.organs.iter().find(|s|s.internal_id == OrganInternalId::AdmPrez);
        //assert_eq!(adm_prez.unwrap().source_uid, String::from("0b21bba1-f44d-4216-b465-147665360c06"));
    }
}