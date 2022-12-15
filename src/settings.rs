use std::{fs, path::PathBuf};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Settings
{
    pub in_dir : PathBuf,
    pub out_dir : PathBuf,
    pub timer : u64
}

impl Settings
{
    pub fn load_settings() -> Option<Settings>
    {
        let mut file = std::env::current_dir().expect("Невозможно определить текущую директорию!");
        file.push("settings.json");
        let contents = match fs::read_to_string(&file) 
        {
            Ok(c) => c,
            Err(_) => {
                eprintln!("Немогу открыть файл `{}`", &file.display());
                return None;
            }
        };
        let data: Settings = match serde_json::from_str(&contents) 
        {
            Ok(d) => d,
            Err(e) => 
            {
                eprintln!("Неверный формат файла `{}` {}", &file.display(), e);
                return None;
            }
        };
        if !data.in_dir.exists()
        {
            eprintln!("Ошибка, директории in_dir = `{}` не существует", data.in_dir.display());
            return None;
        }
        if !data.out_dir.exists()
        {
            eprintln!("Ошибка, директории out_dir = `{}` не существует", data.out_dir.display());
            return None;
        }
       Some(data)
    }


}

#[test]
fn testload()
{
    if let Some(s) = Settings::load_settings()
    {
        assert_eq!(s.in_dir.display().to_string(), "in".to_owned());
        assert_eq!(s.out_dir.display().to_string(), "out".to_owned());
    }
}