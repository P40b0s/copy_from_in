use std::sync::Arc;

use logger::error;
use once_cell::sync::OnceCell;
use settings::Task;
use tauri::{AppHandle, Manager};
use websocket_service::{Client, Command};


static TAURI : OnceCell<Arc<AppHandle>> = OnceCell::new();
pub async fn start_ws_service(addr: String, handle: Arc<AppHandle>)
{   
    Client::start_client(&addr).await;
    TAURI.set(handle);
    Client::on_receive_message(on_receive).await;
}


async fn on_receive(msg: websocket_service::WebsocketMessage)
{

    match msg.command.get_target()
    {
        "settings/tasks" => settings_operations(&msg.command).await,
        _=>()
    };
    ()
}

async fn settings_operations(cmd: &Command)
{
    match cmd.get_target()
    {
        "updated" => 
        {
            let payload = cmd.extract_payload::<Task>();
            if let Ok(pl) = payload
            {
                TAURI.get().unwrap().emit_all("settings/tasks/update", pl);
            }
            else
            {
                error!("Ошибка извлечения нагрузки из сообщения ->{}", payload.err().unwrap().to_string());
            }
        },
        _=>()
    };
}