
use logger::debug;
use settings::Task;
use tauri::plugin::{Builder, TauriPlugin};
use tauri::Runtime;
use transport::{Packet, Pagination};
use crate::http_service;
use crate::Error;


#[tauri::command]
pub async fn get(Pagination {row, offset} : Pagination) -> Result<Vec<Packet>, Error>
{
    logger::info!("pagination row:{} offset:{}", row,offset);
    let users = PacketTable::get_users_with_offset(row, offset, None).await?;
    Ok(users)
}
#[tauri::command]
pub async fn get_packets_list2() -> Result<Vec<Packet>, Error>
{
    http_service::get::<Vec<Packet>>("packets/list").await
}


pub fn packets_plugin<R: Runtime>() -> TauriPlugin<R> 
{
    Builder::new("packets")
      .invoke_handler(tauri::generate_handler![
        //get,
        get_packets_list2
        ])
      .build()
}
