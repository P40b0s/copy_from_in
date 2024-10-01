mod error;
mod cli;
mod packets_handler;
pub use error::Error;
use packets_handler::start_packets_handler;
pub use utilites;
mod copyer;
mod state;
mod services;
mod db;
use std::sync::Arc;
use copyer::DirectoriesSpy;
use logger::{debug, StructLogger};
use state::AppState;
extern crate async_channel;


#[tokio::main(flavor = "multi_thread", worker_threads = 2)]
async fn main() -> Result<(), Error>
{
    StructLogger::new_default();
    let params = cli::Cli::parse_args();
    debug!("Инициализация настроек");
    let app_state = Arc::new(AppState::initialize().await?);
    debug!("Инициализация базы данных");
    db::initialize_db(app_state.get_db_pool()).await;
    let _ = services::start_http_server(params.http_port, Arc::clone(&app_state)).await;
    services::start_ws_server(params.ws_port, Arc::clone(&app_state)).await;
    start_packets_handler(app_state.get_db_pool()).await;
    loop
    {
        let settings = Arc::clone(&app_state);
        let _ = DirectoriesSpy::process_tasks(settings).await;
        tokio::time::sleep(tokio::time::Duration::from_secs(15)).await;
        //for testing
    }
}