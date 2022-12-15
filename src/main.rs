mod copy;
mod settings;
use std::{path::{PathBuf}, thread, time::Duration};
extern crate chrono;
use chrono::Local;
use settings::Settings;


const DATE_FORMAT_STR: &'static str = "%Y-%m-%d][%H:%M:%S";
fn main()
{
   if let Some(settings) = Settings::load_settings()
   {
        let mut except = copy::get_except_names(&settings);
        if except.is_some()
        {
            let mut except = except.as_mut().unwrap();
            loop 
            {
                run_process(&settings, &mut except);
                thread::sleep(Duration::from_millis(settings.timer));
            }
        }
        else
        {
            println!("Ошибка получения списка фафлов...");
            std::io::stdin().read_line(&mut String::new()).unwrap();
        }
   }
   else 
   {
        println!("Ошибка чтения настроек...");
        std::io::stdin().read_line(&mut String::new()).unwrap();
   }
}

fn run_process(settings: &Settings, except: &mut Vec<String>)
{
    if let Some(dirs) = copy::get_dirs(settings)
    {
        for d in dirs
        {
            if !except.contains(&d)
            {
                except.push(d.clone());
                let mut target: PathBuf = PathBuf::from(settings.out_dir.as_path());
                let mut source: PathBuf = PathBuf::from(settings.in_dir.as_path());
                target.push(d.as_str());
                source.push(d.as_str());
                let dt = Local::now();
                println!("{} Обнаружена директория {}, копирую в {}", dt.format(DATE_FORMAT_STR),  source.display(), target.display());
                copy::copy_recursively(source.as_path(), target.as_path());
            }
        }
    }
}