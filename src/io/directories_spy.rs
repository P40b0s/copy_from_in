use std::{self, path::{Path, PathBuf}, sync::Mutex, collections::HashMap, time::Duration};
use logger::{error, info, debug, warn};
use once_cell::sync::{OnceCell, Lazy};
use regex::Regex;
use tokio::{runtime::Runtime, task::JoinHandle};
use crate::{settings::{Settings, Task, CopyModifier}, app::app_state::{self, AppState, STATE}, io};

pub static EXCLUDES: OnceCell<Mutex<HashMap<String, Vec<String>>>> = OnceCell::new();
pub static REGEXES: Lazy<Mutex<HashMap<String, Regex>>> = Lazy::new(|| 
{
    let mut hm: HashMap<String, Regex> = HashMap::new();
    let tsk = Settings::initialize().unwrap();
    for t in &tsk.tasks
    {
        for r in &t.rules
        {
            let rx = ["(?i)", r].concat();
            hm.insert(r.clone(), Regex::new(&rx).unwrap());
        }
    }
    Mutex::new(hm)
});

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

    //Поток мы можем спавнить только в рантайме токио, поэтому вытащил эту функцию отдельно
    //Думаю тут же надо осуществлять и обработку найденых пакетов
    pub fn process_tasks()
    {
        let args = STATE.get().unwrap().lock().unwrap();
        let first_time = args.args.first_initialize.clone();
        drop(args);
            let t = tokio::spawn(async move
            {
                DirectoriesSpy::check_for_new_packets(move |thread, found|
                {
                    if !first_time
                    {
                        debug!("Сообщение от задачи `{}` -> найден новый пакет {}", &thread.task_name, found);
                        let source = Path::new(&thread.source_dir).join(&found);
                        let target = Path::new(&thread.target_dir).join(&found);
                        match thread.copy_modifier
                        {
                            CopyModifier::CopyAll =>
                            {
                                if io::io::path_is_exists(&target)
                                {
                                    warn!("Пакет {} уже существует по пути {} копирование пакета отменено",found, &target.display())
                                }
                                else 
                                {
                                    let _ = io::io::copy_recursively(&source, &target);
                                    info!("Пакет {} скопирован в {} в соответсвии с правилом {}", found, &target.display(), &thread.task_name);
                                }
                                
                            },
                            _ =>
                            {
                                if let Some(entry) = io::io::get_files(&source)
                                {
                                    for e in entry
                                    {
                                        if io::io::extension_is(&e, "xml")
                                        {
                                            if let Some(text) = io::io::read_file(&e.path())
                                            {
                                                for except in &thread.rules
                                                {
                                                    if let Some(rx) = REGEXES.lock().unwrap().get(except)
                                                    {
                                                        if thread.copy_modifier == CopyModifier::CopyExcept
                                                        {
                                                            if !rx.is_match(&text)
                                                            {
                                                                if io::io::path_is_exists(&target)
                                                                {
                                                                    warn!("Пакет {} уже существует по адресу {} копирование пакета отменено",found, target.display())
                                                                }
                                                                else 
                                                                {
                                                                    let _ = io::io::copy_recursively(&source, &target);
                                                                    info!("Пакет {} скопирован в {} в соответсвии с правилом {}", found, &target.display(), &thread.task_name);
                                                                }
                                                            }
                                                        }
                                                        if thread.copy_modifier == CopyModifier::CopyOnly
                                                        {
                                                            if rx.is_match(&text)
                                                            {
                                                                if io::io::path_is_exists(&target)
                                                                {
                                                                    warn!("Пакет {} уже существует по адресу {} копирование пакета отменено",found, target.display())
                                                                }
                                                                else 
                                                                {
                                                                    let _ = io::io::copy_recursively(&source, &target);
                                                                    info!("Пакет {} скопирован в {} в соответсвии с правилом {}", found, &target.display(), &thread.task_name);
                                                                }
                                                            }
                                                        } 
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            },
                        }
                    }
                });
            });
        //}
    }
    ///Обработка осуществляется параллельно, но операция ожидается, поэтому все это надо запускать в процессе
    pub fn check_for_new_packets<F : Send + Sync + 'static>(callback: F) where F: Fn(&Task, &String)
    {
        let guard = STATE.get().unwrap().lock().unwrap();
        let tasks = guard.settings.tasks.clone();
        let _ = drop(guard);
        rayon::scope(|scope|
        {
            for t in &tasks
            {
                scope.spawn(|_task|
                {
                    loop 
                    {
                        let start = std::time::SystemTime::now();
                        let paths = DirectoriesSpy::get_dirs(&t.source_dir);
                        if paths.is_none()
                        {
                            break;
                        }
                        let mut is_change = false;
                        if let Some(reader) = paths.as_ref()
                        {
                            for d in reader
                            {
                                if DirectoriesSpy::add(&t.task_name, d)
                                {
                                    is_change = true;
                                    callback(&t.clone(), d);
                                }    
                            }
                            if is_change
                            {
                                DirectoriesSpy::serialize_exclude(&t.task_name);
                            }
                        }
                        let delay = std::time::Duration::from_millis(t.timer);
                        let end = std::time::SystemTime::now();
                        let duration = end.duration_since(start).unwrap();
                        logger::warn!("Задача {} была завершена за {}c", &t.task_name, duration.as_secs());
                        std::thread::sleep(delay.to_owned());
                    }
                })
            }
        });
    }


    fn get(thread_name: &str) -> Option<Vec<String>>
    {
        let hm = EXCLUDES.get().unwrap().lock().unwrap();
        let ex = hm.get(thread_name);
        ex.cloned()
    }
    fn add(thread_name: &str, dir: &String) -> bool
    {
        let mut guard = EXCLUDES.get().unwrap().lock().unwrap();
        if !guard.contains_key(thread_name)
        {
            guard.insert(thread_name.to_owned(), vec![dir.to_owned()]);
            return true;
        }
        else 
        {
            if let Some(ex) = guard.get_mut(thread_name)
            {
                let d = dir.to_owned();
                if !ex.contains(&d)
                {
                    ex.push(dir.to_owned());
                    return true;
                }
                else 
                {
                    return false;
                }
            }
        }
        return false;
    }
    fn serialize_exclude(task_name: &str,)
    {
        let concat_path = [task_name, ".txt"].concat();
        let file_name = Path::new(&concat_path);
        let list = EXCLUDES.get().unwrap().lock().unwrap();
        if let Some(vec) = list.get(task_name)
        {
            crate::io::serialize(vec, file_name, None);
        }  
    }
    pub fn deserialize_exclude(task: &Task)
    {
        let excl = EXCLUDES.get_or_init(|| Mutex::new(HashMap::new()));
        if !excl.lock().unwrap().contains_key(task.task_name.as_str())
        {
            let file = [&task.task_name, ".txt"].concat();
            let path = Path::new(&file);
            let ex = crate::io::deserialize::<Vec<String>>(&path);
            excl.lock().unwrap().insert(task.task_name.clone(), ex.1);
        }
    }
    
}

macro_rules! regex 
{
    ($re:literal $(,)?) => 
    {{
        static RE: once_cell::sync::OnceCell<regex::Regex> = once_cell::sync::OnceCell::new();
        RE.get_or_init(|| regex::Regex::new($re).unwrap())
    }};
}