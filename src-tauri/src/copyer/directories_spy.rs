use std::{self, collections::HashMap, ops::Deref, path::{Path, PathBuf}, sync::{atomic::AtomicBool, Arc}};
use logger::{debug, error, info, warn, LevelFilter};
use medo_parser::{DeliveryTicketPacket, Packet};
use once_cell::sync::{OnceCell, Lazy};
use settings::{CopyModifier, FileMethods, Settings, Task};
use tokio::sync::Mutex;
use tauri::Manager;
use crate::{ new_packet_found, state::AppState, NEW_DOCS};
use crossbeam_channel::bounded;

use super::{NewDocument, NewPacketInfo};
static TIMERS: Lazy<Arc<Mutex<HashMap<String, u64>>>> = Lazy::new(|| Arc::new(Mutex::new(HashMap::new())));
static INIT: AtomicBool = AtomicBool::new(false);
pub struct DirectoriesSpy;

impl DirectoriesSpy
{
    ///Будет вызываться каждые 15 секунда, надо чтобы сюда пробрасывались актуальные настройки после изменения в глобальном стейте, 
    pub async fn process_tasks<R: tauri::Runtime>(manager: Arc<impl Manager<R>>) -> anyhow::Result<()>
    {
        let state = manager.state::<AppState>().inner();
        let settings = state.get_settings().await;
        if !INIT.load(std::sync::atomic::Ordering::SeqCst)
        {
            for t in &settings.tasks
            {
                Settings::load_exclude(t)
            }
            INIT.store(true, std::sync::atomic::Ordering::Relaxed);
        }
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
                if Self::copy_process(&target_path, &source_path, &founded_packet_name, &task).await
                {
                    let new_packet = NewPacketInfo::not_packet(&founded_packet_name, &task);
                    send_new_document(new_packet).await;
                }
            },
            CopyModifier::CopyOnly =>
            {
                if let Some(packet) = Self::get_packet(&source_path, &task).await
                {
                    if Self::copy_with_rules(&source_path, &target_path, &packet, &task, true).await
                    {
                        let new_packet = NewPacketInfo::from_packet(&packet, &task);
                        send_new_document(new_packet).await;
                    }
                }
                else
                {
                    return;
                }
            },
            CopyModifier::CopyExcept =>
            {
                if let Some(packet) = Self::get_packet(&source_path, &task).await
                {
                    if Self::copy_with_rules(&source_path, &target_path, &packet, &task, false).await
                    {
                        let new_packet = NewPacketInfo::from_packet(&packet, &task);
                        send_new_document(new_packet).await;
                    }
                }
                else
                {
                    return;
                }
                
            },
        }
    }
    async fn copy_process(target_path: &PathBuf,
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
            if let Ok(copy_time) = super::io::copy_recursively_async(&source_path, &target_path, 3000).await
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

    async fn get_packet(source_path: &PathBuf, task : &Task) -> Option<Packet>
    {
        let packet = medo_parser::Packet::parse(&source_path);
        if let Some(errors) = packet.get_error()
        {
            let err = format!("Ошибка обработки пакета {} -> {}", &source_path.display(),  errors);
            error!("{}", &err);
            let pi = NewPacketInfo::from_err(err.as_str(), packet.get_packet_name(), task);
            send_new_document(pi).await;
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
            return Self::copy_process(&target_path, &source_path,  &packet.get_packet_name(), &task).await;
        }
        else
        {
            if task.filters.document_types.len() > 0 && Self::packet_type_rule(packet, task, source_path, need_rule_accept).await 
            {
                return Self::copy_process(&target_path, &source_path,  &packet.get_packet_name(), &task).await;
            }
            if task.filters.document_uids.len() > 0 && Self::source_uid_rule(packet, task, source_path, need_rule_accept).await
            {
                return Self::copy_process(&target_path, &source_path, &packet.get_packet_name(), &task).await;
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
            let pi = NewPacketInfo::from_err(err.as_str(), packet.get_packet_name(), task);
            send_new_document(pi).await;
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
            let pi = NewPacketInfo::from_err(err.as_str(), packet.get_packet_name(), task);
            send_new_document(pi).await;
            return false;
        }
        if task.filters.document_uids.contains(&source_uid.unwrap().into_owned()) == need_rule_accept
        {
            return true;
        }    
        false 
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

// async fn send_new_document(packet: impl Into<NewPacketInfo>, task: &Task)
// {
//     let lg = NEW_DOCS.get().unwrap().lock().await;
//     let mut np: NewPacketInfo = packet.into();
//     //send_report(np.document.as_ref(), task).await;
//     np.task = Some(task.clone());
//     let _ = lg.send(np);
// }

async fn send_new_document(mut packet: NewPacketInfo)
{
    let lg = NEW_DOCS.get().unwrap().lock().await;
    let sended = send_report(packet.document.as_ref(), &packet.name, &packet.task).await;
    packet.report_sended = sended;
    let _ = lg.send(packet);
}

async fn send_report(new_doc: Option<&NewDocument>, packet_name: &str, task: &Task) -> bool
{
    if let Some(r_dir) = task.get_report_dir()
    {
        if let Some(doc) = new_doc
        {
            if doc.doc_uid.is_none()
            || doc.organization_uid.is_none()
            || doc.organization.is_none()
            || doc.source_medo_addressee.is_none()
            {
                logger::error!("В пакете {} не распознаны необходимые свойства для отправки уведомления о доставке, уведомление отправлено не будет", packet_name);
                return false;
            } 
            else
            {
                DeliveryTicketPacket::create_packet(
                    doc.doc_uid.as_ref().unwrap(),
                    doc.organization_uid.as_ref().unwrap(),
                    doc.organization.as_ref().unwrap(),
                    doc.source_medo_addressee.as_ref().unwrap()
                ).send(r_dir);
                return true;
            }
        }
        else 
        {
            logger::error!("В пакете {} не распознаны необходимые свойства для отправки уведомления о доставке, уведомление отправлено не будет", packet_name);
            return false;
        }
    }
    return false;
}