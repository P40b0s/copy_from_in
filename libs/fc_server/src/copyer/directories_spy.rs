use std::{self, collections::HashMap, path::{Path, PathBuf}, sync::{atomic::AtomicBool, Arc}};
use logger::{debug, error, info};
use transport::{DeliveryTicketPacket, PacketInfo};
use once_cell::sync::{Lazy, OnceCell};
use settings::{CopyModifier, FileMethods, Settings, Task};
use tokio::{runtime::Runtime, sync::Mutex};
use transport::Packet;
use utilites::retry;
use crate::state::AppState;
//use crossbeam_channel::{bounded, Receiver, Sender};
use async_channel::{bounded, Sender, Receiver};

use super::{CopyService, ExcludesTrait};

//для каждой задачи есть свой таймер, отнимаем 15 сек от времени задачи каждую итерацию
static TIMERS: Lazy<Arc<Mutex<HashMap<String, u64>>>> = Lazy::new(|| Arc::new(Mutex::new(HashMap::new())));
static NEW_PACKET_EVENT: OnceCell<Arc<Sender<Packet>>> = OnceCell::new();
pub struct DirectoriesSpy
{
    timers: Lazy<Arc<Mutex<HashMap<String, u64>>>>,
    state: Arc<AppState>,
    delay: u64
}

