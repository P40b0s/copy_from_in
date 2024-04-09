mod error;
mod ws;
mod api;
use api::start_http_server;
pub use error::Error;
use ws::{start_ws_server, start_new_packets_handler};
mod helpers;
mod copyer;
mod state;
mod commands;
use std::{default, future, net::SocketAddr, sync::Arc};
use anyhow::Result;
use copyer::{DirectoriesSpy, NewPacketInfo};
use logger::{debug, warn, StructLogger};
use state::AppState;
use once_cell::sync::Lazy;
use tokio::sync::OnceCell;
use crossbeam_channel::{Receiver, bounded};
use websocket_service::{Server, WebsocketMessage};
use clap::{arg, command, Parser};
use serializer::BytesSerializer;
extern crate async_channel;
static APP_STATE : Lazy<Arc<AppState>> = Lazy::new(|| Arc::new(AppState::default()));
#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(next_line_help = true)]
struct Cli 
{
  #[arg(long)]
  ws_port: usize,
  #[arg(long)]
  http_port: usize,
}


#[tokio::main(flavor = "multi_thread", worker_threads = 2)]
async fn main()
{
    StructLogger::initialize_logger();
    let (ws_port, http_port) =
    {
        if let Ok(cli) = Cli::try_parse()
        {
            (cli.ws_port, cli.http_port)
        }
        else
        {
            warn!("При запуске программы не обнаружены аргументы --ws_port и --http_port, будут использоваться агрументы для локального сервера (3010, 3009)");
            (3010, 3009)
        }
    };
    start_http_server(http_port).await;
    start_ws_server(ws_port).await;
    start_new_packets_handler().await;
    loop
    {
        let settings = Arc::clone(&APP_STATE);
        let _ = DirectoriesSpy::process_tasks(settings).await;
        tokio::time::sleep(tokio::time::Duration::from_secs(15)).await;
        //for testing
    }
}