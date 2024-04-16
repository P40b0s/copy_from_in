use std::sync::Arc;

use logger::{debug, error};
use once_cell::sync::OnceCell;
use settings::Task;
use tauri::{AppHandle, Manager};
use service::Client;
use transport::Contract;

use crate::commands;


static TAURI : OnceCell<Arc<AppHandle>> = OnceCell::new();
pub async fn start_ws_service(addr: String, handle: Arc<AppHandle>)
{   
    let _ = TAURI.set(handle);
    Client::<Contract>::start_client(&addr, on_receive).await;
    debug!("стартуем получение сообщений от сервера");
}

fn on_receive(msg: Contract)
{

    debug!("Получено сообщение от сервера {:?}", msg);
    let _ = match msg
    {
        Contract::NewPacket(p) => TAURI.get().unwrap().app_handle().emit_all("packets_update", p),
        Contract::Error(e) => TAURI.get().unwrap().app_handle().emit_all("error", e),
        Contract::ErrorConversion(e) => TAURI.get().unwrap().app_handle().emit_all("error", e),
        Contract::TaskUpdated(t) => TAURI.get().unwrap().app_handle().emit_all("task_updated", t),
        Contract::TaskDeleted(t) => TAURI.get().unwrap().app_handle().emit_all("packets_delete", t),
    };
    // match msg.command.get_target()
    // {
    //     "settings/tasks" => settings_operations(&msg.command),
    //     _=>()
    // }
    
}