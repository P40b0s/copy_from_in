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
        let mut rx = "[a-z0-9]+:type=\"(".to_owned();
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
        let packet_name = &founded_packet_name;
        if !first_time
        {
            let source_path = Path::new(source_dir).join(packet_name);
            let target_path = Path::new(target_dir).join(packet_name);
            debug!("Сообщение от задачи `{}` -> найден новый пакет {}", task_name, source_path.display());
            match task.copy_modifier
            {
                CopyModifier::CopyAll =>
                {
                    if task.only_docs
                    {
                        let mut finded_xml_rc_type = false;
                        let mut regex_ok = false;
                        if let Some(entry) = io::io::get_files(&source_path)
                        {
                            for e in entry
                            {
                                if io::io::extension_is(&e, "xml")
                                {
                                    if let Some(text) = io::io::read_file(&e.path())
                                    {
                                        finded_xml_rc_type = true;
                                        if ONLY_DOC_REGEX.is_match(&text)
                                        {
                                            regex_ok = true;
                                            Self::copy_process(&target_path, &source_path, target_dir, packet_name, &task);
                                        }
                                    }
                                }
                                if io::io::extension_is(&e, "rc")
                                {
                                    finded_xml_rc_type = true;
                                    regex_ok = true;
                                    Self::copy_process(&target_path, &source_path, target_dir, packet_name, &task);
                                }
                            }
                        }
                        if !finded_xml_rc_type
                        {
                            error!("Пакет {} не скопирован, в директории не обнаружены файлы с расширением .xml или .rc", &source_path.display());
                            return;
                        }
                        if !regex_ok
                        {
                            error!("Пакет {} не скопирован, тип пакета xdms:type не соответсвует regex из настрек прграммы {}", &source_path.display(), ONLY_DOC_REGEX.as_str());
                            return;
                        }
                    }
                    else
                    {
                        Self::copy_process(&target_path, &source_path, target_dir, packet_name, &task);
                    }
                },
                CopyModifier::CopyOnly =>
                {
                    if Self::check_rules(&source_path, &task, true)
                    {
                        Self::copy_process(&target_path, &source_path, target_dir, packet_name, &task);
                    }
                },
                CopyModifier::CopyExcept =>
                {
                    if Self::check_rules(&source_path, &task, false)
                    {
                        Self::copy_process(&target_path, &source_path, target_dir, packet_name, &task);
                    }
                },
            }
        }
    }

    ///Отработали ли правила из текущей задачи
    ///`need_rule_accept` при ключе фильтра copy_only нужно поставить true а при ключе copy_except - false
    ///`only_doc` правила подтвердятся только если тип документа один из тек что перечислены в конфиге
    fn check_rules(source_path: &PathBuf, task: &Task, need_rule_accept: bool) -> bool
    {
        if task.rules.is_empty()
        {
            error!("Для задачи {} установлен режим {} но правила не обнаружены, для режимов copy_only и copy_except указывать правила обязательно", task.task_name, task.copy_modifier);
            return false;
        }
        if let Some(entry) = io::io::get_files(&source_path)
        {
            let mut xml_found =false;
            let mut rx_match = false;
            for e in entry
            {
                if io::io::extension_is(&e, "xml")
                {
                    if let Some(text) = io::io::read_file(&e.path())
                    {
                        xml_found = true;
                        for rule in &task.rules
                        {
                            if let Some(rx) = REGEXES.lock().unwrap().get(rule)
                            {
                                if task.only_docs
                                {
                                    if (rx.is_match(&text) == need_rule_accept) && ONLY_DOC_REGEX.is_match(&text)
                                    {
                                        info!("Для задачи `{}` и пакета {} сработало правило {}",task.task_name, source_path.display(), rule);
                                        rx_match = true;
                                    }
                                }
                                else
                                {
                                    if rx.is_match(&text) == need_rule_accept
                                    {
                                        info!("Для задачи `{}` и пакета {} сработало правило {}",task.task_name, source_path.display(), rule);
                                        rx_match = true;
                                    }
                                }
                            }
                        }
                    }
                }
            }
            if !xml_found
            {
                error!("Пакет {} не скопирован, в директории не обнаружены файлы с расширением .xml", &source_path.display());
                return false;
            }
            if !rx_match
            {
                error!("Пакет {} не скопирован, ни одно из указанных в настройках правил для задачи {} не сработало", &source_path.display(), task.get_task_name());
                return false;
            }
        }
        return true;
    }

    fn copy_process(target_path: &PathBuf,
        source_path: &PathBuf,
        target_dir: &PathBuf, 
        packet_dir_name: &str, 
        task : &Task) -> bool
    {
        if io::io::path_is_exists(&target_path)
        {
            warn!("Пакет {} уже существует по адресу {} копирование пакета отменено",packet_dir_name, target_dir.display());
            return false;
        }
        else 
        {
            if let Ok(copy_time) = io::io::copy_recursively(&source_path, &target_path)
            {
                if task.delete_after_copy
                {
                    if let Err(e) = std::fs::remove_dir_all(source_path)
                    {
                        error!("Ошибка удаления директории {} для задачи {} -> {}",source_path.display(), task.task_name, e.to_string() );
                    }
                }
                info!("Задачей `{}` c модификатором {} пакет {} скопирован в {} за {}с.",task.task_name, task.copy_modifier, packet_dir_name, target_dir.display(), copy_time);
                return true;
            }
            else
            {
                error!("Ошибка копирования пакета {} в {} для задачи {}",packet_dir_name, target_dir.display(), task.task_name);
                return false;
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
        let concat_path = [task_name, ".task"].concat();
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
            let file = [&task.task_name, ".task"].concat();
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