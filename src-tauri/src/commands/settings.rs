use std::path::Path;
use logger::{debug, error};
use serde::{Serialize, Deserialize};
use settings::{FileMethods, Settings, Task};
use tauri::plugin::{Builder, TauriPlugin};
use tauri::Runtime;
use crate::http;
use crate::Error;

#[tauri::command]
pub async fn get() -> Result<Vec<Task>, Error>
{
    http::get::<Vec<Task>>("settings/tasks").await
}

#[tauri::command]
pub async fn update(payload: Task) -> Result<(), Error>
{
    debug!("Попытка сохранить задачу {:?}", payload);
    http::post("settings/tasks/update", &payload).await
}

#[tauri::command]
pub async fn delete(payload: Task) -> Result<(), Error>
{
    http::post("settings/tasks/delete", &payload).await
}


pub fn settings_plugin<R: Runtime>() -> TauriPlugin<R> 
{
    Builder::new("settings")
      .invoke_handler(tauri::generate_handler![
        get,
        update,
        delete
        ])
      .build()
}