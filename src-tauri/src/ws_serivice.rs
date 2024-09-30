// use logger::debug;
// use service::Client;
// use transport::Contract;
// use uuid::{NoContext, Timestamp, Uuid};
// use crate::emits::TauriEmits;


// pub struct WebsocketClient;
// impl Client<Contract> for WebsocketClient
// {
//     fn get_id() -> &'static str 
//     {
//         let ts = Timestamp::from_unix(NoContext, 1497624119, 1234);
//         let uuid = Uuid::new_v7(ts);
//         &uuid.to_string()
//     }
// }
// pub async fn start_ws_service(addr: String)
// {   
//     debug!("стартуем получение сообщений от сервера");
//     WebsocketClient::start_client(&addr, |msg|
//     {
//         debug!("Получено сообщение от сервера {:?}", msg);
//         let _ = match msg
//         {
//             Contract::NewPacket(p) => TauriEmits::packets_update(p),
//             Contract::Error(e) => TauriEmits::error(e),
//             Contract::ErrorConversion(e) => TauriEmits::error(e),
//             Contract::TaskUpdated(t) => TauriEmits::task_updated(t),
//             Contract::TaskDeleted(t) => TauriEmits::task_deleted(t),
//         };
//     }).await;
// }