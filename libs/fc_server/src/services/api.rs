use std::net::SocketAddr;
use std::sync::Arc;
use bytes::Bytes;
use http_body_util::{BodyExt, Full};
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{body::Incoming as IncomingBody, header, Method, Request, Response, StatusCode};
use hyper_util::rt::TokioIo;
use logger::{debug, error, info};
use serde::Serialize;
use settings::Task;
use tokio::net::TcpListener;
use anyhow::Result;
use transport::{BytesSerializer, Packet};
use crate::state::AppState;
use super::WebsocketServer;

type BoxBody = http_body_util::combinators::BoxBody<Bytes, hyper::Error>;
static NOTFOUND: &[u8] = b"this endpoint not found";

pub async fn start_http_server(port: usize, app_state: Arc<AppState>) -> Result<()>
{
    let addr = ["0.0.0.0:".to_owned(), port.to_string()].concat();
    
        let addr: SocketAddr = addr.parse().unwrap();
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
                            response_examples(req, Arc::clone(&app_state))
                        });
                        if let Err(err) = http1::Builder::new().serve_connection(io, service).await 
                        //еще настройки из https://docs.rs/hyper/latest/hyper/server/conn/http1/struct.Builder.html
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

async fn response_examples(req: Request<IncomingBody>, app_state: Arc<AppState>) -> Result<Response<BoxBody>> 
{
    match (req.method(), req.uri().path()) 
    {
        (&Method::GET, "/settings/tasks") => get_tasks(app_state).await,
        (&Method::POST, "/settings/tasks/update") => update_task(req, app_state).await,
        (&Method::POST, "/settings/tasks/delete") => delete_task(req, app_state).await,
        (&Method::GET, "/packets/truncate") => truncate(app_state).await,
        (&Method::GET, "/packets/clean") => clean(app_state).await,
        (&Method::POST, "/packets/rescan") => rescan(req, app_state).await,
        //(&Method::GET, "/packets/list") => get_packets_list(app_state).await,
        _ => 
        {
            let err = format!("В Апи отсуствует эндпоинт {}", req.uri().path());
            logger::warn!("{}", &err);
            // Return 404 not found response.
            Ok(Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(full(err))
                .unwrap())
        }
    }
}
async fn get_tasks(app_state: Arc<AppState>) -> Result<Response<BoxBody>> 
{
    let settings = super::settings::get(app_state).await?;
    let bytes = settings.to_bytes()?;
    let body_data = to_body(bytes);
    let response = Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "application/octet-stream")
        .body(body_data)?;
    Ok(response)
}

//что то тут непонятное передается
// async fn get_packets_list(app_state: Arc<AppState>) -> Result<Response<BoxBody>> 
// {
//     let log = super::settings::get_log(app_state).await?;
//     logger::debug!("{:?}", log);
//     let bytes = log.to_bytes()?;
//     let body_data = to_body(bytes);
//     let response = Response::builder()
//         .status(StatusCode::OK)
//         .header(header::CONTENT_TYPE, "application/octet-stream")
//         .body(body_data)?;
//     Ok(response)
// }

async fn update_task(req: Request<IncomingBody>, app_state: Arc<AppState>) -> Result<Response<BoxBody>> 
{
    let body = req.collect().await?.to_bytes();
    let task: Task = Task::from_bytes(&body)?;
    if let Err(e) = super::settings::update(task.clone(), app_state).await
    {
        return error_responce(e);
    }
    let response = response_ok_empty()?;
    //сообщаем всем через вебсокет что мы обновили какую то таску
    WebsocketServer::task_update_event(task).await;
    Ok(response)
}

async fn delete_task(req: Request<IncomingBody>, app_state: Arc<AppState>) -> Result<Response<BoxBody>> 
{
    let body = req.collect().await?.to_bytes();
    let task: Task = Task::from_bytes(&body)?;
    if let Err(e) = super::settings::delete(task.clone(), app_state).await
    {
        return error_responce(e);
    }
    let response = response_ok_empty()?;
    WebsocketServer::task_delete_event(task).await;
    Ok(response)
}

