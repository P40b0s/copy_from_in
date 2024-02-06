use logger::error;
use serde::{Serialize, Deserialize};
use tauri::plugin::{Builder, TauriPlugin};
use tauri::Runtime;
use uuid::Uuid;

use crate::helpers::{Date, DateTimeFormat, DateFormat};
use crate::models::{DiseaseType, User, Dictionary};
use crate::state::AppState;
use crate::{Error};

#[derive(Debug, Serialize, Deserialize)]
pub struct Pagination
{
    pub row: u32,
    pub offset: u32
}
// #[tauri::command]
// pub async fn get(Pagination {row, offset} : Pagination) -> Result<Vec<User>, Error>
// {
//     logger::info!("pagination row:{} offset:{}", row,offset);
//     let users = UsersTable::get_users_with_offset(row, offset, None).await?;
//     Ok(users)
// }

// #[tauri::command]
// pub async fn add_or_update(payload: User) -> Result<User, Error>
// {
//     //let mut pl = payload;
//     //pl.id = Uuid::new_v4().to_string();
//     let _ = UsersTable::add_or_replace(&payload).await?;
//     logger::info!("Добавлен пользователь {}", payload.full_name());
//     Ok(payload)
// }

// pub fn users_plugin<R: Runtime>() -> TauriPlugin<R> 
// {
//     Builder::new("users")
//       .invoke_handler(tauri::generate_handler![
//         get,
//         add_or_update,
//         ])
//       .build()
// }