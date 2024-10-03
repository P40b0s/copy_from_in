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
pub use const_format::concatcp;
use logger::{debug, StructLogger};
use once_cell::sync::OnceCell;
use tauri::{AppHandle, Manager};
pub static HANDLE : OnceCell<Arc<AppHandle>> = OnceCell::new();

#[tokio::main]
async fn main() 
{
    StructLogger::new_default();
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

// fn new_packet_found<R: tauri::Runtime>(packet: NewPacketInfo, manager: Arc<impl Manager<R>>) 
// {
//   logger::info!("Поступил новый документ! {} {:?}", packet.get_packet_name(), &packet);
//     manager
//         .emit_all("new_packet_found", packet)
//         .unwrap();
// }


//оставлю на память
// tauri::Builder::default()
//   .manage(AppState 
//   {
//     settings: Mutex::new(Settings::default())
//   })
//   .setup(|app| 
//     {
//       let app_handle = Arc::new(app.handle());
//       //let st = app_handle.state::<AppState>().inner();
//       //новая арка на каждый асинхронный рантайм
//       let handle_1 = Arc::clone(&app_handle);
//       let handle_2 = Arc::clone(&app_handle);
//       tauri::async_runtime::spawn(async move
//       {
//         Client::on_receive_message(|msg|
//         {
//           async 
//           {
//             let handle = Arc::clone(&handle_1);
//             match msg.command.get_target() 
//             {
//               "settings" => 
//               {
//                 match msg.command.get_method()
//                 {
//                   "reload" => 
//                   {
//                     let new_settings = msg.command.extract_payload::<Settings>().unwrap();
//                     let stt = handle.state::<AppState>().settings.lock().await;
//                     *stt = new_settings;
//                     //TODO послать эмит в фронтэенд с новыми настройками
//                   }
//                   _ => ()
//                 }
//               }
//               _ => ()
//             }
//         };
//         }).await;
//         let init_msg = WebsocketMessage::new("event", "on_client_connected", None);
//         Client::send_message(&init_msg).await;
//       });
//       tauri::async_runtime::spawn(async move
//       {
//         loop
//         {
//           if let Ok(packet) = new_doc_receiver.recv()
//           {
//             new_packet_found(packet, Arc::clone(&handle_2));
//           }
//         }
//       });
//       Ok(())
//     })
//     .plugin(commands::date_plugin())
//     .plugin(commands::settings_plugin())
//     .plugin(commands::service_plugin())
//     // .invoke_handler(tauri::generate_handler![
//     //   //initialize_app_state,
//     // ])
//     .run(tauri::generate_context!())
//     .expect("Ошибка запуска приложения!");