use std::{self, path::{Path, PathBuf}, sync::Mutex, collections::HashMap};
use logger::{error, info};
use once_cell::sync::OnceCell;
use tokio::runtime::Runtime;
use crate::{settings::{Settings, Task}, app::app_state::{self, AppState}};

pub static EXCLUDES: OnceCell<Mutex<HashMap<String, Vec<String>>>> = OnceCell::new();

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
    pub fn check_for_new_packets<F : Send + Sync + 'static>(task: &Task, f: F) where F: Fn(&String)
    {
        let paths = DirectoriesSpy::get_dirs(&task.source_dir);
        let task = task.clone();
        DirectoriesSpy::deserialize_exclude(&task);
        if paths.is_none()
        {
            return;
        }
        //let handle = Handle::current();

        // tokio::task::spawn_blocking(|| 
        // {
        //     inner_example(handle);
        // })
        // .await
        // .expect("Blocking task panicked");
        if let Some(runtime) = DirectoriesSpy::get_runtime(&task.thread_name)
        {
            let rt = runtime.block_on(
            async move
            {
                let mut interval = tokio::time::interval(std::time::Duration::from_millis(task.timer));
                loop 
                {
                    let mut dirs = vec![];
                    let mut is_change = false;
                    if let Some(reader) = paths.as_ref()
                    {
                        for d in reader
                        {
                            if DirectoriesSpy::add(&task.thread_name, d)
                            {
                                is_change = true;
                                info!("Процесс {} нашел новый пакет {}", &task.thread_name, d);
                                f(d);
                                dirs.push(d.to_owned());
                            }    
                        }
                        if is_change
                        {
                            DirectoriesSpy::serialize_exclude(&task.thread_name);
                        }
                    }
                    interval.tick().await;
                }
            });

        }
    }


    fn get(thread_name: &str) -> Option<Vec<String>>
    {
        let hm = EXCLUDES.get().unwrap().lock().unwrap();
        let ex = hm.get(thread_name);
        ex.cloned()
    }
    fn add(thread_name: &str, dir: &String) -> bool
    {
        if !EXCLUDES.get().unwrap().lock().unwrap().contains_key(thread_name)
        {
            EXCLUDES.get().unwrap().lock().unwrap().insert(thread_name.to_owned(), vec![dir.to_owned()]);
            return true;
        }
        else 
        {
            if let Some(ex) = EXCLUDES.get().unwrap().lock().unwrap().get_mut(thread_name)
            {
                ex.push(dir.to_owned());
                return true;
            }
        }
        return false;
    }
    fn serialize_exclude(thread_name: &str,)
    {
        let concat_path = [thread_name, ".txt"].concat();
        let file_name = Path::new(&concat_path);
        let list = EXCLUDES.get().unwrap().lock().unwrap();
        if let Some(vec) = list.get(thread_name)
        {
            crate::io::serialize(vec, file_name, None);
        }

        
    }
    fn deserialize_exclude(task: &Task)
    {
        if let Some(excludes) = EXCLUDES.get()
        {
            if !EXCLUDES.get().unwrap().lock().unwrap().contains_key(task.thread_name.as_str())
            {
                let path = Path::new(&task.thread_name).join(".txt");
                let ex = crate::io::deserialize::<Vec<String>>(&path);
                EXCLUDES.get().unwrap().lock().unwrap().insert(task.thread_name.clone(), ex.1);
            }
        }
        else 
        {
            EXCLUDES.set(Mutex::new(HashMap::new()));
        }
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
