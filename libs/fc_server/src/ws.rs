use std::{net::SocketAddr, sync::Arc};
use logger::{debug, error, info};
use settings::Task;
use service::{Server, WebsocketMessage};
use transport::Contract;
use crate::{commands, copyer::{self, DirectoriesSpy}, state::AppState, Error, APP_STATE};

///Стартуем сервер вебсокет для приема и отправки сообщений
pub async fn start_ws_server(port: usize)
{
    let addr = ["127.0.0.1:".to_owned(), port.to_string()].concat();
    Server::start_server(&addr, on_receive_message).await;
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
            let message = Contract::new_packet(r).as_ws_message();
            Server::broadcast_message_to_all(&message).await;  
        }
    });
}


pub async fn on_receive_message(addr: SocketAddr, msg: service::WebsocketMessage)
{
    debug!("Серверу поступило сообщение {:?} от {}", &msg, &addr);
    let contract: Contract = msg.into();
    match &contract
    {
        Contract::Error(e) => error!("{}", e),
        Contract::ErrorConversion(e) => error!("{}", e),
        Contract::TaskUpdated(t) => task_updated(&addr, t).await,
        Contract::TaskDeleted(t) => task_deleted(&addr, t).await,
        //остальные сообщения нем на сервере обрабатывать ненужно
        _ => ()
    }
    
}

async fn task_updated(addr: & SocketAddr, task: &Task)
{
    let state = Arc::clone(&APP_STATE);
    debug!("Попытка обновить задачу {:?}", task);
    if let Err(e) = commands::settings::update(task.clone(), state).await
    {
        error!("Ошибка обновления задачи {:?}", &e.to_string());
        send_error_msg(addr, e).await;
    }
    else
    {
        let message = Contract::new_task_updated(task).as_ws_message();
        info!("Задача {} успешно обновлена", task.get_task_name());
        Server::broadcast_message_to_all(&message).await;
    }
}
async fn task_deleted(addr: & SocketAddr, task: &Task)
{
    let state = Arc::clone(&APP_STATE);
    debug!("Попытка удалить задачу {:?}", task);
    if let Err(e) = commands::settings::delete(task.clone(), state).await
    {
        error!("Ошибка удаления задачи {:?}", &e.to_string());
        send_error_msg(addr, e).await;
    }
    else
    {
        let message = Contract::new_task_deleted(task).as_ws_message();
        info!("Задача {} успешно удалена", task.get_task_name());
        Server::broadcast_message_to_all(&message).await;
    }
}

async fn send_error_msg(addr: &SocketAddr, error: Error)
{
    let msg = Contract::new_error(error.to_string()).as_ws_message();
    Server::send(&msg, &addr).await;
}