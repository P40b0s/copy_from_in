#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]
mod helpers;
mod error;
mod ws_serivice;
mod http_service;
mod emits;
mod cli;
mod nosleep;
pub use emits::TauriEmits;
pub use error::Error;
use state::AppState;
use ws_serivice::start_ws_service;
//use ws_serivice::start_ws_service;
use std::{sync::Arc};
pub use logger;
mod commands;
use commands::*;
mod state;
use logger::{debug, StructLogger};
use once_cell::sync::OnceCell;
use tauri::{AppHandle, Manager};
pub static HANDLE : OnceCell<Arc<AppHandle>> = OnceCell::new();

#[tokio::main]
async fn main() 
{
    let _ = StructLogger::new_default();
    nosleep::prevent_sleep().await;
    let args = cli::Cli::parse_or_default();
    let api_addr = args.api_addr();
    let ws_addr = args.ws_addr();
    debug!("api: {} ws: {}", &api_addr, ws_addr);
    let app_state = AppState
    {
       settings_service: http_service::SettingsService::new(&args.current_api_path()),
       utilites_service: http_service::UtilitesService::new(&args.current_api_path()),
       packet_service: http_service::PacketService::new(&args.current_api_path()),
    };
    let state = Arc::new(app_state);
    tauri::Builder::default()
    .setup(|app| 
        {
          let handle = Arc::new(app.app_handle());
          let _ = HANDLE.set(handle);
          tauri::async_runtime::spawn(async move 
          {
              start_ws_service(ws_addr).await;
          });
          Ok(())
        })
    .plugin(commands::date_plugin())
    .plugin(commands::settings_plugin(Arc::clone(&state)))
    .plugin(commands::service_plugin(Arc::clone(&state)))
    .plugin(commands::packets_plugin(Arc::clone(&state)))
    .manage(Arc::clone(&state))
    .run(tauri::generate_context!())
    .expect("Ошибка запуска приложения!");
}