impl DirectoriesSpy
{
    pub fn new(state: Arc<AppState>, tasks_check_delay: u64) -> Self
    {
        Self
        {
            timers: Lazy::new(|| Arc::new(Mutex::new(HashMap::new()))),
            state,
            delay: tasks_check_delay
        }
    }
    pub async fn start_tasks(&self)
    {
        let handle  = tokio::runtime::Handle::current();
        //let state = Arc::clone(&self.state);
        tokio::task::block_in_place(move || 
        {
            let _ = handle.block_on(async move 
            {
                loop 
                {
                    let settings = self.state.get_settings().await;
                    for t in &settings.tasks
                    {
                        let mut guard = self.timers.lock().await;
                        if guard.contains_key(&t.name)
                        {
                            let countdown = guard.get(&t.name).unwrap() - self.delay;
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
                                let service = self.state.get_copy_service();
                                if tsk.generate_exclude_file
                                {
                                    let _ = service.excludes_service.replace(&tsk).await;
                                    let mut guard = self.state.settings.lock().await;
                                    let task = guard.tasks.iter_mut().find(|t|t.get_task_name() == tsk.get_task_name()).unwrap();
                                    task.generate_exclude_file = false;
                                    let _ = guard.save(settings::Serializer::Toml);
                                }
                                tokio::spawn(async move
                                {
                                    let ready_tasks: Vec<(Arc<Task>, String)> = Self::scan_dir(tsk, service).await;
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
                    tokio::time::sleep(tokio::time::Duration::from_millis(self.delay)).await;
                }
            });
        });
    }
    pub async fn subscribe_new_packet_event() -> Receiver<Packet>
    {
        let (sender, receiver) = bounded::<Packet>(2);
        let _ = NEW_PACKET_EVENT.set(Arc::new(sender));
        receiver
    }
    
    async fn copy_files_process(task: Arc<Task>, founded_packet_name : String)
    {
        let task_name = task.get_task_name();
        let source_dir = task.get_source_dir();
        let target_dirs = task.get_target_dirs();
        let packet_name = &founded_packet_name;
        let source_path = Path::new(source_dir).join(packet_name);
        //let target_path = Path::new(target_dir).join(packet_name);
        debug!("Сообщение от задачи `{}` -> найден новый пакет {}", task_name, source_path.display());
        match task.copy_modifier
        {
            //копируются все директории поэтому парсить транспортный пакет не имеет смысла
            CopyModifier::CopyAll =>
            {
                let mut packet = Packet::new_empty(&founded_packet_name, &task);
                for td in target_dirs
                {
                    let target_path = Path::new(td).join(packet_name);
                    if Self::copy_process(&target_path, &source_path, &founded_packet_name, &task).await
                    {
                        packet.add_copy_status(true, target_path.as_path().display().to_string());
                    }
                    else 
                    {
                        packet.add_copy_status(false, target_path.as_path().display().to_string());
                    }
                }
                if task.delete_after_copy && packet.copy_ok()
                {
                    if let Err(e) = tokio::fs::remove_dir_all(&source_path).await
                    {
                        error!("Ошибка удаления директории {} для задачи {} -> {}",source_path.display(), task.name, e.to_string() );
                    }
                }
                new_packet_found(packet).await;
            },
            CopyModifier::CopyOnly => Self::copy_with_rules(&source_path, target_dirs, packet_name, &task, true).await,
            CopyModifier::CopyExcept => Self::copy_with_rules(&source_path, target_dirs, packet_name, &task, false).await
        }
        
    }

    async fn copy_with_rules(source_path: &PathBuf, target_dirs: &[PathBuf], packet_name: &str, task: &Task, need_rule_accept: bool)
    {
        if let Some(packet) = Self::get_packet(&source_path, &task).await
        {
            let mut packet = packet;
            if !Self::autoclean(&source_path, &packet, &task).await
            {
                if !packet.is_err()
                {
                    if Self::pass_rules(&source_path, &packet, &task, need_rule_accept).await
                    {
                        for td in target_dirs
                        {
                            let target_path = Path::new(td).join(packet_name);
                            if Self::copy_process(&target_path, &source_path, packet_name, &task).await
                            {
                                packet.add_copy_status(true, target_path.as_path().display().to_string());
                            }
                            else 
                            {
                                packet.add_copy_status(false, target_path.as_path().display().to_string());
                            }
                        }
                        if task.delete_after_copy && packet.copy_ok()
                        {
                            if let Err(e) = tokio::fs::remove_dir_all(source_path).await
                            {
                                error!("Ошибка удаления директории {} для задачи {} -> {}",source_path.display(), task.name, e.to_string() );
                            }
                        }
                        new_packet_found(packet).await;
                    }
                }
                else
                {
                    new_packet_found(packet).await;
                }
            }
        }
        
    }

    async fn copy_process(target_path: &PathBuf,
        source_path: &PathBuf,
        packet_dir_name: &str, 
        task : &Task) -> bool
    {
        let cp_result = retry(10, 10000, 30000, ||
        {
            super::io::copy_recursively_async(Arc::new(source_path.clone()), Arc::new(target_path.clone()), 2000)
           
        }).await;
        if let Ok(_) = cp_result
        {
            info!("Задачей `{}` c модификатором {} пакет {} скопирован в {}",task.name, task.copy_modifier, packet_dir_name, &target_path.display());
            return true;
        }
        else
        {
            error!("Ошибка копирования пакета (на десятой попытке) {} в {} для задачи {}",packet_dir_name, &target_path.display(), task.name);
            return false;
        }
    }

    async fn get_packet(source_path: &PathBuf, task : &Task) -> Option<Packet>
    {
        if let Some(packet_info) = PacketInfo::parse(source_path)
        {
            Some(Packet::parse(source_path, packet_info, task))
        }
        else 
        {
            None
        }
       
    }

    async fn pass_rules(source_path: &PathBuf, packet: &Packet, task: &Task, need_rule_accept: bool) -> bool
    {
        let mut pass = false;
        if task.filters.document_types.len() > 0 && task.filters.document_uids.len() > 0 
        && Self::packet_type_rule(packet, task, source_path, need_rule_accept).await 
        && Self::source_uid_rule(packet, task, source_path, need_rule_accept).await
        {
            pass = true;
        }
        else
        {
            if task.filters.document_types.len() > 0 && Self::packet_type_rule(packet, task, source_path, need_rule_accept).await 
            {
                pass = true;
            }
            if task.filters.document_uids.len() > 0 && Self::source_uid_rule(packet, task, source_path, need_rule_accept).await
            {
                pass = true;
            }
        }
        pass
    }
    ///если установлена автоочистка пакетов то при удалении вернет true иначе false
    async fn autoclean(source_path: &PathBuf, packet: &Packet, task: &Task) -> bool
    {
        if task.autocleaning
        {
            if let Some(dt) = packet.get_packet_info().packet_type.as_ref()
            {
                if task.clean_types.contains(&dt)
                {
                    if let Ok(_) = tokio::fs::remove_dir_all(source_path).await
                    {
                        debug!("Пакет {} был удален, так как в настройках задачи {} включен флаг автоочистка", packet.get_packet_name(), packet.get_task().get_task_name());
                        return true;
                    }
                }
            }
        }
        false
    }

    async fn packet_type_rule(packet: &Packet, task: &Task, source_path: &PathBuf, need_rule_accept: bool) -> bool
    {
        let packet_type = &packet.get_packet_info().packet_type;
        if packet_type.is_none()
        {
            let err = format!("Ошибка обработки пакета {} -> тип пакета (xdms:type) в схеме xml не найден", source_path.display());
            error!("{}", &err);
            let pi = Packet::new_err(packet.get_packet_name(),  task, err.as_str());
            new_packet_found(pi).await;
            return false;
        }
        if task.filters.document_types.contains(packet_type.as_ref().unwrap()) == need_rule_accept
        {
            return true;
        }
        false 
    }

    async fn source_uid_rule(packet: &Packet, task: &Task, source_path: &PathBuf, need_rule_accept: bool) -> bool
    {
        let source_uid = packet.get_packet_info()
                                        .sender_info.as_ref().and_then(|a| a.source_guid.as_ref());
        if source_uid.is_none()
        {
            let err = format!("Ошибка обработки пакета {} -> uid отправителя в пакете не найден", source_path.display());
            error!("{}", &err);
            let pi = Packet::new_err(packet.get_packet_name(),  task, err.as_str());
            new_packet_found(pi).await;
            return false;
        }
        if task.filters.document_uids.contains(&source_uid.as_ref().unwrap()) == need_rule_accept
        {
            return true;
        }    
        false
    }

    ///проверяем новые пакеты у тасков с истекшим таймером, получаем список тасков у которых найдены новые пакеты
    async fn scan_dir(task: Arc<Task>, cp_service: Arc<CopyService>) -> Vec<(Arc<Task>, String)>
    {
        let mut prepared_tasks : Vec<(Arc<Task>, String)> = vec![];
        if task.is_active
        {
            let paths = utilites::io::get_dirs_async(&task.source_dir).await;
            if let Ok(reader) = paths.as_ref()
            {
                for d in reader
                {
                    if let Ok(is_added) = cp_service.excludes_service.add(&task.name, d).await
                    {
                        if is_added
                        {
                            prepared_tasks.push((Arc::clone(&task), d.to_owned()));
                        }
                    }
                }
            }
        }
        prepared_tasks
    }
}

///Обнаружен новый пакет при ошибке отправляем всем ошибку
async fn new_packet_found(mut packet: Packet)
{
    if packet.is_err()
    {
        logger::error!("{}", packet.get_error().as_ref().unwrap());
    }
    let sended = send_report(packet.get_packet_name(), packet.get_packet_info(), packet.get_task()).await;
    packet.report_sended = sended;
    if let Some(sender) = NEW_PACKET_EVENT.get()
    {
        let _ = sender.send(packet.clone()).await;
    }
}

async fn send_report(packet_name: &str, new_packet: &PacketInfo, task: &Task) -> bool
{
    if let Some(r_dir) = task.get_report_dir()
    {
        if let Some(e) = new_packet.error.as_ref()
        {
            logger::error!("{}, уведомление отправлено не будет", e);
            return false;
        }
        else
        {
            let doc_uid = new_packet.requisites.as_ref().and_then(|a| a.document_guid.as_ref());
            let source_uid = new_packet.sender_info.as_ref().and_then(|o| o.source_guid.as_ref());
            let organization = new_packet.sender_info.as_ref().and_then(|o| o.organization.as_ref());
            let addresse = new_packet.sender_info.as_ref().and_then(|o| o.medo_addressee.as_ref());
            let mut err: Vec<&str> = Vec::with_capacity(4);
            if doc_uid.is_none()
            {
                err.push("uid документа");
            }
            if source_uid.is_none()
            {
                err.push("uid отправителя");
            }
            if organization.is_none()
            {
                err.push("наименование организации отправителя");
            }
            if addresse.is_none()
            {
                err.push("адрес МЭДО отправителя");
            }
            if err.len() > 0
            {
                logger::error!("Недостаточно информации ({}) для отправки уведомления о доставке по пакету `{}`, уведомление отправлено не будет", err.join("/"), packet_name);
                return false;
            } 
            else
            {
                DeliveryTicketPacket::create_packet(
                    doc_uid.unwrap(),
                    source_uid.unwrap(),
                    organization.unwrap(),
                    addresse.unwrap()
                ).send(r_dir);
                return true;
            }
        }
    }
    return false;
}

#[cfg(test)]
mod tests
{
    #[tokio::test]
    async fn test_get_dirs_async()
    {
        let paths = utilites::io::get_dirs_async("../../test_data/copy_from_in_test_data/in3").await;
        println!("{:?}", paths);
    }
}