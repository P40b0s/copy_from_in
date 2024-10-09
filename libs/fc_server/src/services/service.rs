use std::sync::Arc;
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

pub async fn rescan_packet(packet: Packet, _state: Arc<AppState>) -> Result<(), Error>
{
    //let settings = state.get_settings().await;
    Settings::del_exclude(packet.get_task(), packet.get_packet_name());
    Ok(())
}