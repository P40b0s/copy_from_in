use logger::debug;
use settings::Task;
use tauri::plugin::{Builder, TauriPlugin};
use tauri::Runtime;
use transport::Packet;
use crate::http_service;
use crate::Error;

#[tauri::command]
pub async fn get() -> Result<Vec<Task>, Error>
{
    http_service::get::<Vec<Task>>("settings/tasks").await
}

#[tauri::command]
pub async fn update(payload: Task) -> Result<(), Error>
{
    debug!("Попытка сохранить задачу {:?}", payload);
    http_service::post("settings/tasks/update", &payload).await
}

#[tauri::command]
pub async fn delete(payload: Task) -> Result<(), Error>
{
    http_service::post("settings/tasks/delete", &payload).await
}

#[tauri::command]
pub async fn get_packets_list() -> Result<Vec<Packet>, Error>
{
    http_service::get::<Vec<Packet>>("packets/list").await
}




pub fn settings_plugin<R: Runtime>() -> TauriPlugin<R> 
{
    Builder::new("settings")
      .invoke_handler(tauri::generate_handler![
        get,
        update,
        delete,
        get_packets_list
        ])
      .build()
}