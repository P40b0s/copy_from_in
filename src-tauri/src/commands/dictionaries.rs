use logger::error;
use serde::{Serialize, Deserialize};

use crate::helpers::{Date, DateTimeFormat, DateFormat};
use crate::models::{DiseaseType, User, Dictionary};
use crate::state::AppState;
use crate::{Error};
use tauri::{
    plugin::{Builder, TauriPlugin},
    Runtime,
  };


// #[tauri::command]
// pub async fn get_diseases_types() -> Result<Vec<DiseaseType>, Error>
// {
// 	let selector = Selector::new(&DiseasesTypesTable::full_select());
//     let r = DiseasesTypesTable::select(&selector).await?;
//     Ok(r)
// }

// #[tauri::command]
// pub async fn save_diseases_types(payload: Vec<DiseaseType>) -> Result<Vec<DiseaseType>, Error>
// {
//     logger::info!("{:?}", payload);
// 	for d in payload
//     {
//         DiseasesTypesTable::add_or_replace(&d).await?;
//     }
//     let selector = Selector::new(&DiseasesTypesTable::full_select());
//     let list = DiseasesTypesTable::select(&selector).await?;
//     Ok(list)
// }

// pub fn dictionaries_plugin<R: Runtime>() -> TauriPlugin<R> 
// {
//     Builder::new("dictionaries")
//       .invoke_handler(tauri::generate_handler![
//         get_diseases_types,
//         save_diseases_types
//         ])
//       .build()
// }