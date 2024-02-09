use tauri::plugin::{Builder, TauriPlugin};
use tauri::Runtime;
use crate::helpers::{Date, DateTimeFormat, DateFormat};
use crate::Error;

#[tauri::command]
pub async fn get_date_now() -> Result<String, Error>
{
    let date = Date::now();
    Ok(date.write(DateFormat::Serialize))
}

pub fn date_plugin<R: Runtime>() -> TauriPlugin<R> 
{
    Builder::new("date")
      .invoke_handler(tauri::generate_handler![
        get_date_now
        ])
      .build()
}