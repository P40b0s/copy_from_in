mod error;
mod messages_processor;
pub use error::Error;
mod helpers;
mod copyer;
mod state;
mod commands;
use std::{default, future, sync::Arc};
use anyhow::Result;
use copyer::{DirectoriesSpy, NewPacketInfo};
use logger::{debug, StructLogger};
use state::AppState;
use once_cell::sync::Lazy;
use tokio::sync::OnceCell;
use crossbeam_channel::{Receiver, bounded};
use websocket_service::{Server, WebsocketMessage};

static APP_STATE : Lazy<Arc<AppState>> = Lazy::new(|| Arc::new(AppState::default()));

#[tokio::main]
async fn main()
{
    StructLogger::initialize_logger();
    Server::start_server("127.0.0.1:3010").await;
    let receiver = DirectoriesSpy::subscribe_new_packet_event().await;
    Server::on_receive_msg(|addr, msg|
    {
        debug!("Сервером полчено сообщение от {} через канал {}", addr, &msg.command.target);
        async 
        {
            messages_processor::process_messages(addr, msg, Arc::clone(&APP_STATE)).await;
        };
    }).await;
    //в цикле получаем сообщение от копировальщика
    tokio::spawn(async move
    {
        let receiver = Arc::new(receiver);
        loop 
        {
            if let Ok(r) = receiver.recv()
            {
                debug!("main получено сообщение о парсинге нового пакета");
                let wsmsg = WebsocketMessage::new_with_flex_serialize("packet", "new", Some(&r));
                Server::broadcast_message_to_all(&wsmsg).await;
            }
        }
    });
    loop
    {
        let settings = Arc::clone(&APP_STATE);
        let _ = DirectoriesSpy::process_tasks(settings).await;
        tokio::time::sleep(tokio::time::Duration::from_secs(15)).await;
    }
}