use std::{self, path::{Path, PathBuf}, sync::{Arc}, collections::HashMap};
use logger::{debug, error, info, warn, LevelFilter};
use medo_parser::Packet;
use once_cell::sync::{OnceCell, Lazy};
use settings::{CopyModifier, Settings, Task};
use tokio::sync::Mutex;
use tauri::Manager;
use crate::{ new_packet_found, state::AppState, NEW_DOCS};
use crossbeam_channel::bounded;

use super::{NewDocument, NewPacketInfo};
static TIMERS: Lazy<Arc<Mutex<HashMap<String, u64>>>> = Lazy::new(|| Arc::new(Mutex::new(HashMap::new())));

pub struct DirectoriesSpy;

impl DirectoriesSpy
{
    pub async fn initialize(settings: &Settings)
    {
        for t in &settings.tasks
        {
            Settings::load_exclude(t)
        }
    }
    ///Будет вызываться каждые 15 секунда пока что, надо чтобы сюда пробрасывались актуальные настройки после изменения в глобальном стейте, 
    pub async fn process_tasks<R: tauri::Runtime>(manager: Arc<impl Manager<R>>) -> anyhow::Result<()>
    {
        let state = manager.state::<AppState>().inner();
        let settings = state.get_settings();
        Self::process_timers(&settings).await;
        Ok(())
    }

    async fn process_timers(settings: &Settings)
    {
        for t in &settings.tasks
        {
            let mut guard = TIMERS.lock().await;
            if guard.contains_key(&t.name)
            {
                let countdown = guard.get(&t.name).unwrap() - 15000;
                debug!("{}", countdown);
                if countdown > 0
                {
                    *guard.get_mut(&t.name).unwrap() = countdown;
                    drop(guard);
                }
                else 
                {
                    *guard.get_mut(&t.name).unwrap() = t.timer;
                    drop(guard);
                    //таск 1 а вот пакетов может быть несколько
                    let tsk = Arc::new(t.clone());
                    tokio::spawn(async move
                    {
                        let ready_tasks = Self::scan_dir(tsk).await;
                        for ready_task in ready_tasks
                        {
                            Self::copy_files_process(ready_task.0, ready_task.1).await;
                        }
                    });
                }
            }
            else
            {
                guard.insert(t.name.clone(), t.timer);
                drop(guard);
            }
        }
    }

    async fn copy_files_process(task: Arc<Task>, founded_packet_name : String)
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
            if let Ok(copy_time) = super::io::copy_recursively(&source_path, &target_path, 3000)
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
    ///`need_rule_accept` при ключе фильтра CopyOnly нужно поставить true а при ключе CopyExcept - false
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
                Settings::load_exclude(&t);
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
                                if Settings::add_to_exclude(&t.name, d)
                                {
                                    is_change = true;
                                    let _ = sender.send((t.clone(), d.to_owned())).unwrap();
                                }    
                            }
                            if is_change
                            {
                                Settings::save_exclude(&t.name);
                            }
                        }
                        let delay = t.get_task_delay();
                        //let end = std::time::SystemTime::now();
                        //let duration = end.duration_since(start).unwrap();
                        //if is_change
                        //{
                        //    logger::info!("Задача {} была завершена за {}c., перезапуск задачи через {}c.", std::thread::current().name().unwrap(), duration.as_secs(), &delay.as_secs());
                        //}
                        std::thread::sleep(delay);
                    }
                });
            }
        }
    }

    ///проверяем новые пакеты у тасков с вышедшим таймером, получаем список тасков у которых найдены новые пакеты
    async fn scan_dir(task: Arc<Task>) -> Vec<(Arc<Task>, String)>
    {
        let mut prepared_tasks : Vec<(Arc<Task>, String)> = vec![];
        if task.is_active
        {
            let paths = super::io::get_dirs(&task.source_dir);
            if let Some(reader) = paths.as_ref()
            {
                let mut exclude_is_changed = false;
                for d in reader
                {
                    let cloned_task = Arc::clone(&task);
                    if Settings::add_to_exclude(&cloned_task.name, d)
                    {
                        exclude_is_changed = true;
                        prepared_tasks.push((cloned_task, d.to_owned()));
                    }    
                }
                if exclude_is_changed
                {
                    Settings::save_exclude(&task.name);
                }
            }
        }
        prepared_tasks
    }
}

async fn send_new_document(packet: impl Into<NewPacketInfo>)
{
    let lg = NEW_DOCS.get().unwrap().lock().await;
    let _ = lg.send(packet.into());
}