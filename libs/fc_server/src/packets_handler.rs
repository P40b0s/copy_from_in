use std::sync::Arc;
use super::db::PacketTable;
use db_service::{SqlOperations, SqlitePool};
use transport::Packet;
use crate::{copyer::DirectoriesSpy, db::AddresseTable, services::WebsocketServer};

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
            let p_table = PacketTable::new(&r);
            let test = p_table.add_or_replace(Arc::clone(&pool)).await;
            if test.is_err()
            {
                logger::error!("{}", test.err().unwrap().to_string());
            }
            if let Ok(addreesses) = AddresseTable::try_from(r.get_packet_info())
            {
                //TODO СДЕЛАТЬ чтобы add_or_ignore возвращал количество 
                let _ = addreesses.add_or_ignore(Arc::clone(&pool)).await;
            }
            WebsocketServer::new_packet_event(r).await;
        }
    });
}
