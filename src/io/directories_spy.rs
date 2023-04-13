use std::{self, path::{Path, PathBuf}, sync::Mutex};
use logger::{error, info};
use once_cell::sync::OnceCell;
use tokio::runtime::Runtime;

use crate::{settings::Settings, app::app_state::{self, AppState}, Excludes};

pub struct DirectoriesSpy;

impl DirectoriesSpy
{
    fn get_dirs(path: &PathBuf) -> Option<Vec<String>>
    {
        let paths = std::fs::read_dir(path);
        if paths.is_err()
        {
            error!("😳 Ошибка чтения директории {} - {}", path.display(), paths.err().unwrap());
            return None;
        }
        let mut dirs = vec![];
        for d in paths.unwrap()
        {
            let dir = d.unwrap().file_name().to_str().unwrap().to_owned();
            dirs.push(dir);
        }
        return Some(dirs);
    }
    fn get_runtime(th_name: &str) -> Option<Runtime>
    {
        let mut dir_searcher_runtime = tokio::runtime::Builder::new_multi_thread()
        .thread_name(th_name)
        .enable_all()
        .worker_threads(1)
        .build();
        if let Ok(r) = dir_searcher_runtime
        {
            return Some(r);
        }
        else 
        {
            error!("{}", dir_searcher_runtime.err().unwrap());
            return None;    
        }
        
    }
    //TODO сделать инициализацию для первого запуска
    pub fn check_for_new_packets<F : Send + Sync>(path: &Path, th_name: &str, f: F) where F: Fn(String)
    {
        let pb = path.to_path_buf();
        let paths = DirectoriesSpy::get_dirs(&pb);
        if paths.is_none()
        {
            return;
        }
        if let Some(runtime) = DirectoriesSpy::get_runtime(th_name)
        {
            runtime.block_on(
            tokio::spawn(async 
            {
                let time = AppState::get_settings().timer;
                let interval = tokio::time::interval(std::time::Duration::from_millis(time));
                loop 
                {
                    let mut dirs = vec![];
                    let mut is_change = false;
                    if let Some(reader) = paths
                    {
                        for d in reader
                        {
                            if !Excludes::in_excludes(&d)
                            {
                                is_change = true;
                                Excludes::add(&d);
                                info!("Процесс {} нашел новый пакет {}", th_name, &d);
                                f(d);
                                dirs.push(d);
                            }    
                        }
                        if is_change
                        {
                            Excludes::serialize();
                        }
                    }
                    interval.tick().await;
                }
            }));
        }
    }
   
    // pub fn get_except_names() -> Option<Vec<String>>
    // {
    //     let path = app_state::get_settings();
        
    //     if let Some(dirs) = get_dirs(settings)
    //     {
    //         if let Some(settings) = Settings::load_settings()
    //         {
    //             let mut except = copy::get_except_names(&settings);
    //             if except.is_some()
    //             {
    //                 let mut except = except.as_mut().unwrap();
    //                 loop 
    //                 {
    //                     run_process(&settings, &mut except);
    //                     thread::sleep(Duration::from_millis(settings.timer));
    //                 }
    //             }
    //             else
    //             {
    //                 println!("Ошибка получения списка файлов...");
    //                 std::io::stdin().read_line(&mut String::new()).unwrap();
    //             }
    //         }
    //         else 
    //         {
    //                 println!("Ошибка чтения настроек...");
    //                 std::io::stdin().read_line(&mut String::new()).unwrap();
    //         }
    //                     return  Some(dirs);
    //     }
    //     else {
    //         return None;
    //     }
    // }
}


// pub fn get_except_names(settings: &Settings) -> Option<Vec<String>>
// {
//     if let Some(dirs) = get_dirs(settings)
//     {
//         // if std::fs::write("except.dirs", dirs.join("\n")).is_err()
//         // {
//         //     eprintln!("😳 Немогу создать файл искоючений except.dirs!");
//         //     return None;
//         // }
//         return  Some(dirs);
//     }
//     else {
//         return None;
//     }
// }

// pub fn get_dirs(settings: &Settings) -> Option<Vec<String>>
// {
//     let paths = std::fs::read_dir(settings.in_dir.as_path());
//     if paths.is_err()
//     {
//         eprintln!("😳 Ошибка чтения директории {} - {}", settings.in_dir.display(), paths.err().unwrap());
//         return None;
//     }
//     let mut dirs = vec![];
//     for d in paths.unwrap()
//     {
//         let dir = d.unwrap().file_name().to_str().unwrap().to_owned();
//         dirs.push(dir);
//     }
//     return Some(dirs);
// }

// /// Copy files from source to destination recursively.
// pub fn copy_recursively(source: impl AsRef<Path>, destination: impl AsRef<Path>) -> std::io::Result<()> 
// {
//     std::fs::create_dir_all(&destination)?;
//     for entry in std::fs::read_dir(source)? 
//     {
//         let entry = entry?;
//         let filetype = entry.file_type()?;
//         if filetype.is_dir() 
//         {
//             copy_recursively(entry.path(), destination.as_ref().join(entry.file_name()))?;
//         } 
//         else 
//         {
//             std::fs::copy(entry.path(), destination.as_ref().join(entry.file_name()))?;
//         }
//     }
//     Ok(())
// }
