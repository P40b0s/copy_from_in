use std::sync::Arc;

use logger::{debug, error};
use once_cell::sync::OnceCell;
use settings::Task;
use tauri::{AppHandle, Manager};
use websocket_service::{Client, Command};


static TAURI : OnceCell<Arc<AppHandle>> = OnceCell::new();
pub async fn start_ws_service(addr: String, handle: Arc<AppHandle>)
{   
    let _ = TAURI.set(handle);
    Client::start_client(&addr, on_receive).await;
    debug!("стартуем получение сообщений от сервера");
    // loop 
    // {
    //     tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;
    // }
}

pub async fn start_ws_service2(addr: String)
{   
    Client::start_client(&addr, on_receive).await;
    debug!("стартуем получение сообщений от сервера");
}


fn on_receive(msg: websocket_service::WebsocketMessage)
{

    debug!("Получено сообщение от сервера {:?}", msg);
    // match msg.command.get_target()
    // {
    //     "settings/tasks" => settings_operations(&msg.command),
    //     _=>()
    // }
    
}

fn settings_operations(cmd: &Command)
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