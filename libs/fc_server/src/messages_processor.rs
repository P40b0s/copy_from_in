use std::{net::SocketAddr, sync::Arc};
use settings::Task;
use websocket_service::{Command, Server, WebsocketMessage};

use crate::{commands, copyer, state::AppState, Error};


pub async fn process_messages(addr: SocketAddr, msg: websocket_service::WebsocketMessage, state: Arc<AppState>)
{
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
                if let Err(e) = commands::update(task, state).await
                {
                    send_error_msg(addr, "settings", e);
                }
            }
        }
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
    }
}

async fn send_error_msg(addr: &SocketAddr, target: &str, error: Error)
{
    let msg = WebsocketMessage::new_with_flex_serialize(target, "error", Some(&error.to_string()));
    Server::send(&msg, &addr).await;
}