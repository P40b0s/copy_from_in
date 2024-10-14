use std::sync::Arc;

//use crate::ws_serivice::WebsocketClient;
use service::Client;
use tauri::plugin::{Builder, TauriPlugin};
use tauri::{Manager, Runtime, State};
use transport::Packet;
use crate::state::AppState;
use crate::ws_serivice::WebsocketClient;
use crate::Error;
use crate::http_service;

#[tauri::command]
pub async fn clear_dirs(state: State<'_, Arc<AppState>>) -> Result<(), Error>
{
  let _ = state.utilites_service.clear_dirs().await;
  Ok(())
}

#[tauri::command]
pub async fn truncate_tasks_excepts(state: State<'_, Arc<AppState>>) -> Result<u32, Error>
{
  let res = state.utilites_service.truncate_tasks_excepts().await?;
  Ok(res)
}

#[tauri::command]
pub async fn ws_server_online(state: tauri::State<'_, Arc<AppState>>) -> Result<bool, Error>
{
  Ok(WebsocketClient::is_connected().await)
}
#[tauri::command]
pub async fn rescan_packet(payload: Packet, state: State<'_, Arc<AppState>>) -> Result<(), Error>
{
  //http_service::post::<Packet>("packets/rescan", &payload).await
  let _ = state.utilites_service.rescan_packet(payload).await?;
  Ok(())
}

#[tauri::command]
pub async fn delete_packet(payload: Packet, state: State<'_, Arc<AppState>>) -> Result<(), Error>
{
  let _ = state.utilites_service.delete_packet(payload).await?;
  Ok(())
}

pub fn service_plugin<R: Runtime>(app_state: Arc<AppState>) -> TauriPlugin<R> 
{
  Builder::new("service")
    .invoke_handler(tauri::generate_handler![
      clear_dirs,
      ws_server_online,
      rescan_packet,
      truncate_tasks_excepts,
      delete_packet,
      ])
    .setup(|app_handle| 
      {
          app_handle.manage(app_state);
          Ok(())
      })
    .build()
}