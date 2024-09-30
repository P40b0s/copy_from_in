
use std::sync::Arc;

use logger::debug;
use settings::Task;
use tauri::plugin::{Builder, TauriPlugin};
use tauri::{Manager, Runtime};
use transport::{Packet, Pagination};
use crate::http_service;
use crate::state::AppState;
use crate::Error;


#[tauri::command]
pub async fn get(state: State<'_, AppState>, Pagination {row, offset} : Pagination) -> Result<Vec<Packet>, Error>
{
    logger::info!("pagination row:{} offset:{}", row,offset);
    let res = state.packet_service.
    //let packets = http_service::get::<Vec<Packet>>("packets", &Pagination {row, offset}).await?;
    //let users = PacketTable::get_users_with_offset(row, offset, None).await?;
    //Ok(users)
}

// #[tauri::command]
// pub async fn get_packets_list2(state: State<'_, AppState>) -> Result<Vec<Packet>, Error>
// {
//     //http_service::get::<Vec<Packet>>("packets/list").await
//     
// }


pub fn packets_plugin<R: Runtime>(app_state: Arc<AppState>) -> TauriPlugin<R> 
{
    Builder::new("packets")
      .invoke_handler(tauri::generate_handler![
        get,
        //get_packets_list2
        ])
        .setup(|app_handle| 
        {
            app_handle.manage(app_state);
            Ok(())
        })
      .build()
}
