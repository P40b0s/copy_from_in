use std::{net::SocketAddr, sync::Arc};
use logger::{backtrace, debug, error};
use settings::Task;
use transport::{Contract, NewPacketInfo};
use service::Server;
use crate::{copyer::DirectoriesSpy, Error};

pub struct WebsocketServer;
impl Server<Contract> for WebsocketServer{}
impl WebsocketServer
{
    pub async fn new_packet_event(packet: NewPacketInfo)
    {
        Self::broadcast_message_to_all(Contract::NewPacket(packet)).await;  
    }
    pub async fn task_update_event(task: Task)
    {
        Self::broadcast_message_to_all(Contract::TaskUpdated(task)).await;  
    }
    pub async fn task_delete_event(task: Task)
    {
        Self::broadcast_message_to_all(Contract::TaskDeleted(task)).await;  
    }
    pub async fn send_error_msg(addr: &SocketAddr, error: Error)
    {
        Self::send(Contract::Error(error.to_string()), &addr).await;
    }
}

///Стартуем сервер вебсокет для приема и отправки сообщений
pub async fn start_ws_server(port: usize)
{
    let addr = ["127.0.0.1:".to_owned(), port.to_string()].concat();
    WebsocketServer::start_server(&addr, |addr, msg|
    {
        async move
        {
            debug!("Серверу поступило сообщение {:?} от {}", &msg, &addr);
            match &msg
            {
                Contract::Error(e) => error!("{}", e),
                Contract::ErrorConversion(e) => error!("{}", e),
                //Contract::TaskUpdated(t) => task_updated(&addr, t).await,
                //Contract::TaskDeleted(t) => task_deleted(&addr, t).await,
                //остальные сообщения нем на сервере обрабатывать ненужно
                _ => ()
            }
        }
    }).await;
}
///стартуем обработчик новых поступивших пакетов
/// одна из функций отправка этих пакетов всем подключенным клиентам через сервер websocket
pub async fn start_new_packets_handler()
{
    let receiver: crate::async_channel::Receiver<transport::NewPacketInfo> = DirectoriesSpy::subscribe_new_packet_event().await;
    //получаем сообщения от копировальщика
    tokio::spawn(async move
    {
        let receiver = Arc::new(receiver);
        while let Ok(r) = receiver.recv().await
        {
            logger::debug!("Сервером отправлен новый пакет {:?}, {}", &r, backtrace!());
            WebsocketServer::new_packet_event(r).await;
        }
    });
}

// async fn task_updated(addr: & SocketAddr, task: &Task)
// {
//     let state = Arc::clone(&APP_STATE);
//     debug!("Попытка обновить задачу {:?}", task);
//     if let Err(e) = commands::settings::update(task.clone(), state).await
//     {
//         error!("Ошибка обновления задачи {:?}", &e.to_string());
//         send_error_msg(addr, e).await;
//     }
//     else
//     {
//         info!("Задача {} успешно обновлена", task.get_task_name());
//         WebsocketServer::broadcast_message_to_all(Contract::new_task_updated(task)).await;
//     }
// }
// async fn task_deleted(addr: & SocketAddr, task: &Task)
// {
//     let state = Arc::clone(&APP_STATE);
//     debug!("Попытка удалить задачу {:?}", task);
//     if let Err(e) = commands::settings::delete(task.clone(), state).await
//     {
//         error!("Ошибка удаления задачи {:?}", &e.to_string());
//         send_error_msg(addr, e).await;
//     }
//     else
//     {
//         info!("Задача {} успешно удалена", task.get_task_name());
//         WebsocketServer::broadcast_message_to_all(Contract::new_task_deleted(task)).await;
//     }
// }

