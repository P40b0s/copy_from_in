use logger::error;
use serde::{Serialize, Deserialize};

use crate::helpers::{Date, DateTimeFormat, DateFormat};
use crate::models::{Dictionary, Disease, DiseaseType, Status, User};
use crate::state::AppState;
use crate::{ Error};
use tauri::{
    plugin::{Builder, TauriPlugin},
    Runtime,
  };

// #[tauri::command]
// pub async fn update_diseases(payload: Vec<Disease>, user_id : &str, state: tauri::State<'_, AppState>) -> Result<FrontendStateUpdater, Error>
// {
//     if payload.len() == 0
//     {
//         let _ = DiseasesTable::delete_by_user_id(user_id).await?;
//     }
//     else
//     {
//         let exists: Vec<String> = payload.iter().map(|m|m.id.clone()).collect();
//         let _ = DiseasesTable::delete_many_exclude_ids(exists, Some(user_id)).await?;
//         for d in &payload
//         {
//             let _ = DiseasesTable::add_or_replace(d).await?;
//         }
//     }
//     let updater = FrontendStateUpdater::update_from_command(state).await?;
//     Ok(updater)
// }

// #[tauri::command]
// pub async fn update_statuses(payload: Vec<Status>, user_id : &str, state: tauri::State<'_, AppState>) -> Result<FrontendStateUpdater, Error>
// {
//     if payload.len() == 0
//     {
//         let _ = StatusesTable::delete_by_user_id(user_id).await?;
//     }
//     else
//     {
//         let exists: Vec<String> = payload.iter().map(|m|m.id.clone()).collect();
//         let _ = StatusesTable::delete_many_exclude_ids(exists, Some(user_id)).await?;
//         for d in &payload
//         {
//             let _ = StatusesTable::add_or_replace(d).await?;
//         }
//     }
//     let updater = FrontendStateUpdater::update_from_command(state).await?;
//     Ok(updater)
// }



// pub fn statuses_plugin<R: Runtime>() -> TauriPlugin<R> 
// {
//     Builder::new("statuses")
//       .invoke_handler(tauri::generate_handler![
//        update_diseases,
//        update_statuses,
//         ])
//       .build()
// }