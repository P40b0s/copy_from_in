use logger::debug;
use once_cell::sync::Lazy;
use service::Client;
use transport::Contract;
use utilites::Date;
use uuid::{NoContext, Timestamp, Uuid};
//use crate::emits::TauriEmits;


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