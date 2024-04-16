use service::Client;
use tauri::plugin::{Builder, TauriPlugin};
use tauri::Runtime;
use transport::{Contract, NewPacketInfo};
use crate::Error;
use crate::http;

#[tauri::command]
pub async fn clear_dirs() -> Result<u32, Error>
{
  http::get::<u32>("packets/clean").await
}

#[tauri::command]
pub async fn truncate_tasks_excepts() -> Result<u32, Error>
{
  http::get::<u32>("packets/truncate").await
}

#[tauri::command]
pub async fn ws_server_online() -> Result<bool, Error>
{
  Ok(Client::<Contract>::is_connected())
}
#[tauri::command]
pub async fn rescan_packet(packet: NewPacketInfo) -> Result<(), Error>
{
  http::post::<NewPacketInfo>("packets/rescan", &packet).await
}

pub fn service_plugin<R: Runtime>() -> TauriPlugin<R> 
{
  Builder::new("service")
    .invoke_handler(tauri::generate_handler![
      clear_dirs,
      ws_server_online,
      rescan_packet,
      truncate_tasks_excepts,
      ])
    .build()
}