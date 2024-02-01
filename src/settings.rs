use std::{fs, path::PathBuf, sync::Mutex, fmt::Display};
use once_cell::sync::{OnceCell, Lazy};
use serde::{Serialize, Deserialize};
use logger::{error, warn};
use crate::{app::STATE, DirectoriesSpy};

//pub static SETTINGS: OnceCell<Mutex<Settings>> = OnceCell::new();
#[derive(Serialize, Deserialize, Clone)]
pub struct Task
{
    pub source_dir : PathBuf,
    pub target_dir: PathBuf,
    pub timer : u64,
    pub task_name: String,
    #[serde(deserialize_with="deserialize_copy_modifier")]
    pub copy_modifier: CopyModifier,
    pub rules: Vec<String>
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
            task_name: "default_thread".to_owned(),
            copy_modifier: CopyModifier::CopyAll,
            rules: vec![] 
        }
    }
}




#[derive(Serialize, Deserialize, Clone, PartialEq)]
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
        Settings { tasks: vec![Task::default()] }
    }
}

impl Settings
{
    pub fn initialize() -> Option<Settings>
    {
        let mut file = std::env::current_dir().expect("Невозможно определить текущую директорию!");
        file.push("settings.json");
        // if STATE.get().unwrap().lock().unwrap().args.default_settings
        // {
        //     warn!("Cоздан файл `{}` с настройками по умолчанию, до следующего запуска программы файл необходимо донастроить, программа будет завершена.", &file.display());
        //     let def = Settings::default();
        //     crate::io::serialize(def, &file, None);
        //     return None;
        // }
        let contents = match fs::read_to_string(&file) 
        {
            Ok(c) => c,
            Err(_) => {
                error!("Немогу открыть файл `{}`", &file.display());
                error!("Файл настроек `{}` не найден, будет создан файл с настройками по умолчанию, до следующего запуска программы файл необходимо донастроить, программа будет завершена.", &file.display());
                let def = Settings::default();
                crate::io::serialize(def, &file, None);
                return None;
            }
        };
        let data: Settings = match serde_json::from_str(&contents) 
        {
            Ok(d) => d,
            Err(e) => 
            {
                error!("Неверный формат файла `{}` {}", &file.display(), e);
                return None;
            }
        };
        for t in &data.tasks
        {
            if !t.source_dir.exists()
            {
                error!("Ошибка, директории `{}` не существует", t.source_dir.display());
                return None;
            }
            if !t.target_dir.exists()
            {
                error!("Ошибка, директории `{}` не существует", t.target_dir.display());
                return None;
            }
            crate::io::DirectoriesSpy::deserialize_exclude(t);
        }
       Some(data)
    }
}


fn deserialize_copy_modifier<'de, D>(deserializer: D) -> Result<CopyModifier, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    let s: &str = serde::de::Deserialize::deserialize(deserializer)?;
    match s 
    {
        "copy_only" => Ok(CopyModifier::CopyOnly),
        "copy_all" => Ok(CopyModifier::CopyAll),
        "copy_except" => Ok(CopyModifier::CopyExcept),
        _ => Err(serde::de::Error::custom("Модификатор может быть только: copy_only, copy_all, copy_except"))
    }
}


#[test]
fn testload()
{
    if let Some(s) = Settings::initialize()
    {
        assert_eq!(s.tasks.iter().nth(0).unwrap().task_name, "architector_thread".to_owned());
    }
}