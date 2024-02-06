use std::{fmt::Display, fs, path::PathBuf, time::Duration};
use serde::{Serialize, Deserialize};
use logger::error;

#[derive(Serialize, Deserialize, Clone)]
pub struct Task
{
    pub source_dir : PathBuf,
    pub target_dir: PathBuf,
    pub timer : u64,
    pub task_name: String,
    #[serde(deserialize_with="deserialize_copy_modifier")]
    pub copy_modifier: CopyModifier,
    ///Копировать только документы
    pub only_docs: bool,
    pub delete_after_copy: bool,
    #[serde(default="empty_rules")]
    pub rules: Vec<String>
}
fn empty_rules() -> Vec<String>
{
    Vec::with_capacity(0)
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
           //""source +\\w+:uid=\"b7eaa52a-ea8e-4f6d-9b2b-c774c58f31e5\"","
            only_docs: true,
            delete_after_copy: false,
            rules: vec![] 
        }
    }
}

impl Task
{
    pub fn get_task_name(&self) -> &str
    {
        &self.task_name
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
    pub tasks: Vec<Task>,
    pub doc_types: Vec<String>
}
impl Default for Settings
{
    fn default() -> Self 
    {
        Settings 
        { 
            tasks: vec![Task::default()],
            doc_types: vec!["Транспортный\\s+пакет".to_owned(), "Документ".to_owned()]
        }
    }
}

impl Settings
{
    pub fn initialize() -> Option<Settings>
    {
        let mut file = std::env::current_dir().expect("Невозможно определить текущую директорию!");
        file.push("settings.json");
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

#[test]
fn test_load_settings()
{
    logger::StructLogger::initialize_logger();
    if let Some(s) = Settings::initialize()
    {
        println!("{:?}", s.doc_types);
    }
    
}