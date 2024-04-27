use std::sync::Arc;
use settings::Settings;
use transport::Packet;
use crate::copyer::PacketsCleaner;
use crate::state::AppState;
use crate::Error;

pub async fn clear_dirs(state: Arc<AppState>) -> Result<u32, Error>
{
  let settings = state.get_settings().await;
  let r = Settings::clear_packets(&settings)?;
  Ok(r)
}

pub async fn truncate_tasks_excepts(state: Arc<AppState>) -> Result<u32, Error>
{
    let settings = state.get_settings().await;
    let r = settings.truncate_excludes();
    Ok(r)
}

pub async fn rescan_packet(packet: Packet, state: Arc<AppState>) -> Result<(), Error>
{
    //let settings = state.get_settings().await;
    Settings::del_exclude(packet.get_task(), packet.get_packet_name());
    Ok(())
}