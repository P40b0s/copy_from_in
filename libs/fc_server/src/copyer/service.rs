use std::{path::Path, sync::{atomic::AtomicBool, Arc}};
use db_service::SqlitePool;
use settings::{Settings, Task};
use tokio::runtime::Runtime;
use transport::{Packet, PacketInfo};
use crate::{copyer::io::get_files, db::PacketTable, services::WebsocketServer, state::AppState};
use super::{excludes::{ExcludesService, ExcludesTrait}, io::get_dirs, SqliteExcludes};
static CLEAN_IN_PROGRESS: AtomicBool = AtomicBool::new(false);

pub struct PacketCleaner{}
impl PacketCleaner
{
    ///нам нужно вернуть только колчество удаленных пакетов, ошибки нас не интересуют
    /// + вернуть надо через websocket и не ждать ответа, это можеть быть долго
    pub async fn clean_packets(&self, app_state: Arc<AppState>)
    {
        if !CLEAN_IN_PROGRESS.load(std::sync::atomic::Ordering::Relaxed)
        {
            CLEAN_IN_PROGRESS.store(true, std::sync::atomic::Ordering::Relaxed);
            let rt  = Runtime::new().unwrap();
            let settings = app_state.get_settings().await;
            tokio::task::block_in_place(move || 
            {
                let handle = rt.handle();
                let _ = handle.block_on(async move 
                {
                    let mut count = 0;
                    for t in &settings.tasks
                    {
                        if t.clean_types.len() > 0
                        {   
                            if let Some(dirs) = super::io::get_dirs(t.get_source_dir()) 
                            {
                                for d in &dirs
                                {
                                    let source_path = Path::new(t.get_source_dir()).join(d);
                                    //надо сконвертить внутренний пакет мэдо в универсальный транспортный пакет что лежит в transport
                                    let packet_info = PacketInfo::parse(&source_path);
                                    let packet = Packet::parse(&source_path, packet_info, t);
                                    if let Some(e) = packet.get_error()
                                    {
                                        let wrn = ["Директория ", d, " не является пакетом ", &e.1].concat();
                                        logger::warn!("{}", &wrn);
                                        if let Some(files) = get_files(&source_path)
                                        {
                                            if files.is_empty()
                                            {
                                                let _ = std::fs::remove_dir_all(&source_path);
                                                PacketTable::truncate(t.get_task_name(), &[d.to_owned()], app_state.get_db_pool()).await;
                                                let inf = ["В задаче ", t.get_task_name(), " удалена пустая директория ", &source_path.display().to_string()].concat();
                                                logger::info!("{}", inf);
                                                count+=1;
                                            }
                                        }
                                    }
                                    else 
                                    {
                                        if let Some(pt) = packet.get_packet_info().packet_type.as_ref()
                                        {
                                            if t.clean_types.contains(pt)
                                            {
                                                let _ = std::fs::remove_dir_all(&source_path);
                                                PacketTable::truncate(t.get_task_name(), &[d.to_owned()], app_state.get_db_pool()).await;
                                                let inf = ["Пакет ", &source_path.display().to_string(), " типа `", &pt, "` в задаче " , t.get_task_name(),"  удален"].concat();
                                                logger::info!("{}", inf);
                                                count+=1;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    CLEAN_IN_PROGRESS.store(false, std::sync::atomic::Ordering::Relaxed);
                    logger::info!("Очистка закончена, удалено {} пакетов", count);
                    WebsocketServer::clean_task_complete(count).await;
                });
            });
        }
        
    }
}


pub struct CopyService
{
    pub excludes_service : Box<dyn ExcludesTrait + Send + Sync>,
    pub packets_cleaner: PacketCleaner
}
impl CopyService
{
    pub fn new<T: ExcludesTrait + 'static + Send + Sync>(exclude_service: T) -> Self
    {
        Self 
        { 
            excludes_service: Box::new(exclude_service),
            packets_cleaner: PacketCleaner {} 
        }
    }
}

#[cfg(test)]
mod tests
{
    use settings::{FileMethods, Serializer, Settings};

    use crate::copyer::{CopyService};

    #[test]
    fn test_task_cleaner()
    {
        let _ = logger::StructLogger::new_default();
        // let s = CopyerService {
        //     excludes: Box::new(KeyValueStore::new())
        // };
        // let r = s.excludes.add("t1", "d1");
        // assert!(r.is_ok());
        // let d = s.excludes.delete("t1", "d1");
        // assert!(d.is_ok());
        // let cl = s.excludes.clear("t1");
        // assert!(cl.is_ok());
    }
}