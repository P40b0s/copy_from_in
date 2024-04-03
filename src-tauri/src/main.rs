#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]
mod helpers;
mod error;
use clap::{arg, command, Parser};
//mod copyer;
//use copyer::{DirectoriesSpy, NewPacketInfo};
use crossbeam_channel::{bounded, Sender};
pub use error::Error;
use settings::Settings;
use websocket_service::{Client, WebsocketMessage};
use std::{fmt::Display, sync::Arc, time::Duration};
pub use logger;
mod commands;
use commands::*;
mod state;
use state::AppState;
pub use const_format::concatcp;
use logger::{warn, StructLogger};
use once_cell::sync::OnceCell;
use tauri::Manager;
use tokio::sync::Mutex;


#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(next_line_help = true)]
struct Cli 
{
  #[arg(long)]
  server: String,
  #[arg(long)]
  port: String,
}



#[tokio::main]
async fn main() 
{
  StructLogger::initialize_logger();
  let (ip, port) =
  {
    if let Ok(cli) = Cli::try_parse()
    {
      (cli.server, cli.port)
    }
    else
    {
      warn!("При запуске программы не обнаружены аргументы --server и --port, будут использоваться агрументы для локального сервера");
      ("127.0.0.1".to_owned(), "3010".to_owned())
    }
  };
  let addr = ["ws://", &ip, ":", &port, "/"].concat();
  Client::start_client(&addr).await;
 
  tauri::Builder::default()
  .manage(AppState 
  {
    settings: Mutex::new(Settings::default())
  })
  .setup(|app| 
    {
      let app_handle = Arc::new(app.handle());
      //let st = app_handle.state::<AppState>().inner();
      //новая арка на каждый асинхронный рантайм
      let handle_1 = Arc::clone(&app_handle);
      let handle_2 = Arc::clone(&app_handle);
      tauri::async_runtime::spawn(async move
      {
        Client::on_receive_message(|msg|
        {
          async 
          {
            let handle = Arc::clone(&handle_1);
            match msg.command.get_target() 
            {
              "settings" => 
              {
                match msg.command.get_method()
                {
                  "reload" => 
                  {
                    let new_settings = msg.command.extract_payload::<Settings>().unwrap();
                    let stt = handle.state::<AppState>().settings.lock().await;
                    *stt = new_settings;
                    //TODO послать эмит в фронтэенд с новыми настройками
                  }
                  _ => ()
                }
              }
              _ => ()
            }
        };
        }).await;
        let init_msg = WebsocketMessage::new("event", "on_client_connected", None);
        Client::send_message(&init_msg).await;
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

// pub enum TauriEvent
// {
//   Test,
//   UpdateState,
//   UpdateDate,
//   UpdateUsers
// }

// impl Display for TauriEvent
// {
//   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result 
//   {
//       match &self
//       {
//         TauriEvent::Test => f.write_str("test"),
//         TauriEvent::UpdateState => f.write_str("update_state"),
//         TauriEvent::UpdateDate => f.write_str("update_date"),
//         TauriEvent::UpdateUsers => f.write_str("update_users"),
//       }
//   }
// }