async fn clean(app_state: Arc<AppState>) -> Result<Response<BoxBody>> 
{
    let cl = super::service::clear_dirs(app_state).await;
    if let Err(e) = cl
    {
        return error_responce(e);
    }
    let response = response_ok(cl.unwrap())?;
    Ok(response)
}
async fn truncate(app_state: Arc<AppState>) -> Result<Response<BoxBody>> 
{
    let trunc = super::service::truncate_tasks_excepts(app_state).await;
    if let Err(e) = trunc
    {
        return error_responce(e);
    }
    let response : Response<BoxBody> = response_ok(trunc.unwrap())?;
    Ok(response)
}

async fn rescan(req: Request<IncomingBody>, app_state: Arc<AppState>) -> Result<Response<BoxBody>> 
{
    let body = req.collect().await?.to_bytes();
    let packet = Packet::from_bytes(&body)?;
    if let Err(e) = super::service::rescan_packet(packet, app_state).await
    {
        return error_responce(e);
    }
    let response = response_ok_empty()?;
    Ok(response)
}

fn full<T: Into<Bytes>>(chunk: T) -> BoxBody 
{
    Full::new(chunk.into())
        .map_err(|never| match never {})
        .boxed()
}

fn response_ok<T : BytesSerializer + Serialize>(obj: T) -> Result<Response<BoxBody>>
{
    
    let bytes = obj.to_bytes()?;
    let body_data = to_body(bytes);
    let resp = Response::builder()
    .status(StatusCode::OK)
    .header(header::CONTENT_TYPE, "application/octet-stream")
    .body(body_data)?;
    Ok(resp)
}


fn response_ok_empty() -> Result<Response<BoxBody>>
{
    let resp =  Response::builder()
    .status(StatusCode::OK)
    .header(header::CONTENT_TYPE, "application/json")
    .body(empty_body())?;
    Ok(resp)
}

pub fn to_body(bytes: Bytes) -> BoxBody
{
    Full::new(bytes)
        .map_err(|never| match never {})
        .boxed()
}

fn empty_body() -> BoxBody
{
    to_body(Bytes::new())
}

fn error_responce(error: crate::Error) -> Result<Response<BoxBody>>
{
    let req = Response::builder()
        .status(StatusCode::UNAVAILABLE_FOR_LEGAL_REASONS)
        .header(header::CONTENT_TYPE, "text/html; charset=utf-8")
        .body(full(error.to_string()))?;
    Ok(req)
}

// pub trait ToBody
// {
//     fn to_body(&self) -> Result<BoxBody> where Self: Serialize
//     {
//         let mut s = flexbuffers::FlexbufferSerializer::new();
//         let _ = self.serialize(&mut s)?;
//         let bytes = Bytes::copy_from_slice(s.view());
//         Ok(Full::new(bytes)
//             .map_err(|never| match never {})
//             .boxed())
//     }
//     fn from_body(body: &Bytes) -> anyhow::Result<Self> where for <'de> Self : Deserialize<'de>
//     {
//         let r = flexbuffers::Reader::get_root(body.as_ref())?;
//         let deserialize = Self::deserialize(r).with_context(|| "Ошибка десериализации".to_owned())?;
//         Ok(deserialize)
//     }
// }
// impl ToBody for Settings{}
// impl ToBody for Task{}

#[cfg(test)]
mod tests
{
    use std::collections::HashMap;
    use url::Url;
    #[test]
    fn test_uri()
    {
      
        let uri = "http://192.168.0.1/foo/bar?offset=20".parse::<Url>().unwrap();
        assert_eq!(uri.path(), "/foo/bar");
        assert_eq!(uri.query(), Some("offset=20"));
        assert!(uri.host().is_some());
        let params: HashMap<String, String> = uri
        .query()
        .map(|v| {
            url::form_urlencoded::parse(v.as_bytes())
                .into_owned()
                .collect()
        })
    .unwrap_or_else(HashMap::new);
    assert_eq!(params.get("offset"), Some(&"20".to_owned()));
    }
}