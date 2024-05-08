use std::sync::Arc;

use db::{Operations, PacketsTable};
use logger::backtrace;
use transport::Packet;

use crate::{copyer::DirectoriesSpy, services::WebsocketServer};

///стартуем обработчик новых поступивших пакетов
/// одна из функций отправка этих пакетов всем подключенным клиентам через сервер websocket
pub async fn start_packets_handler()
{
    let receiver: crate::async_channel::Receiver<Packet> = DirectoriesSpy::subscribe_new_packet_event().await;
    //получаем сообщения от копировальщика
    tokio::spawn(async move
    {
        let receiver = Arc::new(receiver);
        while let Ok(r) = receiver.recv().await
        {
            logger::debug!("Сервером отправлен новый пакет {:?}, {}", &r, backtrace!());
            //точнее она там и так есть, надо просто ее протащить наверх при ошибке тоже добавлять в БД
            //TODO нужно сделать обычный id и добавить имя таска которым пакет был обработан
            let p_table = PacketsTable::new(&r);
            let test = p_table.add_or_replace().await;
            if test.is_err()
            {
                logger::error!("{}", test.err().unwrap().to_string());
            }
            WebsocketServer::new_packet_event(r).await;
        }
    });
}
