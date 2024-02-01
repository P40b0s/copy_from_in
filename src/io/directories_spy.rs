use std::{self, path::{Path, PathBuf}, sync::Mutex, collections::HashMap};
use logger::{debug, error, info, warn};
use once_cell::sync::{OnceCell, Lazy};
use regex::Regex;
use crate::{settings::{Settings, Task, CopyModifier}, app::app_state::STATE, io};
use crossbeam_channel::bounded;
pub static EXCLUDES: OnceCell<Mutex<HashMap<String, Vec<String>>>> = OnceCell::new();
pub static REGEXES: Lazy<Mutex<HashMap<String, Regex>>> = Lazy::new(|| 
{
    let mut hm: HashMap<String, Regex> = HashMap::new();
    let guard = STATE.get().unwrap().lock().unwrap();
    for t in &guard.settings.tasks
    {
        for r in &t.rules
        {
            let rx = ["(?i)", r].concat();
            hm.insert(r.clone(), Regex::new(&rx).unwrap());
        }
    }
    Mutex::new(hm)
});
pub static ONLY_DOC_REGEX: Lazy<Regex> = Lazy::new(|| 
    {
        //let tsk = Settings::initialize().unwrap();
        let guard = STATE.get().unwrap().lock().unwrap();
        let doc_types = &guard.settings.doc_types;
        let mut rx = "[a-z]+:type=\"(".to_owned();
        let end_rx = ")\"";
        for (i, dt) in doc_types.iter().enumerate()
        {
            if i == doc_types.len() - 1
            {
                let type_pattern = ["(",dt, ")"].concat();
                rx.push_str(&type_pattern);
            }
            else 
            {
                let type_pattern = ["(",dt, ")|"].concat();
                rx.push_str(&type_pattern);
            }
        }
        rx.push_str(&end_rx);
        Regex::new(&rx).unwrap()
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
    // fn get_runtime(th_name: &str) -> Option<Runtime>
    // {
    //     let mut dir_searcher_runtime = tokio::runtime::Builder::new_multi_thread()
    //     .thread_name(th_name)
    //     .enable_all()
    //     .worker_threads(1)
    //     .build();
    //     if let Ok(r) = dir_searcher_runtime
    //     {
    //         return Some(r);
    //     }
    //     else 
    //     {
    //         error!("{}", dir_searcher_runtime.err().unwrap());
    //         return None;    
    //     }
    // }

    ///Возвырат сообщений из канала реализован в главном потоке, управление в main не возвращается, 
    ///так как главный поток больше ни для чего не используется, оставлю так
    pub fn process_tasks()
    {
        let args = STATE.get().unwrap().lock().unwrap();
        let first_time = args.args.first_initialize.clone();
        drop(args);
        let (s, r) = bounded::<(Task, String)>(10);
        DirectoriesSpy::check_for_new_packets_spawn(s);
        loop 
        {
            while let Ok(rec) = r.recv() 
            {
                Self::copy_files_process(first_time, rec.0, rec.1);
            }
        }
    }

    // fn check_regex<F : Send + Sync + 'static>(callback: F) where F: Fn(&Task, &String)
    // {

    // }

    fn copy_files_process(first_time: bool, task: Task, founded_packet_name : String)
    {
        let task_name = task.get_task_name();
        let source_dir = task.get_source_dir();
        let target_dir = task.get_target_dir();
        let dir_name = &founded_packet_name;
        if !first_time
        {
            let source = Path::new(source_dir).join(dir_name);
            let target = Path::new(target_dir).join(dir_name);
            debug!("Сообщение от задачи `{}` -> найден новый пакет {}", task_name, source.display());
            let mut ret = false;
            // if task.only_docs
            // {
            //     if let Some(entry) = io::io::get_files(&source)
            //     {
            //         for e in entry
            //         {
            //             if io::io::extension_is(&e, "xml")
            //             {
            //                 if let Some(text) = io::io::read_file(&e.path())
            //                 {
            //                     if task.copy_modifier == CopyModifier::CopyOnly || task.copy_modifier == CopyModifier::CopyExcept
            //                     {
            //                         for rule in &task.rules
            //                         {
            //                             if let Some(rx) = REGEXES.lock().unwrap().get(rule)
            //                             {

            //                             }
            //                         }
            //                     }
            //                 }
            //             }
            //         }
            //     }
            // }


            match task.copy_modifier
            {
                CopyModifier::CopyAll =>
                {
                    if io::io::path_is_exists(&target)
                    {
                        warn!("Пакет {} уже существует по пути {} копирование пакета отменено",dir_name, &target.display());
                    }
                    else 
                    {
                        if task.only_docs
                        {
                            if let Some(entry) = io::io::get_files(&source)
                            {
                                for e in entry
                                {
                                    if io::io::extension_is(&e, "xml")
                                    {
                                        if let Some(text) = io::io::read_file(&e.path())
                                        {
                                            if ONLY_DOC_REGEX.is_match(&text)
                                            {
                                                if let Ok(copy_time) = io::io::copy_recursively(&source, &target)
                                                {
                                                    info!("Задачей `{}` пакет {} скопирован в {} за {}с. в соответсвии с правилом {}",task_name, dir_name, target_dir.display(), copy_time, &task.copy_modifier);
                                                    ret = true;
                                                }
                                                else
                                                {
                                                    error!("Ошибка копирования пакета {} в {} для задачи {}",dir_name, target_dir.display(), task_name);
                                                }
                                            }
                                        }
                                    }
                                    if io::io::extension_is(&e, "rc")
                                    {
                                        if let Ok(copy_time) = io::io::copy_recursively(&source, &target)
                                        {
                                            info!("Задачей `{}` пакет {} скопирован в {} за {}с. в соответсвии с правилом {}",task_name, dir_name, target_dir.display(), copy_time, &task.copy_modifier);
                                            ret = true;
                                        }
                                        else
                                        {
                                            error!("Ошибка копирования пакета {} в {} для задачи {}",dir_name, target_dir.display(), task_name);
                                        }
                                    }
                                }
                            }
                        }
                        else
                        {
                            if let Ok(copy_time) = io::io::copy_recursively(&source, &target)
                            {
                                info!("Задачей `{}` пакет {} скопирован в {} за {}с. в соответсвии с правилом {}",task_name, dir_name, target_dir.display(), copy_time, &task.copy_modifier);
                                ret = true;
                            }
                            else
                            {
                                error!("Ошибка копирования пакета {} в {} для задачи {}",dir_name, target_dir.display(), task_name);
                            }
                        }
                    }
                },
                CopyModifier::CopyOnly =>
                {
                    if let Some(entry) = io::io::get_files(&source)
                    {
                        for e in entry
                        {
                            //info!("найден файл {}", &e.file_name().to_str().unwrap());
                            if io::io::extension_is(&e, "xml")
                            {
                                //info!("найден xml файл {}", &e.file_name().to_str().unwrap());
                                if let Some(text) = io::io::read_file(&e.path())
                                {
                                    //info!("xml {}", &text);
                                    for rule in &task.rules
                                    {
                                        if let Some(rx) = REGEXES.lock().unwrap().get(rule)
                                        {
                                            if task.only_docs
                                            {
                                                if rx.is_match(&text) && ONLY_DOC_REGEX.is_match(&text)
                                                {
                                                    if io::io::path_is_exists(&target)
                                                    {
                                                        warn!("Пакет {} уже существует по адресу {} копирование пакета отменено",dir_name, target_dir.display())
                                                    }
                                                    else 
                                                    {
                                                        if let Ok(copy_time) = io::io::copy_recursively(&source, &target)
                                                        {
                                                            info!("Задачей `{}` пакет {} скопирован в {} за {}с. в соответсвии с правилом {}",task_name, dir_name, target_dir.display(), copy_time, rule);
                                                            ret = true;
                                                        }
                                                        else
                                                        {
                                                            error!("Ошибка копирования пакета {} в {} для задачи {}",dir_name, target_dir.display(), task_name);
                                                        }
                                                    }
                                                }
                                            }
                                            else
                                            {
                                                if rx.is_match(&text)
                                                {
                                                    if io::io::path_is_exists(&target)
                                                    {
                                                        warn!("Пакет {} уже существует по адресу {} копирование пакета отменено",dir_name, target_dir.display())
                                                    }
                                                    else 
                                                    {
                                                        if let Ok(copy_time) = io::io::copy_recursively(&source, &target)
                                                        {
                                                            info!("Задачей `{}` пакет {} скопирован в {} за {}с. в соответсвии с правилом {}",task_name, dir_name, target_dir.display(), copy_time, rule);
                                                            ret = true;
                                                        }
                                                        else
                                                        {
                                                            error!("Ошибка копирования пакета {} в {} для задачи {}",dir_name, target_dir.display(), task_name);
                                                        }
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
                CopyModifier::CopyExcept =>
                {
                    if let Some(entry) = io::io::get_files(&source)
                    {
                        for e in entry
                        {
                            if io::io::extension_is(&e, "xml")
                            {
                                if let Some(text) = io::io::read_file(&e.path())
                                {
                                    for rule in &task.rules
                                    {
                                        if let Some(rx) = REGEXES.lock().unwrap().get(rule)
                                        {
                                            if task.only_docs
                                            {
                                                if !rx.is_match(&text) && ONLY_DOC_REGEX.is_match(&text)
                                                {
                                                    if io::io::path_is_exists(&target)
                                                    {
                                                        warn!("Пакет {} уже существует по адресу {} копирование пакета отменено",dir_name, target_dir.display())
                                                    }
                                                    else 
                                                    {
                                                        if let Ok(copy_time) = io::io::copy_recursively(&source, &target)
                                                        {
                                                            info!("Задачей `{}` пакет {} скопирован в {} за {}с. в соответсвии с правилом {}",task_name, dir_name, target_dir.display(), copy_time, rule);
                                                            ret = true;
                                                        }
                                                        else
                                                        {
                                                            error!("Ошибка копирования пакета {} в {} для задачи {}",dir_name, target_dir.display(), task_name);
                                                        }
                                                    }
                                                }
                                            }
                                            else
                                            {
                                                if !rx.is_match(&text)
                                                {
                                                    if io::io::path_is_exists(&target)
                                                    {
                                                        warn!("Пакет {} уже существует по адресу {} копирование пакета отменено",dir_name, target_dir.display())
                                                    }
                                                    else 
                                                    {
                                                        if let Ok(copy_time) = io::io::copy_recursively(&source, &target)
                                                        {
                                                            info!("Задачей `{}` пакет {} скопирован в {} за {}с. в соответсвии с правилом {}",task_name, dir_name, target_dir.display(), copy_time, rule);
                                                            ret = true;
                                                        }
                                                        else
                                                        {
                                                            error!("Ошибка копирования пакета {} в {} для задачи {}",dir_name, target_dir.display(), task_name);
                                                        }
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
            if !ret
            {
                warn!("Пакет {} не скопирован из-за ошибки или потому-что он не указан в правилах", &source.display());
            }
        }
    }

    ///Каждый таск обрабатывается в отдельном потоке
    fn check_for_new_packets_spawn(sender : crossbeam_channel::Sender<(Task, String)>)
    {
        let guard = STATE.get().unwrap().lock().unwrap();
        let tasks = guard.settings.tasks.clone();
        let _ = drop(guard);
        for t in tasks
        {
            let builder = std::thread::Builder::new().name(t.task_name.clone());
            let sender = sender.clone();
            let _ = builder.spawn(move ||
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
                                let _ = sender.send((t.clone(), d.to_owned())).unwrap();
                            }    
                        }
                        if is_change
                        {
                            DirectoriesSpy::serialize_exclude(&t.task_name);
                        }
                    }
                    let delay = t.get_task_delay();
                    let end = std::time::SystemTime::now();
                    let duration = end.duration_since(start).unwrap();
                    if is_change
                    {
                        logger::info!("Задача {} была завершена за {}c., перезапуск задачи через {}c.", std::thread::current().name().unwrap(), duration.as_secs(), &delay.as_secs());
                    }
                    std::thread::sleep(delay);
                }
            });
        }
    }
    ///Добавить к задаче имя директории, чтобы больше ее не копировать
    /// если возвращает true то директория успешно добавлена в список, если false то такая директория там уже есть
    fn add(task_name: &str, dir: &String) -> bool
    {
        let mut guard = EXCLUDES.get().unwrap().lock().unwrap();
        if !guard.contains_key(task_name)
        {
            guard.insert(task_name.to_owned(), vec![dir.to_owned()]);
            return true;
        }
        else 
        {
            if let Some(ex) = guard.get_mut(task_name)
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
        let guard = EXCLUDES.get().unwrap().lock().unwrap();
        if let Some(vec) = guard.get(task_name)
        {
            crate::io::serialize(vec, file_name, None);
        }  
    }
    pub fn deserialize_exclude(task: &Task)
    {
        let excl = EXCLUDES.get_or_init(|| Mutex::new(HashMap::new()));
        let mut guard = excl.lock().unwrap();
        if !guard.contains_key(task.task_name.as_str())
        {
            let file = [&task.task_name, ".txt"].concat();
            let path = Path::new(&file);
            let ex = crate::io::deserialize::<Vec<String>>(&path);
            guard.insert(task.task_name.clone(), ex.1);
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