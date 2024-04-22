use crate::ws_serivice::WebsocketClient;
use service::Client;
use tauri::plugin::{Builder, TauriPlugin};
use tauri::Runtime;
use transport::{Contract, NewPacketInfo};
use crate::Error;
use crate::http_service;

#[tauri::command]
pub async fn clear_dirs() -> Result<u32, Error>
{
  http_service::get::<u32>("packets/clean").await
}

#[tauri::command]
pub async fn truncate_tasks_excepts() -> Result<u32, Error>
{
  http_service::get::<u32>("packets/truncate").await
}

#[tauri::command]
pub async fn ws_server_online() -> Result<bool, Error>
{
  Ok(WebsocketClient::is_connected())
}
#[tauri::command]
pub async fn rescan_packet(payload: NewPacketInfo) -> Result<(), Error>
{
  http_service::post::<NewPacketInfo>("packets/rescan", &payload).await
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