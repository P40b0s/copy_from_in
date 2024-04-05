use tauri::plugin::{Builder, TauriPlugin};
use tauri::Runtime;
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

pub fn service_plugin<R: Runtime>() -> TauriPlugin<R> 
{
  Builder::new("service")
    .invoke_handler(tauri::generate_handler![
      clear_dirs,
      truncate_tasks_excepts,
      ])
    .build()
}