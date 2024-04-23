use logger::debug;
use service::Client;
use transport::Contract;

use crate::emits::TauriEmits;

pub struct WebsocketClient;
impl Client<Contract> for WebsocketClient{}
pub async fn start_ws_service(addr: String)
{   
    debug!("стартуем получение сообщений от сервера");
    WebsocketClient::start_client(&addr, |msg|
    {
        debug!("Получено сообщение от сервера {:?}", msg);
        let _ = match msg
        {
            Contract::NewPacket(p) => TauriEmits::packets_update(p),
            Contract::Error(e) => TauriEmits::error(e),
            Contract::ErrorConversion(e) => TauriEmits::error(e),
            Contract::TaskUpdated(t) => TauriEmits::task_updated(t),
            Contract::TaskDeleted(t) => TauriEmits::task_deleted(t),
        };
    }).await;
}