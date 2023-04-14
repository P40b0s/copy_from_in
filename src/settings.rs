use std::{fs, path::PathBuf, sync::Mutex};
use once_cell::sync::{OnceCell, Lazy};
use serde::{Serialize, Deserialize};
use logger::{error};

//pub static SETTINGS: OnceCell<Mutex<Settings>> = OnceCell::new();
#[derive(Serialize, Deserialize, Clone)]
pub struct Task
{
    pub source_dir : PathBuf,
    pub target_dir: PathBuf,
    pub timer : u64,
    pub thread_name: String
}
#[derive(Serialize, Deserialize, Clone)]
pub struct Settings
{
    pub tasks: Vec<Task>
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
        }
       Some(data)
    }
}

#[test]
fn testload()
{
    if let Some(s) = Settings::initialize()
    {
        assert_eq!(s.tasks.iter().nth(0).unwrap().thread_name, "architector_thread".to_owned());
    }
}