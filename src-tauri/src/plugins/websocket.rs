use std::sync::Arc;

use logger::debug;
use service::Client;
use settings::Task;
use tauri::plugin::{Builder, TauriPlugin};
use tauri::{App, AppHandle, Emitter, Manager, Runtime, State};
use transport::{Contract, Packet, Senders};
use crate::http_service;
use crate::state::AppState;
use crate::ws_serivice::WebsocketClient;
use crate::Error;




pub fn packets_update<R: Runtime>(handle: Arc<AppHandle<R>>, packet: Packet)
{
    let _ = handle.emit("packets_update", packet);
}
pub fn error<R: Runtime>(handle: Arc<AppHandle<R>>, error: String)
{
    let _ = handle.emit("error", error);
}
pub fn task_updated<R: Runtime>(handle: Arc<AppHandle<R>>, task: Task)
{
    let _ = handle.emit("task_updated", task);
}
pub fn task_deleted<R: Runtime>(handle: Arc<AppHandle<R>>, task_name: String)
{
    let _ = handle.emit("task_deleted", task_name);
}
pub fn clean_start<R: Runtime>(handle: Arc<AppHandle<R>>)
{
    let _ = handle.emit("clean_start", ());
}
pub fn clean_complete<R: Runtime>(handle: Arc<AppHandle<R>>, count: u32)
{
    let _ = handle.emit("clean_complete", count);
}
pub fn need_packets_refresh<R: Runtime>(handle: Arc<AppHandle<R>>)
{
    let _ = handle.emit("need_packets_refresh", ());
}
pub fn sender_update<R: Runtime>(handle: Arc<AppHandle<R>>, sender: Senders)
{
    let _ = handle.emit("sender_update", sender);
}



pub struct WebsocketPlugin
{
    addresse: String
}
impl WebsocketPlugin
{
    pub fn new(addr: String) -> Self
    {
        Self
        {
            addresse: addr
        }
    }
}
impl super::PluginContext for WebsocketPlugin
{
    const NAME: &str = "websocket";
    fn build<R: Runtime>(&self, app_state: Arc<AppState>) -> TauriPlugin<R> 
    {
        let addr = self.addresse.clone();
        Builder::new(Self::NAME)
        .setup(|app_handle, p| 
        {
            let handle = Arc::new(app_handle.to_owned());
            tauri::async_runtime::spawn(async move 
            {
                debug!("стартуем получение сообщений от сервера");
                WebsocketClient::start_client(&addr, move |msg|
                {
                    let handle = Arc::clone(&handle);
                    debug!("Получено сообщение от сервера {:?}", msg);
                    let _ = match msg
                    {
                        Contract::NewPacket(p) => packets_update(handle, p),
                        Contract::Error(e) => error(handle, e),
                        Contract::ErrorConversion(e) => error(handle, e),
                        Contract::TaskUpdated(t) => task_updated(handle,t),
                        Contract::TaskDeleted(t) => task_deleted(handle,t),
                        Contract::CleanStart => clean_start(handle),
                        Contract::CleanComplete(c) => clean_complete(handle,c),
                        Contract::NeedPacketsrefresh => need_packets_refresh(handle,),
                        Contract::SenderUpdate(s) => sender_update(handle,s),
                    };
                }).await;
            });
            app_handle.manage(app_state);
            Ok(())
        })
        .build()
    }
}
