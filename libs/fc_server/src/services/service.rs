use std::sync::Arc;
use db_service::SqlOperations;
use logger::debug;
use settings::Settings;
use transport::Packet;
use crate::copyer::PacketsCleaner;
use crate::db::PacketTable;
use crate::state::AppState;
use crate::Error;

use super::WebsocketServer;


pub async fn clean_packets(state: Arc<AppState>)
{
    Settings::clean_packets(state).await;
}

pub async fn truncate_tasks_excepts(state: Arc<AppState>) -> Result<u32, Error>
{
    let settings = state.get_settings().await;
    let r = settings.truncate_excludes();
    for (task, excludes) in r.1
    {
        PacketTable::truncate(&task, &excludes, state.get_db_pool()).await
    }
    WebsocketServer::need_packets_update().await;
    Ok(r.0)
}

pub async fn rescan_packet(packet: Packet, state: Arc<AppState>) -> Result<(), Error>
{
    debug!("Получен запрос на пересканирование пакета {}", packet.get_packet_name());
    Settings::del_exclude(packet.get_task(), packet.get_packet_name());
    PacketTable::delete_by_id(packet.get_id(), state.get_db_pool()).await;
    Ok(())
}

pub async fn delete_packet(packet: Packet, state: Arc<AppState>) -> Result<(), Error>
{
    debug!("Получен запрос на удаление пакета {}", packet.get_packet_name());
    Settings::del_exclude(packet.get_task(), packet.get_packet_name());
    PacketTable::delete_by_id(packet.get_id(), state.get_db_pool()).await;
    let del_dir = packet.get_task().get_source_dir().join(packet.get_packet_info().get_packet_dir());
    let _ = std::fs::remove_dir_all(&del_dir);
    Ok(())
}