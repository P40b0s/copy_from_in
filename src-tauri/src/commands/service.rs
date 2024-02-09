use logger::error;
use serde::{Serialize, Deserialize};
use settings::{FileMethods, Settings, Task};
use tauri::plugin::{Builder, TauriPlugin};
use tauri::Runtime;
use uuid::Uuid;
use crate::helpers::{Date, DateTimeFormat, DateFormat};
use crate::state::AppState;
use crate::Error;

#[tauri::command]
pub async fn clear_dirs(state: tauri::State<'_, AppState>) -> Result<(), Error>
{
  let settings = state.get_settings();
  Ok(())
}
#[tauri::command]
pub async fn clear_tasks(state: tauri::State<'_, AppState>) -> Result<(), Error>
{
    let settings = state.get_settings();
    Ok(())
}

pub fn service_plugin<R: Runtime>() -> TauriPlugin<R> 
{
    Builder::new("service")
      .invoke_handler(tauri::generate_handler![
        clear_dirs,
        clear_tasks
        ])
      .build()
}