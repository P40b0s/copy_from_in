mod error;
mod cli;
mod packets_handler;
pub use error::Error;
use packets_handler::start_packets_handler;
pub use utilites;
mod copyer;
mod state;
mod services;
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
    db::initialize_db().await;
    let _ = services::start_http_server(params.http_port, Arc::clone(&APP_STATE)).await;
    services::start_ws_server(params.ws_port, Arc::clone(&APP_STATE)).await;
    start_packets_handler().await;
    loop
    {
        let settings = Arc::clone(&APP_STATE);
        let _ = DirectoriesSpy::process_tasks(settings).await;
        tokio::time::sleep(tokio::time::Duration::from_secs(15)).await;
        //for testing
    }
}