use std::sync::Arc;

use logger::debug;
use settings::Task;
use tauri::plugin::{Builder, TauriPlugin};
use tauri::{App, Manager, Runtime, State};
use transport::Packet;
use crate::http_service;
use crate::state::AppState;
use crate::Error;

#[tauri::command]
pub async fn get(state: State<'_, Arc<AppState>>) -> Result<Vec<Task>, Error>
{
    debug!("Запрос списка тасков");
    let res = state.settings_service.get().await?;
    Ok(res)
}

#[tauri::command]
pub async fn update(payload: Task, state: State<'_, Arc<AppState>>) -> Result<(), Error>
{
    debug!("Попытка сохранить задачу {:?}", payload);
    let _ = state.settings_service.update(payload).await?;
    Ok(())
}

#[tauri::command]
pub async fn delete(payload: Task, state: State<'_, Arc<AppState>>) -> Result<(), Error>
{
    let _ = state.settings_service.delete(payload).await?;
    Ok(())
}


pub fn settings_plugin<R: Runtime>(app_state: Arc<AppState>) -> TauriPlugin<R> 
{
    Builder::new("settings")
      .invoke_handler(tauri::generate_handler![
        get,
        update,
        delete
        ])
        .setup(|app_handle| 
        {
            app_handle.manage(app_state);
            Ok(())
        })
      .build()
}