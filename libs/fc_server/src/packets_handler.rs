use std::sync::Arc;

use db::DbInterface;
use logger::backtrace;
use transport::Packet;

use crate::{copyer::DirectoriesSpy, WebsocketServer};

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
            //TODO если пакет не парсится то пакетинфо отсутсвует! надо это переделать! добавить минимальную инфу о пакете хоть о пустом
            //точнее она там и так есть, надо просто ее протащить наверх при ошибке тоже добавлять в БД
            let _add_to_base = r.get_packet_info().as_ref().unwrap().add_or_ignore();
            WebsocketServer::new_packet_event(r).await;
        }
    });
}
