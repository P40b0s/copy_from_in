#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]
mod helpers;
mod error;
mod ws_serivice;
mod http;
use clap::{arg, command, Parser};
pub use error::Error;
use http::initialize_http_requests;
use ws_serivice::start_ws_service;
use std::{sync::Arc};
pub use logger;
mod commands;
use commands::*;
mod state;
pub use const_format::concatcp;
use logger::{debug, warn, StructLogger};
use once_cell::sync::OnceCell;
use tauri::AppHandle;


#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(next_line_help = true)]
struct Cli 
{
  #[arg(long)]
  host: String,
  #[arg(long)]
  ws_port: usize,
  #[arg(long)]
  api_port: usize,
}
impl Default for Cli
{
  fn default() -> Self 
  {
    Self { host: "127.0.0.1".to_owned(), api_port: 3009, ws_port: 3010}
  }
}


static HANDLE : OnceCell<Arc<AppHandle>> = OnceCell::new();

#[tokio::main]
async fn main() 
{
  StructLogger::initialize_logger();
  let args =
  {
    let parsed = Cli::try_parse();
    if let Ok(cli) = parsed
    {
      cli
    }
    else
    {
      warn!("При запуске программы не обнаружены аргументы --server --ws_port и --api_port, будут использоваться агрументы для локального сервера -> {}", parsed.err().unwrap().to_string());
      Cli::default()
    }
  };

  
  let api_addr = [&args.host, ":", &args.api_port.to_string()].concat();
  let ws_addr = ["ws://", &args.host, ":", &args.ws_port.to_string(), "/"].concat();
  debug!("api: {} ws: {}", &api_addr, ws_addr);
  //start_ws_service2(ws_addr).await;
  initialize_http_requests(api_addr);
  tauri::Builder::default()
  .setup(|app| 
    {
      let handle = Arc::new(app.handle());
      tauri::async_runtime::spawn(async move 
      {
        start_ws_service(ws_addr, handle).await;
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