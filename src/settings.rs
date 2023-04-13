use std::{fs, path::PathBuf, sync::Mutex};
use once_cell::sync::{OnceCell, Lazy};
use serde::{Serialize, Deserialize};
use logger::{error};

//pub static SETTINGS: OnceCell<Mutex<Settings>> = OnceCell::new();

#[derive(Serialize, Deserialize, Clone)]
pub struct Settings
{
    pub medo_compliete_in_dir : PathBuf,
    pub medo_in_dir: PathBuf,
    pub architector_in_dir : PathBuf,
    pub timer : u64,
    pub except_dirs_file: String

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
        if !data.in_dir.exists()
        {
            error!("Ошибка, директории in_dir = `{}` не существует", data.in_dir.display());
            return None;
        }
        if !data.out_dir.exists()
        {
            error!("Ошибка, директории out_dir = `{}` не существует", data.out_dir.display());
            return None;
        }
       Some(data)
    }


}

#[test]
fn testload()
{
    if let Some(s) = Settings::initialize()
    {
        assert_eq!(s.in_dir.display().to_string(), "in".to_owned());
        assert_eq!(s.out_dir.display().to_string(), "out".to_owned());
    }
}