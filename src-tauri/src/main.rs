#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]
mod helpers;
mod error;
mod copyer;
use copyer::{DirectoriesSpy, NewPacketInfo};
use crossbeam_channel::{bounded, Sender};
pub use error::Error;
use std::{fmt::Display, sync::Arc, time::Duration};
pub use logger;
mod commands;
use commands::*;
mod state;
use state::AppState;
pub use const_format::concatcp;
use logger::StructLogger;
use once_cell::sync::OnceCell;
use tauri::Manager;
use tokio::sync::Mutex;

pub static NEW_DOCS: OnceCell<Mutex<Sender<NewPacketInfo>>> = OnceCell::new();

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

  let (new_doc_sender, new_doc_receiver) = bounded::<NewPacketInfo>(5);
  let _ = NEW_DOCS.set(Mutex::new(new_doc_sender));
  tauri::Builder::default()
  .manage(AppState::default())
  .setup(|app| 
    {
      let app_handle = Arc::new(app.handle());
      let st = app_handle.state::<AppState>().inner();
      let settings = st.get_settings();
      //новая арка на каждый асинхронный рантайм
      let handle_1 = Arc::clone(&app_handle);
      let handle_2 = Arc::clone(&app_handle);
      tauri::async_runtime::spawn(async move
      {
        DirectoriesSpy::initialize(&settings).await;
        loop 
        {
          let _ = DirectoriesSpy::process_tasks(Arc::clone(&handle_1)).await;
          tokio::time::sleep(tokio::time::Duration::from_secs(15)).await;
        }
      });
      tauri::async_runtime::spawn(async move
      {
        loop
        {
          if let Ok(packet) = new_doc_receiver.recv()
          {
            new_packet_found(packet, Arc::clone(&handle_2));
          }
        }
      });
      Ok(())
    })
    .plugin(commands::date_plugin())
    .plugin(commands::settings_plugin())
    .plugin(commands::service_plugin())
    // .invoke_handler(tauri::generate_handler![
    //   //initialize_app_state,
    // ])
    .run(tauri::generate_context!())
    .expect("Ошибка запуска приложения!");
}

fn new_packet_found<R: tauri::Runtime>(packet: NewPacketInfo, manager: Arc<impl Manager<R>>) 
{
  logger::info!("Поступил новый документ! {} {:?}", packet.get_packet_name(), &packet);
    manager
        .emit_all("new_packet_found", packet)
        .unwrap();
}
// fn event_to_front<R: tauri::Runtime, P: Serialize + Clone>(event: TauriEvent, payload: P, manager: Arc<impl Manager<R>>) 
// {
//     manager
//         .emit_all(&event.to_string(), payload)
//         .unwrap();
// }

// The Tauri command that gets called when Tauri `invoke` JavaScript API is
// called
// #[tauri::command]
// async fn js2rs(message: String, state: tauri::State<'_, AppState>) -> Result<(), String> 
// {
//   info!("{} js2rs", message);
//   Ok(())
// }

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