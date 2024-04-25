mod error;
mod ws;
mod api;
mod cli;
use api::start_http_server;
pub use error::Error;
pub use utilites;
use ws::{start_ws_server, start_new_packets_handler};
pub use ws::WebsocketServer;
mod copyer;
mod state;
mod commands;
use std::sync::Arc;
use copyer::DirectoriesSpy;
use logger::StructLogger;
use state::AppState;
use once_cell::sync::Lazy;

extern crate async_channel;
static APP_STATE : Lazy<Arc<AppState>> = Lazy::new(|| Arc::new(AppState::default()));


#[tokio::main(flavor = "multi_thread", worker_threads = 2)]
async fn main()
{
    StructLogger::initialize_logger();
    let params = cli::Cli::parse_args();
    let _ = start_http_server(params.http_port).await;
    start_ws_server(params.ws_port).await;
    start_new_packets_handler().await;
    loop
    {
        let settings = Arc::clone(&APP_STATE);
        let _ = DirectoriesSpy::process_tasks(settings).await;
        tokio::time::sleep(tokio::time::Duration::from_secs(15)).await;
        //for testing
    }
}