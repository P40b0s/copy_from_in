use std::sync::Arc;
use db_service::SqlOperations;
use http_body_util::BodyExt;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{body::Incoming,  Method, Request, Response, StatusCode};
use hyper_util::rt::TokioIo;
use logger::{debug, error, info};
use serde_json::Value;
use settings::Task;
use tokio::net::TcpListener;
use anyhow::Result;
use transport::{BytesSerializer, Packet, Pagination};
use utilites::http::{empty_response, error_response, json_response, ok_response, BoxBody};
use crate::db::PacketTable;
use crate::state::AppState;
use super::WebsocketServer;

impl From<crate::Error> for Result<Response<BoxBody>, crate::Error>
{
    fn from(value: crate::Error) -> Self 
    {
        Ok(error_response(value.to_string(), StatusCode::BAD_REQUEST))
    }
}

//type BoxBody = http_body_util::combinators::BoxBody<Bytes, hyper::Error>;
static NOTFOUND: &[u8] = b"this endpoint not found";

pub async fn start_http_server(port: usize, app_state: Arc<AppState>) -> Result<()>
{
    let addr = ["0.0.0.0:".to_owned(), port.to_string()].concat();
    let listener = TcpListener::bind(&addr).await?;
    debug!("api доступно по http://{}", addr);
    tokio::spawn(async move
    {
        loop 
        {
            let connected = listener.accept().await;
            let app_state = Arc::clone(&app_state);
            if let Ok((stream, addr)) = connected
            {
                let io = TokioIo::new(stream);
                tokio::task::spawn(async move 
                {
                    let service = service_fn(move |req|
                    {
                        info!("Поступил запрос от {} headers->{:?}", &addr, req.headers());
                        router(req, Arc::clone(&app_state))
                    });
                    if let Err(err) = http1::Builder::new().serve_connection(io, service).await 
                    {
                        error!("Ошибка обслуживания соединения: {:?}", err);
                    }
                });
            }
            else 
            {
                error!("Ошибка подключения клиента к api {}", connected.err().unwrap().to_string());
            }
        }
    });
    Ok(())
}

async fn router(req: Request<Incoming>, app_state: Arc<AppState>) -> Result<Response<BoxBody>, crate::Error> 
{
    let resp = match (req.method(), req.uri().path()) 
    {
        (&Method::GET, "/api/v1/settings/tasks") => get_tasks(app_state).await,
        (&Method::PUT, "/api/v1/settings/tasks/update") => update_task(req, app_state).await,
        (&Method::DELETE, "/api/v1/settings/tasks/delete") => delete_task(req, app_state).await,
        (&Method::GET, "/api/v1/packets/truncate") => truncate(app_state).await,
        (&Method::GET, "/api/v1/packets/clean") => clean(app_state).await,
        (&Method::POST, "/api/v1/packets/rescan") => rescan(req, app_state).await,
        (&Method::POST, "/api/v1/packets/delete") => delete(req, app_state).await,
        (&Method::GET, "/api/v1/packets") => get_packets(req, app_state).await,
        (&Method::GET, "/api/v1/packets/search") => search_packets(req, app_state).await,
        (&Method::GET, "/api/v1/packets/count") => get_packets_count(app_state).await,
        _ => 
        {
            let err = ["Эндпоинт ", req.uri().path(), " отсутсвует в схеме API"].concat();
            logger::warn!("{}", &err);
            Ok(utilites::http::error_response(err, StatusCode::NOT_FOUND))
        }
    };
    if resp.is_err()
    {
        error!("{}", resp.as_ref().err().unwrap());
        Ok(utilites::http::error_response(resp.err().unwrap().to_string(), StatusCode::BAD_REQUEST))
    }
    else
    {
        resp
    }
}
/// /settings/tasks
async fn get_tasks(app_state: Arc<AppState>) -> Result<Response<BoxBody>, crate::Error> 
{
    let settings = super::settings::get(app_state).await;
    if settings.is_err()
    {
        let err = settings.err().unwrap();
        logger::error!("{}", &err);
        return err.into();
    }
    let settings = settings.unwrap();
    Ok(json_response(&settings))
}

async fn get_packets_count(app_state: Arc<AppState>) -> Result<Response<BoxBody>, crate::Error> 
{
    let guard = app_state.settings.lock().await;
    let names = guard.get_visible_tasks_names();
    drop(guard);
    let count = PacketTable::packets_count(app_state.get_db_pool(), names).await?;
    Ok(ok_response(count.to_string()))
}

