use std::{net::SocketAddr, sync::Arc};
use logger::debug;
use settings::Task;
use websocket_service::{Command, Server, WebsocketMessage};

use crate::{commands, copyer::{self, DirectoriesSpy}, state::AppState, Error, APP_STATE};

///Стартуем сервер вебсокет для приема и отправки сообщений
pub async fn start_ws_server(port: usize)
{
    let addr = ["127.0.0.1:".to_owned(), port.to_string()].concat();
    Server::start_server(&addr).await;
    Server::on_receive_message(on_receive_message).await;
}
///стартуем обработчик новых поступивших пакетов
/// одна из функций отправка этих пакетов всем подключенным клиентам через сервер websocket
pub async fn start_new_packets_handler()
{
    let receiver = DirectoriesSpy::subscribe_new_packet_event().await;
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
}


pub async fn on_receive_message(addr: SocketAddr, msg: websocket_service::WebsocketMessage)
{
    let state = Arc::clone(&APP_STATE);
    if msg.success
    {
        match msg.command.get_target()
        {
            "settings" => 
            {
                settings_worker(&addr, &msg.command, state).await;
            },
            "event" => 
            {
                event_worker(&addr, &msg.command, state).await;
            },
            _ => {}
        }
    }
}

async fn settings_worker(addr: &SocketAddr, cmd: &Command, state: Arc<AppState>)
{
    match cmd.get_method()
    {
        "update" => 
        {
            if let Ok(task) = cmd.extract_payload::<Task>()
            {
                if let Err(e) = commands::settings::update(task, state).await
                {
                    send_error_msg(addr, "settings", e);
                }
            }
        }
        _=>()
    }
}

async fn event_worker(addr: &SocketAddr, cmd: &Command, state: Arc<AppState>)
{
    match cmd.get_method()
    {
        "on_client_connected" => 
        {
            let settings = state.get_settings().await;
            let ws_settings = WebsocketMessage::new_with_flex_serialize("settings", "reload", Some(&settings));
            Server::send(&ws_settings, &addr).await;
            let packets = copyer::get_full_log().await;
            let ws_logs = WebsocketMessage::new_with_flex_serialize("packets", "reload", Some(&packets));
            Server::send(&ws_logs, &addr).await;
        }
        _ => ()
    }
}

async fn send_error_msg(addr: &SocketAddr, target: &str, error: Error)
{
    let msg = WebsocketMessage::new_with_flex_serialize(target, "error", Some(&error.to_string()));
    Server::send(&msg, &addr).await;
}