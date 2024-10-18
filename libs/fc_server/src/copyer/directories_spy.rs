use std::{self, collections::HashMap, path::{Path, PathBuf}, sync::{atomic::AtomicBool, Arc}};
use logger::{debug, error, info};
use transport::{DeliveryTicketPacket, PacketInfo};
use once_cell::sync::{Lazy, OnceCell};
use settings::{CopyModifier, FileMethods, Settings, Task};
use tokio::sync::Mutex;
use transport::Packet;
use crate::state::AppState;
//use crossbeam_channel::{bounded, Receiver, Sender};
use async_channel::{bounded, Sender, Receiver};

use super::CopyerService;

//для каждой задачи есть свой таймер, отнимаем 15 сек от времени задачи каждую итерацию
static TIMERS: Lazy<Arc<Mutex<HashMap<String, u64>>>> = Lazy::new(|| Arc::new(Mutex::new(HashMap::new())));
//static PACKETS: Lazy<Mutex<VecDeque<Packet>>> = Lazy::new(|| Mutex::new(VecDeque::with_capacity(LOG_LENGHT + 1)));
static NEW_PACKET_EVENT: OnceCell<Arc<Sender<Packet>>> = OnceCell::new();
static INIT: AtomicBool = AtomicBool::new(false);
pub struct DirectoriesSpy;


impl DirectoriesSpy
{
    pub async fn subscribe_new_packet_event() -> Receiver<Packet>
    {
        let (sender, receiver) = bounded::<Packet>(2);
        let _ = NEW_PACKET_EVENT.set(Arc::new(sender));
        receiver
    }
    ///Будет вызываться каждые 15 секунд, надо чтобы сюда пробрасывались актуальные настройки после изменения в глобальном стейте, 
    pub async fn process_tasks(state: Arc<AppState>) -> anyhow::Result<()>
    {
        
        //let settings = state.get_settings().await;
        // if !INIT.load(std::sync::atomic::Ordering::SeqCst)
        // {
        //     for t in &settings.tasks
        //     {
        //         Settings::load_exclude(t)
        //     }
        //     INIT.store(true, std::sync::atomic::Ordering::Relaxed);
        // }
        Self::process_timers(Arc::clone(&state)).await;
        Ok(())
    }

