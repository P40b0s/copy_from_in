use std::sync::Arc;

use tauri::AppHandle;
use tokio::sync::Mutex;
use settings::{FileMethods, Settings};

use crate::http_service::{SettingsService, UtilitesService, PacketService};

pub struct AppState
{
    //pub settings: Mutex<Settings>,
    pub settings_service: SettingsService,
    pub utilites_service: UtilitesService,
    pub packet_service: PacketService,
}


impl AppState
{
    // pub async fn get_settings(&self) -> Settings
    // {
    //     let guard = self.settings.lock().await;
    //     guard.clone()
    // }
}
