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
pub async fn get(state: tauri::State<'_, AppState>) -> Result<Vec<Task>, Error>
{
    let settings = state.get_settings();
    Ok(settings.tasks)
}

#[tauri::command]
pub async fn update(payload: Vec<Task>, state: tauri::State<'_, AppState>) -> Result<(), Error>
{
    logger::debug!("Запрос сохранения настроек {:?}", &payload);
    let settings = Settings 
    {
        tasks: payload.clone()
    };
    let save_state = settings.save(settings::Serializer::Toml).map_err(|e| Error::SettingsValidation(e));
    if let Err(e) = save_state.as_ref()
    {
        error!("Ошибка сохранения настроек! {}", &e.to_string());
        save_state?
    }
    let mut s = state.settings.lock().unwrap();
    s.tasks = payload;

    Ok(())
}
#[tauri::command]
pub async fn update_with_gen_excludes(payload: Task, state: tauri::State<'_, AppState>) -> Result<(), Error>
{
   
    let mut sett = state.settings.lock().unwrap();
    if let Some(t) = sett.tasks.iter_mut().find(|f| &f.name == &payload.name)
    {
        *t = payload;
    }
    let save_state = sett.save(settings::Serializer::Toml).map_err(|e| Error::SettingsValidation(e));
    if let Err(e) = save_state.as_ref()
    {
        error!("Ошибка сохранения настроек! {}", &e.to_string());
        save_state?
    }
    let mut s = state.settings.lock().unwrap();
    s.tasks = payload;

    Ok(())
}


pub fn settings_plugin<R: Runtime>() -> TauriPlugin<R> 
{
    Builder::new("settings")
      .invoke_handler(tauri::generate_handler![
        get,
        update,
        update_with_gen_excludes
        ])
      .build()
}