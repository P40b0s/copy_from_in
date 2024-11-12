use std::sync::Arc;
use logger::debug;
use transport::Packet;
use crate::copyer::{CopyService, ExcludesTrait, PacketCleaner};
use crate::db::PacketTable;
use crate::state::AppState;
use crate::Error;

use super::WebsocketServer;


pub async fn clean_packets(state: Arc<AppState>)
{
    let service = state.get_copy_service();
    service.packets_cleaner.clean_packets(state).await;
    //Settings::clean_packets(state).await;
}

pub async fn truncate_tasks_excepts(state: Arc<AppState>) -> Result<u32, Error>
{
    let settings = state.get_settings().await;
    let service = state.get_copy_service();
    let result = service.excludes_service.truncate(&settings.tasks).await?;
    WebsocketServer::need_packets_update().await;
    Ok(result as u32)
}

pub async fn rescan_packet(packet: Packet, state: Arc<AppState>) -> Result<(), Error>
{
    debug!("Получен запрос на пересканирование пакета {}", packet.get_packet_name());
    let service = state.get_copy_service();
    let _ = service.excludes_service.delete(packet.get_task().get_task_name(), packet.get_packet_name());
    PacketTable::delete_by_id(packet.get_id(), state.get_db_pool()).await;
    Ok(())
}

pub async fn delete_packet(packet: Packet, state: Arc<AppState>) -> Result<(), Error>
{
    debug!("Получен запрос на удаление пакета {}", packet.get_packet_name());
    let service = state.get_copy_service();
    let _ = service.excludes_service.delete(packet.get_task().get_task_name(), packet.get_packet_name());
    PacketTable::delete_by_id(packet.get_id(), state.get_db_pool()).await;
    let del_dir = packet.get_task().get_source_dir().join(packet.get_packet_info().get_packet_dir());
    let _ = std::fs::remove_dir_all(&del_dir);
    Ok(())
}