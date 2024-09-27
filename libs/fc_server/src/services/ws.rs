use std::{net::SocketAddr, sync::Arc};
use logger::{backtrace, debug, error};
use settings::Task;
use transport::{Contract, Packet};
use service::Server;
use crate::{copyer::DirectoriesSpy, state::AppState, Error};

pub struct WebsocketServer;
impl Server<Contract> for WebsocketServer{}
impl WebsocketServer
{
    pub async fn new_packet_event(packet: Packet)
    {
        Self::broadcast_message_to_all(Contract::NewPacket(packet)).await;  
    }
    pub async fn task_update_event(task: Task)
    {
        Self::broadcast_message_to_all(Contract::TaskUpdated(task)).await;  
    }
    pub async fn task_delete_event(task_name: &str)
    {
        Self::broadcast_message_to_all(Contract::TaskDeleted(task_name.to_owned())).await;  
    }
    pub async fn send_error_msg(addr: &SocketAddr, error: Error)
    {
        Self::send(Contract::Error(error.to_string()), &addr).await;
    }
}

///Стартуем сервер вебсокет для приема и отправки сообщений
pub async fn start_ws_server(port: usize, app_state: Arc<AppState>)
{
    let addr = ["0.0.0.0:".to_owned(), port.to_string()].concat();
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

