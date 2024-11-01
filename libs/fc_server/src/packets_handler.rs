use std::sync::Arc;
use super::db::PacketTable;
use db_service::{SqlOperations, SqlitePool};
use transport::Packet;
use crate::{copyer::DirectoriesSpy, services::WebsocketServer};

///стартуем обработчик новых поступивших пакетов
/// одна из функций отправка этих пакетов всем подключенным клиентам через сервер websocket
pub async fn start_packets_handler(pool: Arc<SqlitePool>)
{
    let receiver: crate::async_channel::Receiver<Packet> = DirectoriesSpy::subscribe_new_packet_event().await;
    tokio::spawn(async move
    {
        let receiver = Arc::new(receiver);
        while let Ok(r) = receiver.recv().await
        {
            let pool = Arc::clone(&pool);
            let p_table = PacketTable::new(&r);
            
            let test = p_table.add_or_replace(pool).await;
            if test.is_err()
            {
                logger::error!("{}", test.err().unwrap().to_string());
            }
            WebsocketServer::new_packet_event(r).await;
        }
    });
}
