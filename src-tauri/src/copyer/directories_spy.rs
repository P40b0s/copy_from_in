use std::{self, path::{Path, PathBuf}, sync::{Arc, Mutex}, collections::HashMap};
use logger::{debug, error, info, warn, LevelFilter};
use medo_parser::Packet;
use once_cell::sync::{OnceCell, Lazy};
use settings::{CopyModifier, Settings, Task};
use tauri::Manager;
use crate::{ new_packet_found, state::AppState, NEW_DOCS};
use crossbeam_channel::bounded;

use super::{NewDocument, NewPacketInfo};
pub static EXCLUDES: OnceCell<Mutex<HashMap<String, Vec<String>>>> = OnceCell::new();

pub struct DirectoriesSpy;
impl DirectoriesSpy
{
    ///Возвырат сообщений из канала реализован в главном потоке, управление в main не возвращается, 
    ///так как главный поток больше ни для чего не используется, оставлю так
    pub async fn process_tasks<R: tauri::Runtime>(manager: Arc<impl Manager<R>>) -> anyhow::Result<()>
    {
        let state = manager.state::<AppState>().inner();
        let (s, r) = bounded::<(Task, String)>(10);
        let settings = state.get_settings();
        DirectoriesSpy::start_tasks(settings, s).await;
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
        match task.copy_modifier
        {
            CopyModifier::CopyAll =>
            {
                if Self::copy_process(&target_path, &source_path, &founded_packet_name, &task)
                {
                    send_new_document(NewDocument::new(&founded_packet_name)).await;
                }
            },
            CopyModifier::CopyOnly =>
            {
                if let Some(packet) = Self::get_packet(&source_path).await
                {
                    if Self::copy_with_rules(&source_path, &target_path, &packet, &task, true).await
                    {
                        send_new_document(&packet).await;
                    }
                }
                else
                {
                    return;
                }
            },
            CopyModifier::CopyExcept =>
            {
                if let Some(packet) = Self::get_packet(&source_path).await
                {
                    if Self::copy_with_rules(&source_path, &target_path, &packet, &task, false).await
                    {
                        send_new_document(&packet).await;
                    }
                }
                else
                {
                    return;
                }
                
            },
        }
    }

    async fn get_packet(source_path: &PathBuf) -> Option<Packet>
    {
        let packet = medo_parser::Packet::parse(&source_path);
        if let Some(errors) = packet.get_error()
        {
            let err = format!("Ошибка обработки пакета {} -> {}", &source_path.display(),  errors);
            error!("{}", &err);
            send_new_document(err).await;
            return None;
        }
        return Some(packet)
    }

    ///Отработали ли правила из текущей задачи
    ///`need_rule_accept` при ключе фильтра copy_only нужно поставить true а при ключе copy_except - false
    ///`only_doc` правила подтвердятся только если тип документа один из тек что перечислены в конфиге
    async fn copy_with_rules(source_path: &PathBuf, target_path: &PathBuf, packet: &Packet, task: &Task, need_rule_accept: bool) -> bool
    {
        if task.filters.document_types.len() > 0 && task.filters.document_uids.len() > 0 
        && Self::packet_type_rule(packet, task, source_path, need_rule_accept).await 
        && Self::source_uid_rule(packet, task, source_path, need_rule_accept).await
        {
            return Self::copy_process(&target_path, &source_path,  &packet.get_packet_name(), &task);
        }
        else
        {
            if task.filters.document_types.len() > 0 && Self::packet_type_rule(packet, task, source_path, need_rule_accept).await 
            {
                return Self::copy_process(&target_path, &source_path,  &packet.get_packet_name(), &task);
            }
            if task.filters.document_uids.len() > 0 && Self::source_uid_rule(packet, task, source_path, need_rule_accept).await
            {
                return Self::copy_process(&target_path, &source_path, &packet.get_packet_name(), &task);
            }
        }
        return false;
    }

    async fn packet_type_rule(packet: &Packet, task: &Task, source_path: &PathBuf, need_rule_accept: bool) -> bool
    {
        let packet_type = packet.get_packet_type();
        if packet_type.is_none()
        {
            let err = format!("Ошибка обработки пакета {} -> выбрано копирование пакетов по типу, но тип пакета не найден", source_path.display());
            error!("{}", &err);
            send_new_document(err).await;
            return false;
        }
        if task.filters.document_types.contains(&packet_type.unwrap().into_owned()) == need_rule_accept
        {
            return true;
        }
        false 
    }
    async fn source_uid_rule(packet: &Packet, task: &Task, source_path: &PathBuf, need_rule_accept: bool) -> bool
    {
        let source_uid = packet.get_source_uid();
        if source_uid.is_none()
        {
            let err = format!("Ошибка обработки пакета {} -> выбрано копирование пакетов по uid отправителя, но uid отправителя в пакете не найден", source_path.display());
            error!("{}", &err);
            send_new_document(err).await;
            return false;
        }
        if task.filters.document_uids.contains(&source_uid.unwrap().into_owned()) == need_rule_accept
        {
            return true;
        }    
        false 
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
    async fn start_tasks(settings: Settings, sender : crossbeam_channel::Sender<(Task, String)>)
    {
        let tasks = settings.tasks;
        for t in tasks
        {
            if !t.is_active
            {
                let wrn = format!("Задач {} -> не активна и не будет запущена (флаг is_active)", t.get_task_name());
                warn!("{}", &wrn);
                continue;
            }
            else
            {
                Self::deserialize_exclude(&t);
                let builder = std::thread::Builder::new().name(t.name.clone());
                let sender = sender.clone();
                let _ = builder.spawn(move ||
                {
                    loop 
                    {
                        let start = std::time::SystemTime::now();
                        let paths = super::io::get_dirs(&t.source_dir);
                        if paths.is_none()
                        {
                            continue;
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

async fn send_new_document(packet: impl Into<NewPacketInfo>)
{
    let lg = NEW_DOCS.get().unwrap().lock().await;
    let _ = lg.send(packet.into());
}