use logger::debug;
use once_cell::sync::Lazy;
use service::Client;
use transport::Contract;
use utilites::Date;
use uuid::{NoContext, Timestamp, Uuid};
use crate::emits::TauriEmits;


static CLIENT_ID: Lazy<String> = Lazy::new(|| {
    let uuid = uuid::Uuid::new_v7(Timestamp::from_gregorian(Date::now().as_naive_datetime().and_utc().timestamp() as u64, 1234));
    uuid.to_string()
});
pub struct WebsocketClient;
impl Client<Contract> for WebsocketClient
{
    fn get_id() -> &'static str 
    {
        &CLIENT_ID
    }
}
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
            Contract::CleanStart => TauriEmits::clean_start(),
            Contract::CleanComplete(c) => TauriEmits::clean_complete(c),
        };
    }).await;
}