/// get "/packets/list"  
/// get "/packets/list?limit=20&offset=200"
async fn get_packets(req: Request<Incoming>, app_state: Arc<AppState>) -> Result<Response<BoxBody>, crate::Error> 
{
    let data = if let Some(q) = utilites::http::get_query(req.uri())
    {
        let limit = q.get("limit");
        let offset = q.get("offset");
        //выбираем только те таски у котрых есть флаг visible=true
        let guard = app_state.settings.lock().await;
        let names = guard.get_visible_tasks_names();
        drop(guard);
        if limit.is_some() && offset.is_some()
        {
            let paging = Pagination 
            {
                row: limit.unwrap().parse().unwrap(),
                offset: offset.unwrap().parse().unwrap()
            };
            PacketTable::get_with_offset(paging.row, paging.offset, app_state.get_db_pool(), names).await
        }
        else 
        {
            return Ok(error_response("В запросе должны присутсвовать параметры limit и offset".to_owned(), StatusCode::BAD_REQUEST));
        }
    }
    else 
    {
        PacketTable::select_all(app_state.get_db_pool()).await
    };
    if let Err(e) = data
    {
        logger::error!("{}", &e);
        return Ok(error_response(e.to_string(), StatusCode::BAD_REQUEST));
    }
    let table_data = data.unwrap();
    let guard = app_state.settings.lock().await;
    let tasks = guard.tasks.clone();
    drop(guard);
    let mut complex_data: Vec<Packet> = Vec::with_capacity(table_data.capacity());
    for d in table_data
    {
        if let Some(task) = tasks.iter().find(|f| f.get_task_name() == d.get_task_name())
        {
            complex_data.push(Packet::new_from_db(task.clone(), d.get_id(), d.get_packet_info(), d.report_is_sended()));
        }
        else
        {
            logger::error!("Задачи {} не существует в текущих настройках программы! Невозможно связять запись БД {}", d.get_task_name(), d.get_id());
        }
    }
    Ok(json_response(&complex_data))
}

async fn search_packets(req: Request<Incoming>, app_state: Arc<AppState>) -> Result<Response<BoxBody>, crate::Error> 
{
    let body = req.collect().await?.to_bytes();
    let value = serde_json::from_slice::<Value>(&body);
    if value.is_err()
    {
        return Ok(error_response("В запросе должен присутсвовать параметр value".to_owned(), StatusCode::BAD_REQUEST));
    }
    let value = value.unwrap();
    let data =
    {
        let search_string = value["value"].as_str();
        if let Some(s) = search_string
        {
            PacketTable::search(s, app_state.get_db_pool()).await
        }
        else 
        {
            return Ok(error_response("В запросе должен присутсвовать параметр value".to_owned(), StatusCode::BAD_REQUEST));
        }
    };
    if let Err(e) = data
    {
        logger::error!("{}", &e);
        return Ok(error_response(e.to_string(), StatusCode::BAD_REQUEST));
    }
    let guard = app_state.settings.lock().await;
    let tasks = guard.tasks.clone();
    drop(guard);
    let data = data.unwrap();
    let mut complex_data: Vec<Packet> = Vec::with_capacity(data.capacity());
    for d in data
    {
        if let Some(task) = tasks.iter().find(|f| f.get_task_name() == d.get_task_name())
        {
            complex_data.push(Packet::new_from_db(task.clone(), d.get_id(), d.get_packet_info(), d.report_is_sended()));
        }
        else
        {
            logger::error!("Задачи {} не существует в текущих настройках программы! Невозможно связять запись БД {}", d.get_task_name(), d.get_id());
        }
    }
    Ok(json_response(&complex_data))
}


/// put "/settings/tasks"
/// в обратку сообщаем всем клиентам через websocket
async fn update_task(req: Request<Incoming>, app_state: Arc<AppState>) -> Result<Response<BoxBody>, crate::Error> 
{
    let body = req.collect().await?.to_bytes();
    let task: Task = Task::from_bytes(&body)?;
    let _ = super::settings::update(task.clone(), app_state).await?;
    let response = ok_response(["Задача ", task.get_task_name(), " обновлена"].concat());
    //сообщаем всем через вебсокет что мы обновили какую то таску
    WebsocketServer::task_update_event(task).await;
    Ok(response)
}
/// delete "/settings/tasks/delete"
/// в обратку сообщаем всем клиентам через websocket
async fn delete_task(req: Request<Incoming>, app_state: Arc<AppState>) -> Result<Response<BoxBody>, crate::Error> 
{
    if let Some(data) = utilites::http::get_query(req.uri())
    {
        if let Some(name) = data.get("name")
        {
            let  _ = super::settings::delete(name, app_state).await?;
            let response = ok_response(["Задача ", name, " удалена"].concat());
            WebsocketServer::task_delete_event(name).await;
            return Ok(response);
        }
        else 
        {
            return Ok(error_response("В запросе должен присутсвовать параметр name".to_owned(), StatusCode::BAD_REQUEST));
        }
    }
    else 
    {
        return Ok(error_response("В запросе должен присутсвовать параметр name".to_owned(), StatusCode::BAD_REQUEST));
    };
}

async fn clean(app_state: Arc<AppState>) -> Result<Response<BoxBody>, crate::Error> 
{
    super::service::clean_packets(app_state).await;
    let response = empty_response(StatusCode::OK);
    Ok(response)
}
async fn truncate(app_state: Arc<AppState>) -> Result<Response<BoxBody>, crate::Error> 
{
    let trunc = super::service::truncate_tasks_excepts(app_state).await?;
    let response = ok_response(trunc.to_string());
    Ok(response)
}

async fn rescan(req: Request<Incoming>, app_state: Arc<AppState>) -> Result<Response<BoxBody>, crate::Error> 
{
    let body = req.collect().await?.to_bytes();
    let packet = Packet::from_bytes(&body)?;
    let _ = super::service::rescan_packet(packet, app_state).await?;
    let response = empty_response(StatusCode::OK);
    Ok(response)
}

async fn delete(req: Request<Incoming>, app_state: Arc<AppState>) -> Result<Response<BoxBody>, crate::Error> 
{
    let body = req.collect().await?.to_bytes();
    let packet = Packet::from_bytes(&body)?;
    let _ = super::service::delete_packet(packet, app_state).await?;
    let response = empty_response(StatusCode::OK);
    Ok(response)
}

#[cfg(test)]
mod tests
{
    
}