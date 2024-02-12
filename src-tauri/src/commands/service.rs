use logger::error;
use serde::{Serialize, Deserialize};
use settings::{FileMethods, Settings, Task};
use tauri::plugin::{Builder, TauriPlugin};
use tauri::Runtime;
use uuid::Uuid;
use crate::copyer::PacketsCleaner;
use crate::helpers::{Date, DateTimeFormat, DateFormat};
use crate::state::AppState;
use crate::Error;

#[tauri::command]
pub async fn clear_dirs(state: tauri::State<'_, AppState>) -> Result<u32, Error>
{
  let settings = state.get_settings();
  let r = Settings::clear_packets(&settings)?;
  Ok(r)
}
#[tauri::command]
pub async fn truncate_tasks_excepts(state: tauri::State<'_, AppState>) -> Result<u32, Error>
{
    let settings = state.get_settings();
    let r = settings.truncate_excludes();
    Ok(r)
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