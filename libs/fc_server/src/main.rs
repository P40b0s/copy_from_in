mod error;
mod cli;
mod packets_handler;
pub use error::Error;
use once_cell::sync::OnceCell;
use packets_handler::start_packets_handler;
use tokio::runtime::Handle;
pub use utilites;
mod copyer;
mod state;
mod services;
mod db;
use std::sync::{Arc, RwLock};
use copyer::DirectoriesSpy;
use logger::{debug, StructLogger};
use state::AppState;
extern crate async_channel;
pub static TOKIO_NANDLE: OnceCell<Arc<RwLock<Handle>>> = OnceCell::new();

#[tokio::main(flavor = "multi_thread", worker_threads = 2)]
async fn main() -> Result<(), Error>
{
    let _ = StructLogger::new_default();
    let _ = TOKIO_NANDLE.set(Arc::new(RwLock::new(Handle::current())));
    let params = cli::Cli::parse_args();
    debug!("Инициализация настроек");
    let app_state = Arc::new(AppState::initialize().await?);
    let _ = services::start_http_server(params.http_port, Arc::clone(&app_state)).await;
    services::start_ws_server(params.ws_port, Arc::clone(&app_state)).await;
    start_packets_handler(app_state.get_db_pool()).await;
    let settings = Arc::clone(&app_state);
    let ds = DirectoriesSpy::new(settings, 15000);
    ds.start_tasks().await;

    loop
    {
        //let settings = Arc::clone(&app_state);
        //let _ = DirectoriesSpy::process_tasks(settings).await;
        tokio::time::sleep(tokio::time::Duration::from_secs(200)).await;
        //for testing
    }
}