    async fn process_timers(state: Arc<AppState>)
    {
        let settings = state.get_settings().await;
        for t in &settings.tasks
        {
            let mut guard = TIMERS.lock().await;
            if guard.contains_key(&t.name)
            {
                let countdown = guard.get(&t.name).unwrap() - 15000;
                //debug!("{}", countdown);
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
                    let service = Arc::clone(&state.copyer_service);
                    if tsk.generate_exclude_file
                    {
                        service.excludes.replace(&tsk);
                        let mut guard = state.settings.lock().await;
                        let task = guard.tasks.iter_mut().find(|t|t.get_task_name() == tsk.get_task_name()).unwrap();
                        task.generate_exclude_file = false;
                        guard.save(settings::Serializer::Toml);
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
            //копируются все директории поэтому парсить транспортный пакет не имеет смысла
            CopyModifier::CopyAll =>
            {
                if Self::copy_process(&target_path, &source_path, &founded_packet_name, &task).await
                {
                    let new_packet = Packet::new_empty(&founded_packet_name, &task);
                    new_packet_found(new_packet).await;
                }
            },
            CopyModifier::CopyOnly =>
            {
                let packet = Self::get_packet(&source_path, &task).await;
                if !packet.is_err()
                {
                    if Self::copy_with_rules(&source_path, &target_path, &packet, &task, true).await
                    {
                        new_packet_found(packet).await;
                    }
                }
                else
                {
                    new_packet_found(packet).await;
                }
            },
            CopyModifier::CopyExcept =>
            {
                let packet = Self::get_packet(&source_path, &task).await;
                if !packet.is_err()
                {
                    if Self::copy_with_rules(&source_path, &target_path, &packet, &task, false).await
                    {
                        new_packet_found(packet).await;
                    }
                }
                else
                {
                    new_packet_found(packet).await;
                }
                
            },
        }
    }
    async fn copy_process(target_path: &PathBuf,
        source_path: &PathBuf,
        packet_dir_name: &str, 
        task : &Task) -> bool
    {
        //если это оставить то рескан не сработает, надо тогда удалять еще пакет по адресу копирования, пока так нормально
        // if super::io::path_is_exists(&target_path)
        // {
        //     warn!("Пакет {} уже существует по адресу {} копирование пакета отменено",packet_dir_name, target_path.display());
        //     return false;
        // }
        // else 
        // {
            if let Ok(_) = super::io::copy_recursively_async(Arc::new(source_path.clone()), Arc::new(target_path.clone()), 2000).await
            {  
                if task.delete_after_copy
                {
                    if let Err(e) = std::fs::remove_dir_all(source_path)
                    {
                        error!("Ошибка удаления директории {} для задачи {} -> {}",source_path.display(), task.name, e.to_string() );
                    }
                }
                info!("Задачей `{}` c модификатором {} пакет {} скопирован в {}",task.name, task.copy_modifier, packet_dir_name, &target_path.display());
                return true;
            }
            else
            {
                error!("Ошибка копирования пакета {} в {} для задачи {}",packet_dir_name, &target_path.display(), task.name);
                return false;
            }
        //}
    }


    async fn get_packet(source_path: &PathBuf, task : &Task) -> Packet
    {
        let packet_info = PacketInfo::parse(source_path);
        Packet::parse(source_path, packet_info, task)
    }

    ///Отработали ли правила из текущей задачи
    ///`need_rule_accept` при ключе фильтра CopyOnly нужно поставить true а при ключе CopyExcept - false
    ///`only_doc` правила подтвердятся только если тип документа один из тек что перечислены в конфиге
    async fn copy_with_rules(source_path: &PathBuf, target_path: &PathBuf, packet: &Packet, task: &Task, need_rule_accept: bool) -> bool
    {
        if task.autocleaning
        {
            if let Some(dt) = packet.get_packet_info().packet_type.as_ref()
            {
                if task.clean_types.contains(&dt)
                {
                    if let Ok(_) = std::fs::remove_dir_all(source_path)
                    {
                        debug!("Пакет {} был удален, так как в настройках задачи {} включен флаг автоочистка", packet.get_packet_name(), packet.get_task().get_task_name());
                        return false;
                    }
                }
            }
        }
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
        let packet_type = &packet.get_packet_info().packet_type;
        if packet_type.is_none()
        {
            let err = format!("Ошибка обработки пакета {} -> тип пакета в схеме xml не найден", source_path.display());
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
    async fn scan_dir(task: Arc<Task>, cp_service: Arc<CopyerService>) -> Vec<(Arc<Task>, String)>
    {
        let mut prepared_tasks : Vec<(Arc<Task>, String)> = vec![];
        if task.is_active
        {
            let paths = super::io::get_dirs(&task.source_dir);
            if let Some(reader) = paths.as_ref()
            {
                //let mut exclude_is_changed = false;
                for d in reader
                {
                    let cloned_task = Arc::clone(&task);
                    if let Ok(is_added) = cp_service.excludes.add(&cloned_task.name, d)
                    {
                        if is_added
                        {
                            //exclude_is_changed = true;
                            prepared_tasks.push((cloned_task, d.to_owned()));
                        }
                    }
                }
                // if exclude_is_changed
                // {
                //     Settings::save_exclude(&task.name);
                // }
            }
        }
        prepared_tasks
    }
}

///Обнаружен новый пакет при ошибке отправляем всем ошибку
async fn new_packet_found(mut packet: Packet)
{
    logger::debug!("Сервером отправлен новый пакет {:?}, {}", &packet, logger::backtrace!());
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
            logger::error!("Ошибка парсера {} в пакете {}, уведомление отправлено не будет", e, packet_name);
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