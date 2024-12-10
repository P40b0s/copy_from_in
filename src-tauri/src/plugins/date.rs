use std::sync::Arc;

use tauri::plugin::{Builder, TauriPlugin};
use tauri::Runtime;
use crate::helpers::{Date, DateTimeFormat, DateFormat};
use crate::state::AppState;
use crate::Error;

#[tauri::command]
async fn get_date_now() -> Result<String, Error>
{
    let date = Date::now();
    Ok(date.write(DateFormat::Serialize))
}

pub struct DatePlugin{}
impl super::Plugin for DatePlugin
{
    const NAME: &str = "date";
    fn build<R: Runtime>(_app_state: Arc<AppState>) -> TauriPlugin<R> 
    {
        Builder::new(Self::NAME)
        .invoke_handler(tauri::generate_handler![
            get_date_now
            ])
        .build()
    }
}
