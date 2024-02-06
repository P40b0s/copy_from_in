use std::{self, path::{Path, PathBuf}, sync::{Arc, Mutex}, collections::HashMap};
use logger::{debug, error, info, warn, LevelFilter};
use medo_parser::Packet;
use once_cell::sync::{OnceCell, Lazy};
use settings::{CopyModifier, Settings, Task};
use tauri::Manager;
use crate::{ state::AppState, LOG_SENDER};
use crossbeam_channel::bounded;
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

    ///Возвырат сообщений из канала реализован в главном потоке, управление в main не возвращается, 
    ///так как главный поток больше ни для чего не используется, оставлю так
    pub async fn process_tasks<R: tauri::Runtime>(manager: Arc<impl Manager<R>>) -> anyhow::Result<()>
    {
        let state = manager.state::<AppState>().inner();
        let (s, r) = bounded::<(Task, String)>(10);
        let settings = state.get_settings();
        DirectoriesSpy::check_for_new_packets_spawn(settings, s);
        loop 
        {
            while let Ok(rec) = r.recv() 
            {
                Self::copy_files_process(rec.0, rec.1).await;
            }
        }
    }

    async fn copy_files_process(task: Task, founded_packet_name : String)
    {
        let task_name = task.get_task_name();
        let source_dir = task.get_source_dir();
        let target_dir = task.get_target_dir();
        let packet_name = &founded_packet_name;
        let source_path = Path::new(source_dir).join(packet_name);
        let target_path = Path::new(target_dir).join(packet_name);
        debug!("Сообщение от задачи `{}` -> найден новый пакет {}", task_name, source_path.display());
        let packet = medo_parser::Packet::parse(&source_path);
        if let Some(errors) = packet.get_error()
        {
            let err = format!("Ошибка обработки пакета {} -> {}", &source_path.display(),  errors);
            let lg = LOG_SENDER.get().unwrap().lock().await;
            error!("{}", &err);
            let _ = lg.send((LevelFilter::Error, err));
            Self::delete(task_name, &founded_packet_name);
            return;
        }
        match task.copy_modifier
        {
            CopyModifier::CopyAll =>
            {
                Self::copy_process(&target_path, &source_path, &packet.get_packet_name(), &task);
            },
            CopyModifier::CopyOnly =>
            {
                Self::rule_is_confirmed(&source_path, &target_path, &packet, &task, true).await;
            },
            CopyModifier::CopyExcept =>
            {
                Self::rule_is_confirmed(&source_path, &target_path, &packet, &task, false).await;
            },
        }
    }

    ///Отработали ли правила из текущей задачи
    ///`need_rule_accept` при ключе фильтра copy_only нужно поставить true а при ключе copy_except - false
    ///`only_doc` правила подтвердятся только если тип документа один из тек что перечислены в конфиге
    async fn rule_is_confirmed(source_path: &PathBuf, target_path: &PathBuf, packet: &Packet, task: &Task, need_rule_accept: bool) -> bool
    {
        // if task.document_types.len() == 0 && task.document_uids.len() == 0
        // {
        //     let wrn = format!("Для задачи {} -> не определены правила!", source_path.display());
        //     let lg = LOG_SENDER.get().unwrap().lock().await;
        //     warn!("{}", &wrn);
        //     let _ = lg.send((LevelFilter::Warn, wrn));
        //     Self::delete(&task.name, &packet.get_packet_name().to_owned());
        //     return false;
        // }
        //rc пакеты не копиаируются если включен режим известных типов документов

        if task.document_types.len() > 0 && task.document_uids.len() > 0
        {
            let packet_type = packet.get_packet_type();
            if packet_type.is_none()
            {
                let err = format!("Ошибка обработки пакета {} -> выбрано копирование пакетов по типу, но тип пакета не найден", source_path.display());
                let lg = LOG_SENDER.get().unwrap().lock().await;
                error!("{}", &err);
                let _ = lg.send((LevelFilter::Error, err));
                Self::delete(&task.name, &packet.get_packet_name().to_owned());
                return false;
            }
            if task.document_types.contains(&packet_type.unwrap().into_owned()) == need_rule_accept
            {
                return Self::copy_process(&target_path, &source_path,  &packet.get_packet_name(), &task);
            }    
        }
        else 
        {
            if task.document_types.len() > 0
            {
                let packet_type = packet.get_packet_type();
                if packet_type.is_none()
                {
                    let err = format!("Ошибка обработки пакета {} -> выбрано копирование пакетов по типу, но тип пакета не найден", source_path.display());
                    let lg = LOG_SENDER.get().unwrap().lock().await;
                    error!("{}", &err);
                    let _ = lg.send((LevelFilter::Error, err));
                    Self::delete(&task.name, &packet.get_packet_name().to_owned());
                    return false;
                }
                if task.document_types.contains(&packet_type.unwrap().into_owned()) == need_rule_accept
                {
                    return Self::copy_process(&target_path, &source_path,  &packet.get_packet_name(), &task);
                }    
            }
            if task.document_uids.len() > 0
            {
                let source_uid = packet.get_source_uid();
                if source_uid.is_none()
                {
                    let err = format!("Ошибка обработки пакета {} -> выбрано копирование пакетов по uid отправителя, но uid отправителя в пакете не найден", source_path.display());
                    let lg = LOG_SENDER.get().unwrap().lock().await;
                    error!("{}", &err);
                    let _ = lg.send((LevelFilter::Error, err));
                    Self::delete(&task.name, &packet.get_packet_name().to_owned());
                    return false;
                }
                if task.document_uids.contains(&source_uid.unwrap().into_owned()) == need_rule_accept
                {
                    return Self::copy_process(&target_path, &source_path, &packet.get_packet_name(), &task);
                }    
            }
        }
        return false;
    }

    fn copy_process(target_path: &PathBuf,
        source_path: &PathBuf,
        packet_dir_name: &str, 
        task : &Task) -> bool
    {
        if super::io::path_is_exists(&target_path)
        {
            warn!("Пакет {} уже существует по адресу {} копирование пакета отменено",packet_dir_name, target_path.display());
            return false;
        }
        else 
        {
            if let Ok(copy_time) = super::io::copy_recursively(&source_path, &target_path)
            {
                if task.delete_after_copy
                {
                    if let Err(e) = std::fs::remove_dir_all(source_path)
                    {
                        error!("Ошибка удаления директории {} для задачи {} -> {}",source_path.display(), task.name, e.to_string() );
                    }
                }
                info!("Задачей `{}` c модификатором {} пакет {} скопирован в {} за {}с.",task.name, task.copy_modifier, packet_dir_name, &target_path.display(), copy_time);
                return true;
            }
            else
            {
                error!("Ошибка копирования пакета {} в {} для задачи {}",packet_dir_name, &target_path.display(), task.name);
                return false;
            }
        }
    }

    ///Каждый таск обрабатывается в отдельном потоке
    async fn check_for_new_packets_spawn(settings: Settings, sender : crossbeam_channel::Sender<(Task, String)>)
    {
        let tasks = settings.tasks;
        for t in tasks
        {
            if !t.is_active
            {
                let wrn = format!("Задач {} -> не активна и не будет запущена (флаг is_active)", t.get_task_name());
                let lg = LOG_SENDER.get().unwrap().lock().await;
                warn!("{}", &wrn);
                let _ = lg.send((LevelFilter::Warn, wrn));
            }
            else if (t.copy_modifier == CopyModifier::CopyOnly || t.copy_modifier == CopyModifier::CopyExcept) && (t.document_types.len() == 0 && t.document_uids.len() == 0)
            {
                let wrn = format!("Для задачи {} -> не определены правила!", t.get_task_name());
                let lg = LOG_SENDER.get().unwrap().lock().await;
                warn!("{}", &wrn);
                let _ = lg.send((LevelFilter::Warn, wrn));
            }
            else
            {
                let builder = std::thread::Builder::new().name(t.name.clone());
                let sender = sender.clone();
                let _ = builder.spawn(move ||
                {
                    loop 
                    {
                        let start = std::time::SystemTime::now();
                        let paths = Self::get_dirs(&t.source_dir);
                        if paths.is_none()
                        {
                            break;
                        }
                        let mut is_change = false;
                        if let Some(reader) = paths.as_ref()
                        {
                            for d in reader
                            {
                                if Self::add(&t.name, d)
                                {
                                    is_change = true;
                                    let _ = sender.send((t.clone(), d.to_owned())).unwrap();
                                }    
                            }
                            if is_change
                            {
                                Self::serialize_exclude(&t.name);
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
    fn delete(task_name: &str, dir: &String)
    {
        let mut guard = EXCLUDES.get().unwrap().lock().unwrap();
        if let Some(v) = guard.get_mut(task_name)
        {
            v.retain(|r| r != dir);
        }
    }
    fn serialize_exclude(task_name: &str,)
    {
        let concat_path = [task_name, ".task"].concat();
        let file_name = Path::new(&concat_path);
        let guard = EXCLUDES.get().unwrap().lock().unwrap();
        if let Some(vec) = guard.get(task_name)
        {
            super::serialize::serialize(vec, file_name, None);
        }  
    }
    pub fn deserialize_exclude(task: &Task)
    {
        let excl = EXCLUDES.get_or_init(|| Mutex::new(HashMap::new()));
        let mut guard = excl.lock().unwrap();
        if !guard.contains_key(task.name.as_str())
        {
            let file = [&task.name, ".task"].concat();
            let path = Path::new(&file);
            let ex = super::serialize::deserialize::<Vec<String>>(&path);
            guard.insert(task.name.clone(), ex.1);
        }
    }
    
}