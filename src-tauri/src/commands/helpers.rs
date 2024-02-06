use logger::error;
use serde::{Serialize, Deserialize};
use tauri::plugin::{Builder, TauriPlugin};
use tauri::Runtime;
use crate::helpers::{Date, DateTimeFormat, DateFormat};
use crate::models::{DiseaseType, User, Dictionary};
use crate::state::AppState;
use crate::{Error};


#[tauri::command]
pub async fn get_date_now() -> Result<String, Error>
{
    let date = Date::now();
    Ok(date.write(DateFormat::Serialize))
}
// #[tauri::command]
// pub async fn initialize_app_state(state: tauri::State<'_, AppState>) -> Result<FrontendStateUpdater, Error>
// {
//     let updater = FrontendStateUpdater::update_from_command(state).await?;
//     Ok(updater)
// }

pub fn date_plugin<R: Runtime>() -> TauriPlugin<R> 
{
    Builder::new("date")
      .invoke_handler(tauri::generate_handler![
        get_date_now
        ])
      .build()
}