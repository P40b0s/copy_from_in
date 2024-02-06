#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]
mod models;
mod helpers;
mod file;
mod error;
mod copyer;
use crossbeam_channel::{bounded, Sender};
pub use error::Error;
use serde::Serialize;
use settings::FileMethods;
use state_updater::{DateState, StateUpdater};
mod state;
mod state_updater;
use std::{sync::{Arc, atomic::AtomicU32}, ops::Deref, fmt::Display, process::exit};

use helpers::{Date, DateTimeFormat};
pub use logger;
mod commands;
use commands::*;
pub use const_format::concatcp;
use file::*;
use logger::{info, LevelFilter, StructLogger};
use once_cell::sync::{OnceCell};
use state::AppState;
use tauri::Manager;
use tokio::{sync::Mutex };

pub static LOG_SENDER: OnceCell<Mutex<Sender<(LevelFilter, String)>>> = OnceCell::new();


fn main() 
{
  StructLogger::initialize_logger();
  // tauri::async_runtime::spawn(async move 
  // {
  //   let i  = repository::initialize().await;
  //   if i.is_err()
  //   {
  //     logger::error!("{} Выход из программы с кодом 11!", i.err().unwrap());
  //     exit(11);
  //   }
  // });
  //let (async_proc_input_tx, async_proc_input_rx) = mpsc::channel(1);
  let (log_sender, log_receiver) = bounded::<(LevelFilter, String)>(5);
  let _ = LOG_SENDER.set(Mutex::new(log_sender));

  tauri::Builder::default()
  .manage(AppState::default())
  .setup(|app| 
    {
      let app_handle = Arc::new(app.handle());
      //новая арка на каждый асинхронный рантайм
      let handle_1 = Arc::clone(&app_handle);
      let handle_2 = Arc::clone(&app_handle);
      let handle_3 = Arc::clone(&app_handle);
      tauri::async_runtime::spawn(async move 
      {
          loop 
          {
            if let Ok(output) = log_receiver.recv()
            {
              //надо разделить на ошибки предупрежедения итд
              logger::info!("Приехало сообщение от directories_spy! {}", output.1);
              //rs2js(output, Arc::clone(&handle_1));
            }
          }
      });
      tauri::async_runtime::spawn(async move 
      {
          loop 
          {
            let _ = DateState::update_from_thread(Arc::clone(&handle_2)).await;
            tokio::time::sleep(tokio::time::Duration::new(60, 0)).await;
          }
      });
      tauri::async_runtime::spawn(async move 
        {
            loop 
            {
              let _ = DateState::update_from_thread(Arc::clone(&handle_3)).await;
              tokio::time::sleep(tokio::time::Duration::new(60, 0)).await;
            }
        });
      Ok(())
    })
    //.plugin(commands::dictionaries_plugin())
    //.plugin(commands::users_plugin())
    .plugin(commands::date_plugin())
    //.plugin(commands::statuses_plugin())
    //.invoke_handler(tauri::generate_handler![
    //  initialize_app_state,
    //  ])
    .run(tauri::generate_context!())
    .expect("Ошибка запуска приложения!");
}

fn rs2js<R: tauri::Runtime>(message: String, manager: Arc<impl Manager<R>>) 
{
    info!("{} rs2js",message);
    manager
        .emit_all("rs2js", message)
        .unwrap();
}
fn event_to_front<R: tauri::Runtime, P: Serialize + Clone>(event: TauriEvent, payload: P, manager: Arc<impl Manager<R>>) 
{
    manager
        .emit_all(&event.to_string(), payload)
        .unwrap();
}

// The Tauri command that gets called when Tauri `invoke` JavaScript API is
// called
#[tauri::command]
async fn js2rs(message: String, state: tauri::State<'_, AppState>) -> Result<(), String> 
{
  info!("{} js2rs", message);
  Ok(())
}

pub enum TauriEvent
{
  Test,
  UpdateState,
  UpdateDate,
  UpdateUsers
}

impl Display for TauriEvent
{
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result 
  {
      match &self
      {
        TauriEvent::Test => f.write_str("test"),
        TauriEvent::UpdateState => f.write_str("update_state"),
        TauriEvent::UpdateDate => f.write_str("update_date"),
        TauriEvent::UpdateUsers => f.write_str("update_users"),
      }
  }
}

