
use std::sync::Arc;

use logger::debug;
use settings::Task;
use tauri::plugin::{Builder, TauriPlugin};
use tauri::{Manager, Runtime, State};
use transport::{Packet, Pagination};
use crate::http_service;
use crate::state::AppState;
use crate::Error;


#[tauri::command]
pub async fn get_packets_list(Pagination {row, offset} : Pagination, state: State<'_, Arc<AppState>>) -> Result<Vec<Packet>, Error>
{
    let res = state.packet_service.get(Pagination {row, offset}).await?;
    Ok(res)
}
#[tauri::command]
pub async fn search_packets(payload: &str, state: State<'_, Arc<AppState>>) -> Result<Vec<Packet>, Error>
{
    debug!("Поиск строки:{}", payload);
    let res = state.packet_service.search(payload).await?;
    Ok(res)
    //let packets = http_service::get::<Vec<Packet>>("packets", &Pagination {row, offset}).await?;
    //let users = PacketTable::get_users_with_offset(row, offset, None).await?;
    //Ok(users)
}
#[tauri::command]
pub async fn get_count(state: State<'_, Arc<AppState>>) -> Result<u32, Error>
{
    let res = state.packet_service.count().await?;
    Ok(res)
}

pub fn packets_plugin<R: Runtime>(app_state: Arc<AppState>) -> TauriPlugin<R> 
{
    Builder::new("packets")
      .invoke_handler(tauri::generate_handler![
        get_packets_list,
        search_packets,
        get_count
        ])
        .setup(|app_handle| 
        {
            app_handle.manage(app_state);
            Ok(())
        })
      .build()
}
